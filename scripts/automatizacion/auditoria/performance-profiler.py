#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
PERFORMANCE-PROFILER - Analizador de Rendimiento
Detecta código ineficiente y sugiere optimizaciones.
Trigger: Análisis automático o manual
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import re
from pathlib import Path

PERFORMANCE_RULES = {
    "python": [
        {
            "pattern": r"\.keys\(\)|\.values\(\)|\.items\(\)",
            "issue": "Itera sobre dict.keys() innecesario",
            "suggestion": "Itera directamente sobre el dict",
        },
        {
            "pattern": r"for\s+.*\s+in\s+.*\.keys\(\):",
            "issue": "Uso de .keys() en for",
            "suggestion": "for key in dict: es más eficiente",
        },
        {
            "pattern": r"\[\s*i\s+for\s+i\s+in\s+range\(",
            "issue": "Lista comprensión con range",
            "suggestion": "Usa list(range(n)) directamente",
        },
        {
            "pattern": r"\.append\(.*\+",
            "issue": "Concatenación en append",
            "suggestion": "Usa una lista temporal o join al final",
        },
        {
            "pattern": r'"\s*\+\s*".*"\s*\+\s*"',
            "issue": "Concatenación de strings",
            "suggestion": "Usa f-strings o .join()",
        },
        {
            "pattern": r"str\(.*\)\s*==",
            "issue": "str() innecesario en comparación",
            "suggestion": "Compara directamente",
        },
        {
            "pattern": r"len\(.*\)\s*==\s*0",
            "issue": "len() == 0",
            "suggestion": 'Usa "if not lista:" o "if lista:"',
        },
        {
            "pattern": r"if\s+\w+\s+==\s+True:",
            "issue": "Comparación con True",
            "suggestion": 'Usa "if variable:"',
        },
        {
            "pattern": r"while\s+True:",
            "issue": "While True sin break",
            "suggestion": "Asegúrate de tener condición de salida",
        },
        {
            "pattern": r"import\s+\*",
            "issue": "Import wildcard",
            "suggestion": "Importa solo lo necesario",
        },
    ],
    "javascript": [
        {
            "pattern": r"array\[array\.length\]",
            "issue": "Acceso a índice fuera de rango",
            "suggestion": "Usa array[array.length - 1]",
        },
        {
            "pattern": r"for\s*\(\s*var\s+\w+\s*=\s*0.*\.length",
            "issue": "Cachea length en for",
            "suggestion": "let len = arr.length; for(let i=0; i<len; i++)",
        },
        {
            "pattern": r"\.push\(.*\)\s*;?\s*\.push\(",
            "issue": "Múltiples push",
            "suggestion": "Usa spread operator o concat",
        },
        {
            "pattern": r"JSON\.parse\(JSON\.stringify\(",
            "issue": "Deep clone ineficiente",
            "suggestion": "Usa structuredClone() o lodash.cloneDeep()",
        },
    ],
    "rust": [
        {
            "pattern": r"\.clone\(\)",
            "issue": "Clonación innecesaria",
            "suggestion": "Usa referencias o Rc/Arc cuando sea posible",
        },
        {
            "pattern": r"for\s+.*\.iter\(\).*\.iter\(\)",
            "issue": "Doble iter() innecesario",
            "suggestion": "Solo necesitas un iter()",
        },
    ],
}


def analyze_performance(filepath, language):
    issues = []

    if language not in PERFORMANCE_RULES:
        return issues

    try:
        with open(filepath, "r", encoding="utf-8") as f:
            lines = f.readlines()

        for rule in PERFORMANCE_RULES[language]:
            pattern = re.compile(rule["pattern"])

            for i, line in enumerate(lines, 1):
                if pattern.search(line):
                    issues.append(
                        {
                            "file": str(filepath),
                            "line": i,
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


def estimate_impact(issue):
    impacts = {"O(n²)": "Alto", "memoria": "Medio", "string": "Medio", "iter()": "Bajo"}

    for key, impact in impacts.items():
        if key in issue.get("suggestion", "").lower():
            return impact

    return "Bajo"


def main():
    print("[AI] PERFORMANCE-PROFILER - Analizador de Rendimiento")
    print("=" * 60)

    root = Path(".")
    all_issues = []

    for ext in ["*.py", "*.rs", "*.js", "*.ts"]:
        for filepath in root.rglob(ext):
            if any(
                x in str(filepath)
                for x in ["node_modules", "target", "__pycache__", ".venv"]
            ):
                continue

            lang = get_language(filepath)
            if not lang:
                continue

            issues = analyze_performance(filepath, lang)
            all_issues.extend(issues)

    print(f"\n[INFO] Problemas de rendimiento encontrados: {len(all_issues)}")

    if not all_issues:
        print("\n[OK] ¡Tu código está optimizado!")
        return 0

    print("\n[PROBLEMAS DE RENDIMIENTO]")
    print("-" * 60)

    high_impact = [i for i in all_issues if estimate_impact(i) == "Alto"]
    medium_impact = [i for i in all_issues if estimate_impact(i) == "Medio"]
    low_impact = [i for i in all_issues if estimate_impact(i) == "Bajo"]

    if high_impact:
        print(f"\n[ALTO IMPACTO] {len(high_impact)} problemas:")
        for issue in high_impact[:5]:
            print(f"\n  📄 {issue['file']}:{issue['line']}")
            print(f"  ⚠️  {issue['issue']}")
            print(f"  💡 {issue['suggestion']}")

    if medium_impact:
        print(f"\n[MEDIO IMPACTO] {len(medium_impact)} problemas:")
        for issue in medium_impact[:3]:
            print(f"  - {issue['file']}:{issue['line']} → {issue['issue']}")

    if low_impact:
        print(f"\n[BAJO IMPACTO] {len(low_impact)} optimizaciones menores")

    print("\n" + "=" * 60)
    print("[RECOMENDACIONES]")
    print("1. Prioriza los problemas de alto impacto")
    print("2. Usa profilers como cProfile (Python) o cargo flamegraph (Rust)")
    print("3. Mide antes y después de optimizar")

    return 0


if __name__ == "__main__":
    sys.exit(main())
