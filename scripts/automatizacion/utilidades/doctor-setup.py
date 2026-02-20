#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
DOCTOR-SETUP - Verificación de Entorno
Verifica que la computadora tenga todo lo necesario para desarrollar.
Trigger: Onboarding de nuevos miembros o manual
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import platform
import subprocess
import shutil
from pathlib import Path


def check_command(cmd, name, required=True):
    result = shutil.which(cmd)
    if result:
        print(f"   [OK] {name}: {result}")
        return True
    else:
        status = "[OK]" if not required else "[MISSING]"
        print(f"   {status} {name}: No encontrado")
        return required == False


def check_version(cmd, args, min_version, name):
    try:
        result = subprocess.run(
            [cmd] + args.split(), capture_output=True, text=True, timeout=5
        )
        version = result.stdout.strip()

        if min_version and min_version.lower() in version.lower():
            print(f"   [OK] {name}: {version[:50]}")
            return True
        else:
            print(f"   [WARN] {name}: {version[:50]}")
            return True
    except:
        print(f"   [WARN] {name}: No se pudo verificar")
        return True


def check_python_env():
    print("\n🐍 Python:")
    check_command("python", "Python", required=False)
    check_command("pip", "pip", required=False)

    if Path("requirements.txt").exists():
        print("   [OK] requirements.txt existe")
    else:
        print("   [WARN] requirements.txt no existe")


def check_node_env():
    print("\n🟢 Node.js:")
    check_command("node", "Node.js", required=False)
    check_command("npm", "npm", required=False)

    if Path("package.json").exists():
        print("   [OK] package.json existe")
        if Path("node_modules").exists():
            print("   [OK] node_modules instalado")
        else:
            print("   [WARN] Ejecuta: npm install")
    else:
        print("   [WARN] package.json no existe")


def check_rust_env():
    print("\n🦀 Rust:")
    check_command("rustc", "Rust", required=False)
    check_command("cargo", "Cargo", required=False)


def check_docker():
    print("\n🐳 Docker:")
    docker_ok = check_command("docker", "Docker", required=False)

    if docker_ok:
        try:
            result = subprocess.run(
                ["docker", "ps"], capture_output=True, text=True, timeout=5
            )
            if result.returncode == 0:
                print("   [OK] Docker daemon funcionando")
        except:
            print("   [WARN] Docker no está corriendo")


def check_git():
    print("\n📦 Git:")
    check_command("git", "Git", required=False)

    if Path(".git").exists():
        print("   [OK] Repositorio Git inicializado")
    else:
        print("   [WARN] No es un repositorio Git")


def check_vscode():
    print("\n🎨 VS Code:")
    code_cmd = "code" if platform.system() != "Windows" else "code.cmd"
    check_command(code_cmd, "VS Code", required=False)


def check_dirs():
    print("\n📁 Estructura del proyecto:")

    required_dirs = ["src", "backend", "frontend", "scripts", "logs"]
    for d in required_dirs:
        if Path(d).exists():
            print(f"   [OK] {d}/")
        else:
            print(f"   [WARN] {d}/ no existe")


def check_env_vars():
    print("\n🔐 Variables de entorno:")

    important_vars = ["DATABASE_URL", "API_KEY", "SECRET_KEY"]
    found = 0

    for var in important_vars:
        if var in os.environ:
            print(f" {var}   [OK] configurada")
            found += 1
        else:
            print(f"   [INFO] {var} no configurada (puede ser normal)")

    if Path(".env").exists():
        print("   [OK] Archivo .env existe")
    elif Path(".env.example").exists():
        print("   [WARN] Copia .env.example a .env")


def main():
    print("=" * 50)
    print("🩺 DOCTOR-SETUP - Verificación de Entorno")
    print("=" * 50)

    print(f"\n🖥️  Sistema: {platform.system()} {platform.release()}")
    print(f"📂 Directorio: {os.getcwd()}")

    check_python_env()
    check_node_env()
    check_rust_env()
    check_docker()
    check_git()
    check_vscode()
    check_dirs()
    check_env_vars()

    print("\n" + "=" * 50)
    print("💡 Próximos pasos:")
    print("   1. Copia .env.example a .env y configura")
    print("   2. Ejecuta: npm install")
    print("   3. Ejecuta: just bootstrap")
    print("=" * 50)

    return 0


if __name__ == "__main__":
    sys.exit(main())
