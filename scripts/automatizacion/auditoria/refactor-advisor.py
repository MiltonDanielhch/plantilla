#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
REFACTOR-ADVISOR - Asesor de Refactorización
Analiza el código y sugiere refactorizaciones específicas.
Trigger: Análisis automático o manual
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import re
from pathlib import Path
from collections import defaultdict
from datetime import datetime


def detect_long_functions(filepath):
    suggestions = []

    try:
        with open(filepath, "r", encoding="utf-8") as f:
            lines = f.readlines()

        in_function = False
        func_start = 0
        func_name = ""
        func_lines = []

        for i, line in enumerate(lines, 1):
            if re.match(r"def\s+\w+", line):
                if in_function and len(func_lines) > 30:
                    suggestions.append(
                        {
                            "type": "long_function",
                            "file": str(filepath),
                            "line": func_start,
                            "name": func_name,
                            "lines": len(func_lines),
                            "suggestion": f"Función '{func_name}' tiene {len(func_lines)} líneas. Considera dividirla.",
                        }
                    )

                in_function = True
                func_start = i
                func_name = re.search(r"def\s+(\w+)", line).group(1)
                func_lines = [line]
            elif in_function:
                func_lines.append(line)
                if line.strip() and not line[0].isspace():
                    in_function = False

    except:
        pass

    return suggestions


def detect_duplicate_code(filepath):
    suggestions = []

    try:
        with open(filepath, "r", encoding="utf-8") as f:
            lines = f.readlines()

        code_blocks = defaultdict(list)

        buffer = []
        key = ""

        for i, line in enumerate(lines):
            stripped = line.strip()
            if len(stripped) > 20 and not stripped.startswith("#"):
                key = stripped[:30]
                buffer.append((i + 1, stripped))

        for (line_num, code), count in [
            (k, len(v)) for k, v in code_blocks.items() if len(v) > 2
        ]:
            suggestions.append(
                {
                    "type": "duplicate_code",
                    "file": str(filepath),
                    "line": line_num,
                    "code": code[:50],
                    "suggestion": f"Código duplicado detectado. Extrae a función reutilizable.",
                }
            )

    except:
        pass

    return suggestions


def detect_nested_code(filepath):
    suggestions = []

    try:
        with open(filepath, "r", encoding="utf-8") as f:
            lines = f.readlines()

        indent_level = 0
        max_nesting = 0
        max_nesting_line = 0

        for i, line in enumerate(lines, 1):
            if line.strip().startswith(("if", "for", "while", "with")):
                current_indent = len(line) - len(line.lstrip())
                if current_indent > max_nesting:
                    max_nesting = current_indent
                    max_nesting_line = i

        if max_nesting > 16:
            suggestions.append(
                {
                    "type": "deep_nesting",
                    "file": str(filepath),
                    "line": max_nesting_line,
                    "suggestion": "Nivel de indentación muy alto. Considera extraer lógica a funciones.",
                }
            )

    except:
        pass

    return suggestions


def suggest_patterns(filepath):
    suggestions = []

    try:
        with open(filepath, "r", encoding="utf-8") as f:
            content = f.read()

        if re.search(r"if\s+\w+\s+is\s+None:", content):
            suggestions.append(
                {
                    "type": "pattern",
                    "file": str(filepath),
                    "suggestion": 'Usa "if variable is None:" → considera usar Optional y match/case',
                }
            )

        if re.search(r"for\s+\w+\s+in\s+range\(len\(", content):
            suggestions.append(
                {
                    "type": "pattern",
                    "file": str(filepath),
                    "suggestion": "Usa enumerate() en lugar de range(len())",
                }
            )

        if re.search(r"\.append\(.*\).*\.append\(", content):
            suggestions.append(
                {
                    "type": "pattern",
                    "file": str(filepath),
                    "suggestion": "Considera usar list comprehension",
                }
            )

        if re.search(r"if\s+\w+\.get\(", content):
            suggestions.append(
                {
                    "type": "pattern",
                    "file": str(filepath),
                    "suggestion": "Usa .get(key, default) de forma más elegante",
                }
            )

    except:
        pass

    return suggestions


def main():
    print("[AI] REFACTOR-ADVISOR - Asesor de Refactorización")
    print("=" * 60)

    root = Path(".")
    all_suggestions = []

    for ext in ["*.py"]:
        for filepath in root.rglob(ext):
            if any(
                x in str(filepath)
                for x in ["node_modules", "target", "__pycache__", ".venv", "venv"]
            ):
                continue

            print(f"[ANALYZING] {filepath}")

            all_suggestions.extend(detect_long_functions(filepath))
            all_suggestions.extend(detect_duplicate_code(filepath))
            all_suggestions.extend(detect_nested_code(filepath))
            all_suggestions.extend(suggest_patterns(filepath))

    print(f"\n[INFO] Sugerencias encontradas: {len(all_suggestions)}")

    if not all_suggestions:
        print("\n[OK] ¡Tu código está bien refactorizado!")
        return 0

    print("\n[SUGERENCIAS DE REFACTORIZACIÓN]")
    print("-" * 60)

    for i, sug in enumerate(all_suggestions[:15], 1):
        print(f"\n{i}. [{sug['type'].upper()}]")

        if "file" in sug:
            print(f"   📄 {sug['file']}")
        if "line" in sug:
            print(f"   📍 Línea: {sug['line']}")
        if "name" in sug:
            print(f"   🔧 Función: {sug['name']} ({sug['lines']} líneas)")

        print(f"   💡 {sug['suggestion']}")

    print("\n" + "=" * 60)
    print(f"[INFO] Total: {len(all_suggestions)} sugerencias")
    print("[TIP] Prioriza las de tipo 'long_function' y 'deep_nesting'")

    return 0


if __name__ == "__main__":
    sys.exit(main())
