#!/usr/bin/env python3
"""
BOOTSTRAP-SCRIPT - Entorno Maestro
Configura el entorno de desarrollo automáticamente.
Trigger: Nueva máquina o reinstalación
"""

import os
import sys
import subprocess
import platform
from pathlib import Path

# Fix for Windows UTF-8 encoding
if sys.platform == "win32":
    import codecs

    sys.stdout = codecs.getwriter("utf-8")(sys.stdout.buffer, "strict")
    sys.stderr = codecs.getwriter("utf-8")(sys.stderr.buffer, "strict")

SYSTEM = platform.system()


def check_command(cmd, name):
    try:
        subprocess.run(cmd, capture_output=True, check=True)
        print(f"   ✅ {name}: instalado")
        return True
    except:
        print(f"   ❌ {name}: NO instalado")
        return False


def install_python():
    print("\n🐍 Python...")
    check_command(["python", "--version"], "Python")

    print("\n📦 Paquetes Python esenciales...")
    packages = ["pip", "virtualenv", "pytest", "black", "ruff"]

    for pkg in packages:
        try:
            subprocess.run(
                [sys.executable, "-m", pkg, "--version"],
                capture_output=True,
                check=True,
            )
            print(f"   ✅ {pkg}")
        except:
            print(f"   ℹ️ {pkg} no instalado (opcional)")


def install_nodejs():
    print("\n🟢 Node.js...")
    check_command(["node", "--version"], "Node.js")
    check_command(["npm", "--version"], "npm")

    print("\n📦 Paquetes globales de Node...")
    global_packages = ["typescript", "ts-node", "nodemon"]

    for pkg in global_packages:
        try:
            subprocess.run(["npm", "list", "-g", pkg], capture_output=True, check=True)
            print(f"   ✅ {pkg}")
        except:
            print(f"   ℹ️ {pkg} no instalado (opcional)")


def install_rust():
    print("\n🦀 Rust...")
    check_command(["rustc", "--version"], "Rust")
    check_command(["cargo", "--version"], "Cargo")


def install_docker():
    print("\�🐳 Docker...")
    if SYSTEM == "Windows":
        check_command(["docker", "--version"], "Docker Desktop")
    else:
        check_command(["docker", "--version"], "Docker")
        check_command(["docker-compose", "--version"], "Docker Compose")


def install_vscode_extensions():
    print("\n🎨 Extensiones de VS Code...")
    extensions = [
        "rust-lang.rust-analyzer",
        "dbaeumer.vscode-eslint",
        "esbenp.prettier-vscode",
        "ms-python.python",
        "ms-python.black-formatter",
    ]

    for ext in extensions:
        try:
            subprocess.run(
                ["code", "--install-extension", ext], capture_output=True, check=True
            )
            print(f"   ✅ {ext}")
        except:
            print(f"   ℹ️ {ext} no instalado")


def setup_git_hooks():
    print("\n🪝 Configurando Git Hooks...")

    hooks_dir = Path(".git/hooks")
    hooks_dir.mkdir(exist_ok=True)

    pre_commit = hooks_dir / "pre-commit"

    hook_content = """#!/bin/bash
# Pre-commit hook de Sintonía 3026

echo "🛡️ SHIELD: Verificando secretos..."
python infra/scripts/shield.py
SHIELD_STATUS=$?

if [ $SHIELD_STATUS -ne 0 ]; then
    echo "❌ Commit bloqueado por SHIELD"
    exit 1
fi

echo "✅ Commit aprobado"
exit 0
"""

    with open(pre_commit, "w", encoding="utf-8") as f:
        f.write(hook_content)

    os.chmod(pre_commit, 0o755)
    print("   ✅ pre-commit configurado")


def setup_project():
    print("\n📁 Configurando proyecto...")

    if Path("requirements.txt").exists():
        print("   📦 Instalando dependencias Python...")
        os.system(f"{sys.executable} -m pip install -r requirements.txt")

    if Path("package.json").exists():
        print("   📦 Instalando dependencias Node...")
        os.system("npm install")

    if Path("Cargo.toml").exists():
        print("   📦 Instalando dependencias Rust...")
        os.system("cargo fetch")


def main():
    print("🚀 BOOTSTRAP-SCRIPT - Entorno Maestro")
    print("=" * 50)
    print(f"🖥️  Sistema: {SYSTEM}")
    print(f"📂 Proyecto: {os.getcwd()}")

    print("\n" + "=" * 50)
    print("📋 VERIFICACIÓN DEL SISTEMA")

    if SYSTEM == "Windows":
        install_python()
        install_nodejs()
        install_docker()
    elif SYSTEM == "Linux":
        install_python()
        install_nodejs()
        install_rust()
        install_docker()
    elif SYSTEM == "Darwin":
        install_python()
        install_nodejs()
        install_rust()

    if Path(".git").exists():
        print("\n" + "=" * 50)
        print("🔧 CONFIGURACIÓN AVANZADA")

        setup_git_hooks()
        setup_project()

    print("\n" + "=" * 50)
    print("✅ Bootstrap completado!")
    print("\n💡 Próximos pasos:")
    print("   • Ejecuta 'just run-backend' para iniciar el backend")
    print("   • Ejecuta 'just run-frontend' para iniciar el frontend")
    print("   • Ejecuta 'just shield' para probar SHIELD")

    return 0


if __name__ == "__main__":
    sys.exit(main())
