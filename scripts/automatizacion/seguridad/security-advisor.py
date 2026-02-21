#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
SECURITY-ADVISOR - Asesor de Seguridad IA
Analiza el código en busca de vulnerabilidades de seguridad.
Trigger: Análisis automático o manual
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import re
from pathlib import Path

SECURITY_RULES = {
    "python": [
        {
            "pattern": r"os\.system\(",
            "severity": "high",
            "issue": "os.system() permite inyección de comandos",
            "suggestion": "Usa subprocess con lista de argumentos",
        },
        {
            "pattern": r"subprocess\.call\(.*shell=True",
            "severity": "high",
            "issue": "shell=True es vulnerable a inyección",
            "suggestion": "Usa shell=False y pasa argumentos como lista",
        },
        {
            "pattern": r"exec\(",
            "severity": "critical",
            "issue": "exec() permite ejecución dinámica",
            "suggestion": "Evita exec() o sanitiza estrictamente la entrada",
        },
        {
            "pattern": r"eval\(",
            "severity": "critical",
            "issue": "eval() permite ejecución dinámica",
            "suggestion": "Evita eval() o usa AST parser",
        },
        {
            "pattern": r"pickle\.loads\(",
            "severity": "high",
            "issue": "pickle es vulnerable a deserialización maliciosa",
            "suggestion": "Usa json o bibliotecas seguras",
        },
        {
            "pattern": r"yaml\.load\(",
            "severity": "high",
            "issue": "yaml.load() es vulnerable a deserialización",
            "suggestion": "Usa yaml.safe_load()",
        },
        {
            "pattern": r"assert\s+\w+\s*==",
            "severity": "low",
            "issue": "Asserts se ignoran con -O",
            "suggestion": "Usa if para validación",
        },
        {
            "pattern": r"SQL\s+.*\+.*%s",
            "severity": "critical",
            "issue": "SQL injection",
            "suggestion": "Usa ORM o parámetros bind",
        },
        {
            "pattern": r'password\s*=\s*["\']',
            "severity": "high",
            "issue": "Password hardcoded",
            "suggestion": "Usa variables de entorno",
        },
        {
            "pattern": r'secret\s*=\s*["\']',
            "severity": "high",
            "issue": "Secret hardcoded",
            "suggestion": "Usa variables de entorno",
        },
    ],
    "javascript": [
        {
            "pattern": r"eval\(",
            "severity": "critical",
            "issue": "eval() es peligroso",
            "suggestion": "Evita eval()",
        },
        {
            "pattern": r"innerHTML\s*=",
            "severity": "high",
            "issue": "XSS via innerHTML",
            "suggestion": "Usa textContent o sanitiza",
        },
        {
            "pattern": r"document\.write\(",
            "severity": "medium",
            "issue": "document.write() es obsoleto y peligroso",
            "suggestion": "Usa DOM API",
        },
        {
            "pattern": r"localStorage\.setItem\(.*password",
            "severity": "high",
            "issue": "No guardes passwords en localStorage",
            "suggestion": "Usa httpOnly cookies",
        },
    ],
    "rust": [
        {
            "pattern": r"unsafe\s+fn",
            "severity": "medium",
            "issue": "Función unsafe",
            "suggestion": "Minimiza código unsafe",
        },
    ],
}


def analyze_security(filepath, language):
    issues = []

    if language not in SECURITY_RULES:
        return issues

    try:
        with open(filepath, "r", encoding="utf-8") as f:
            lines = f.readlines()

        for rule in SECURITY_RULES[language]:
            pattern = re.compile(rule["pattern"], re.IGNORECASE)

            for i, line in enumerate(lines, 1):
                if pattern.search(line) and not line.strip().startswith("#"):
                    issues.append(
                        {
                            "file": str(filepath),
                            "line": i,
                            "severity": rule["severity"],
                            "issue": rule["issue"],
                            "suggestion": rule["suggestion"],
                            "code": line.strip()[:60],
                        }
                    )
    except:
        pass

    return issues


def get_language(filepath):
    ext = Path(filepath).suffix
    mapping = {".py": "python", ".rs": "rust", ".js": "javascript", ".ts": "javascript"}
    return mapping.get(ext, None)


def main():
    print("[AI] SECURITY-ADVISOR - Asesor de Seguridad IA")
    print("=" * 60)

    root = Path(".")
    all_issues = []

    for ext in ["*.py", "*.rs", "*.js", "*.ts", "*.jsx"]:
        for filepath in root.rglob(ext):
            if any(
                x in str(filepath)
                for x in ["node_modules", "target", "__pycache__", ".venv"]
            ):
                continue

            lang = get_language(filepath)
            if not lang:
                continue

            issues = analyze_security(filepath, lang)
            all_issues.extend(issues)

    print(f"\n[INFO] Vulnerabilidades encontradas: {len(all_issues)}")

    if not all_issues:
        print("\n[OK] ¡Tu código es seguro!")
        return 0

    critical = [i for i in all_issues if i["severity"] == "critical"]
    high = [i for i in all_issues if i["severity"] == "high"]
    medium = [i for i in all_issues if i["severity"] == "medium"]

    if critical:
        print(f"\n🚨 [CRITICAL] {len(critical)} vulnerabilidades CRÍTICAS:")
        for issue in critical:
            print(f"\n   📄 {issue['file']}:{issue['line']}")
            print(f"   🚨 {issue['issue']}")
            print(f"   💡 {issue['suggestion']}")

    if high:
        print(f"\n⚠️  [HIGH] {len(high)} vulnerabilidades ALTAS:")
        for issue in high[:5]:
            print(f"   - {issue['file']}:{issue['line']} → {issue['issue']}")

    if medium:
        print(f"\n⚡ [MEDIUM] {len(medium)} recomendaciones de seguridad")

    print("\n" + "=" * 60)
    print("[RESUMEN]")
    print(f"  🔴 Critical: {len(critical)}")
    print(f"  🟠 High: {len(high)}")
    print(f"  🟡 Medium: {len(medium)}")

    if critical or high:
        print("\n⚠️  ¡Corrige las vulnerabilidades antes de desplegar!")
        return 1

    return 0


if __name__ == "__main__":
    sys.exit(main())
