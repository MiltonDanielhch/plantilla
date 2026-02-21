#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
CODE-ANALYZER - Analizador de Código IA
Analiza el código y sugiere mejoras automáticas basadas en best practices.
Trigger: Análisis automático o manual
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import re
import json
from pathlib import Path
from datetime import datetime

RULES = {
    "python": [
        {
            "pattern": r"def\s+\w+\(self\):",
            "issue": "Método sin type hints",
            "severity": "low",
            "suggestion": "Añade type hints: def metodo(self) -> None:",
        },
        {
            "pattern": r"except:",
            "issue": "Excepto vacío",
            "severity": "high",
            "suggestion": "Usa except Exception as e:",
        },
        {
            "pattern": r"print\(",
            "issue": "Print en producción",
            "severity": "medium",
            "suggestion": "Usa logging en lugar de print",
        },
        {
            "pattern": r"global\s+\w+",
            "issue": "Uso de global",
            "severity": "medium",
            "suggestion": "Evita variables globales, usa inyección de dependencias",
        },
        {
            "pattern": r"==\s*True|==\s*False",
            "issue": "Comparación redundante",
            "severity": "low",
            "suggestion": 'Usa "if variable:" o "if not variable:"',
        },
        {
            "pattern": r"if\s+\w+\s+==\s+\w+:",
            "issue": "String comparison",
            "severity": "medium",
            "suggestion": "Considera usar enum o constantes",
        },
        {
            "pattern": r"from\s+\w+\s+import\s+\*",
            "issue": "Import wildcard",
            "severity": "medium",
            "suggestion": "Importa solo lo que necesitas",
        },
    ],
    "rust": [
        {
            "pattern": r"println!",
            "issue": "println! en producción",
            "severity": "medium",
            "suggestion": "Usa log::info! o similar",
        },
        {
            "pattern": r"unsafe\s+{",
            "issue": "Bloque unsafe",
            "severity": "high",
            "suggestion": "Evita código unsafe o documenta por qué es necesario",
        },
        {
            "pattern": r"\.unwrap\(\)",
            "issue": "Uso de unwrap()",
            "severity": "medium",
            "suggestion": "Usa ? o manejo de errores adecuado",
        },
        {
            "pattern": r"mut\s+let",
            "issue": "Mutable binding",
            "severity": "low",
            "suggestion": "Considera inmutabilidad por defecto",
        },
    ],
    "javascript": [
        {
            "pattern": r"var\s+\w+",
            "issue": "Uso de var",
            "severity": "medium",
            "suggestion": "Usa const o let",
        },
        {
            "pattern": r"==\s+\w+|!=",
            "issue": "Comparación no estricta",
            "severity": "high",
            "suggestion": "Usa === o !==",
        },
        {
            "pattern": r"console\.log",
            "issue": "console.log en código",
            "severity": "medium",
            "suggestion": "Usa un logger apropiado",
        },
        {
            "pattern": r"function\s+\w+\s*\(",
            "issue": "Función legacy",
            "severity": "low",
            "suggestion": "Considera arrow functions",
        },
        {
            "pattern": r"await\s+fetch\(",
            "issue": "Fetch sin manejo de errores",
            "severity": "high",
            "suggestion": "Añade try/catch",
        },
    ],
}


def analyze_file(filepath, language):
    issues = []

    if language not in RULES:
        return issues

    try:
        with open(filepath, "r", encoding="utf-8") as f:
            lines = f.readlines()

        for rule in RULES[language]:
            pattern = re.compile(rule["pattern"])

            for i, line in enumerate(lines, 1):
                if pattern.search(line):
                    issues.append(
                        {
                            "file": str(filepath),
                            "line": i,
                            "issue": rule["issue"],
                            "severity": rule["severity"],
                            "suggestion": rule["suggestion"],
                            "code": line.strip()[:60],
                        }
                    )
    except:
        pass

    return issues


def get_language(filepath):
    ext = Path(filepath).suffix
    mapping = {
        ".py": "python",
        ".rs": "rust",
        ".js": "javascript",
        ".ts": "javascript",
        ".jsx": "javascript",
        ".tsx": "javascript",
    }
    return mapping.get(ext, None)


def calculate_score(issues):
    severity_weights = {"high": -20, "medium": -10, "low": -5}
    score = 100

    for issue in issues:
        score += severity_weights.get(issue["severity"], 0)

    return max(0, min(100, score))


def main():
    print("[AI] CODE-ANALYZER - Analizador de Código IA")
    print("=" * 60)

    root = Path(".")
    all_issues = []
    files_analyzed = 0

    for ext in ["*.py", "*.rs", "*.js", "*.ts", "*.jsx", "*.tsx"]:
        for filepath in root.rglob(ext):
            if any(
                x in str(filepath)
                for x in ["node_modules", "target", "__pycache__", ".venv"]
            ):
                continue

            lang = get_language(filepath)
            if not lang:
                continue

            files_analyzed += 1
            issues = analyze_file(filepath, lang)
            all_issues.extend(issues)

    print(f"\n[INFO] Archivos analizados: {files_analyzed}")
    print(f"[INFO] Issues encontrados: {len(all_issues)}")

    score = calculate_score(all_issues)
    print(f"\n[SCORE] Puntuación de calidad: {score}/100")

    if score >= 80:
        print("[OK] ¡Excelente código!")
    elif score >= 60:
        print("[WARN] Código aceptable pero mejorable")
    else:
        print("[ERROR] Código requiere atención")

    high_severity = [i for i in all_issues if i["severity"] == "high"]
    medium_severity = [i for i in all_issues if i["severity"] == "medium"]

    if high_severity:
        print(f"\n[ALERT] {len(high_severity)} issues de prioridad ALTA:")
        for issue in high_severity[:5]:
            print(f"  - {issue['file']}:{issue['line']}")
            print(f"    {issue['issue']}")
            print(f"    → {issue['suggestion']}")

    if medium_severity:
        print(f"\n[WARN] {len(medium_severity)} issues de prioridad MEDIA")

    output_file = Path("logs/code_analysis.json")
    output_file.parent.mkdir(parents=True, exist_ok=True)

    with open(output_file, "w") as f:
        json.dump(
            {
                "timestamp": datetime.now().isoformat(),
                "score": score,
                "files_analyzed": files_analyzed,
                "total_issues": len(all_issues),
                "issues": all_issues,
            },
            f,
            indent=2,
        )

    print(f"\n[OK] Reporte guardado: {output_file}")

    return 0 if score >= 60 else 1


if __name__ == "__main__":
    sys.exit(main())
