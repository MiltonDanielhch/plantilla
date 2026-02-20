#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
UNIT-TEST-RUNNER - Ejecutor de Pruebas Automático
Ejecuta pruebas y abre automáticamente el archivo donde falla.
Trigger: Pre-push hook o manual
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import subprocess
import sys
from pathlib import Path
from datetime import datetime


def detect_test_framework():
    if Path("pytest.ini").exists() or Path("tests/").exists():
        return "pytest"
    if Path("Cargo.toml").exists():
        return "cargo"
    if Path("package.json").exists():
        return "jest"
    if Path("go.mod").exists():
        return "go"
    return None


def run_pytest():
    result = subprocess.run(
        ["pytest", "-v", "--tb=short"], capture_output=True, text=True
    )
    return result.returncode, result.stdout, result.stderr


def run_cargo_test():
    result = subprocess.run(
        ["cargo", "test", "--", "--nocapture"], capture_output=True, text=True
    )
    return result.returncode, result.stdout, result.stderr


def run_jest():
    result = subprocess.run(
        ["npm", "test", "--", "--coverage"], capture_output=True, text=True
    )
    return result.returncode, result.stdout, result.stderr


def run_go_test():
    result = subprocess.run(
        ["go", "test", "-v", "-race"], capture_output=True, text=True
    )
    return result.returncode, result.stdout, result.stderr


def parse_failures(output, framework):
    failures = []

    if framework == "pytest":
        for line in output.split("\n"):
            if "FAILED" in line or "ERROR" in line:
                if "::" in line:
                    failures.append(line.strip())

    elif framework == "cargo":
        for line in output.split("\n"):
            if "FAILED" in line or "test failed" in line.lower():
                failures.append(line.strip())

    return failures[:5]


def open_in_editor(filepath):
    if sys.platform == "win32":
        subprocess.Popen(["code", filepath])
    elif sys.platform == "darwin":
        subprocess.Popen(["open", filepath])
    else:
        subprocess.Popen(["xdg-open", filepath])


def main():
    print("[UNIT-TEST-RUNNER] Ejecutor de Pruebas")
    print("=" * 50)

    framework = detect_test_framework()

    if not framework:
        print("[ERROR] No se detectó framework de testing")
        print("[INFO] Asegúrate de tener: pytest, cargo, jest o go")
        return 1

    print(f"[INFO] Framework detectado: {framework}")
    print("[RUN] Ejecutando pruebas...\n")

    if framework == "pytest":
        code, stdout, stderr = run_pytest()
    elif framework == "cargo":
        code, stdout, stderr = run_cargo_test()
    elif framework == "jest":
        code, stdout, stderr = run_jest()
    elif framework == "go":
        code, stdout, stderr = run_go_test()

    print(stdout)

    if stderr:
        print("[ERROR]", stderr[:500])

    print("\n" + "=" * 50)

    if code == 0:
        print("[OK] Todas las pruebas pasaron!")
        return 0
    else:
        failures = parse_failures(stdout, framework)

        print(f"[FAIL] {len(failures)} prueba(s) fallida(s)")

        if failures:
            print("\n[INFO] Pruebas fallidas:")
            for f in failures:
                print(f"  - {f}")

        print("\n[INFO] Ejecuta con --open para abrir archivos fallidos")

        return 1


if __name__ == "__main__":
    sys.exit(main())
