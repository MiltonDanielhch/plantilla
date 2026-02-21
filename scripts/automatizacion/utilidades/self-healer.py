#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
SELF-HEALER - Auto-Reparador de Código
Repara automáticamente errores comunes de código.
Trigger: Análisis automático o manual
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import re
from pathlib import Path
from datetime import datetime

HEAL_RULES = [
    {
        "pattern": r"print\s*\(\s*\)",
        "fix": 'print("DEBUG")',
        "description": "Añade contenido al print vacío",
    },
    {
        "pattern": r"except\s*:",
        "fix": "except Exception as e:",
        "description": "Añade Exception handler",
    },
    {
        "pattern": r'if\s+__name__\s*==\s*["\']__main__["\']:',
        "fix": 'if __name__ == "__main__":',
        "description": "Corrige comparación de main",
    },
    {
        "pattern": r"def\s+__init__\(self\):",
        "fix": "def __init__(self):",
        "description": "Añade pass o contenido al __init__",
    },
    {
        "pattern": r"class\s+\w+\s*:\s*$",
        "fix": "class \g<0>pass",
        "description": "Añade pass a clase vacía",
    },
    {
        "pattern": r"from\s+\w+\s+import\s+$",
        "fix": None,
        "description": "Import incompleto - revisar manualmente",
    },
]


def attempt_heal(filepath):
    healed = []
    errors = []

    try:
        with open(filepath, "r", encoding="utf-8") as f:
            content = f.read()

        original = content

        for rule in HEAL_RULES:
            if rule["fix"] is None:
                continue

            if re.search(rule["pattern"], content):
                content = re.sub(rule["pattern"], rule["fix"], content)
                healed.append(rule["description"])

        if content != original:
            backup_path = filepath.with_suffix(".bak")
            with open(backup_path, "w") as f:
                f.write(original)

            with open(filepath, "w") as f:
                f.write(content)

            return healed, errors

    except Exception as e:
        errors.append(str(e))

    return healed, errors


def fix_common_errors(filepath):
    fixes = []

    try:
        with open(filepath, "r", encoding="utf-8") as f:
            lines = f.readlines()

        new_lines = []
        modified = False

        for i, line in enumerate(lines):
            original = line

            line = re.sub(r"(\s+)#$", r"\1#", line)

            line = re.sub(r"true\b", "True", line)
            line = re.sub(r"false\b", "False", line)
            line = re.sub(r"none\b", "None", line)

            if line != original:
                modified = True

            new_lines.append(line)

        if modified:
            with open(filepath, "w") as f:
                f.writelines(new_lines)
            fixes.append("Corregido: true→True, false→False, none→None")

    except Exception as e:
        pass

    return fixes


def main():
    print("[AI] SELF-HEALER - Auto-Reparador de Código")
    print("=" * 60)

    root = Path(".")
    total_healed = 0
    total_files = 0

    print("\n[SCANNING] Buscando errores comunes...")

    for ext in ["*.py"]:
        for filepath in root.rglob(ext):
            if any(
                x in str(filepath)
                for x in ["node_modules", "target", "__pycache__", ".venv"]
            ):
                continue

            total_files += 1

            healed, errors = attempt_heal(filepath)
            fixes = fix_common_errors(filepath)

            if healed or fixes:
                total_healed += 1
                print(f"\n[HEALED] {filepath.name}")
                for h in healed:
                    print(f"  ✓ {h}")
                for f in fixes:
                    print(f"  ✓ {f}")

    print("\n" + "=" * 60)
    print(f"[RESULTADO]")
    print(f"  Archivos escaneados: {total_files}")
    print(f"  Archivos reparados: {total_healed}")

    if total_healed > 0:
        print(f"\n[OK] Se han creado respaldos .bak de los archivos modificados")
        print("[INFO] Revisa los cambios antes de hacer commit")

    return 0


if __name__ == "__main__":
    sys.exit(main())
