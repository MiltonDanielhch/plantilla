#!/usr/bin/env python3
"""
PRUNING-SCRIPT - Limpiador de Basura
Elimina archivos innecesarios: __pycache__, .pyc, node_modules, logs, etc.
Trigger: Ejecución manual con `just util-clean` o cron (diario)
"""

import os
import shutil
import sys
import codecs
from pathlib import Path

# Fix for Windows UTF-8 encoding
if sys.platform == "win32":
    sys.stdout = codecs.getwriter("utf-8")(sys.stdout.buffer, "strict")
    sys.stderr = codecs.getwriter("utf-8")(sys.stderr.buffer, "strict")

DRY_RUN = False

PATTERNS_TO_CLEAN = {
    "__pycache__": "directorios",
    "*.pyc": "archivos",
    "*.pyo": "archivos",
    "*.pyd": "archivos",
    ".DS_Store": "archivos",
    "Thumbs.db": "archivos",
    "*.log": "archivos",
    "*.tmp": "archivos",
    "*.swp": "archivos",
    "node_modules": "directorios",
    ".cache": "directorios",
    "dist": "directorios",
    "build": "directorios",
    "target": "directorios",
    ".venv": "directorios",
    "venv": "directorios",
}


def get_size_formatted(size_bytes):
    for unit in ["B", "KB", "MB", "GB"]:
        if size_bytes < 1024:
            return f"{size_bytes:.2f} {unit}"
        size_bytes /= 1024
    return f"{size_bytes:.2f} TB"


def clean_directory(root_dir):
    removed_count = 0
    removed_size = 0
    errors = []

    for pattern, ptype in PATTERNS_TO_CLEAN.items():
        root_path = Path(root_dir)

        if ptype == "directorios":
            for dirpath in root_path.rglob(pattern):
                try:
                    if DRY_RUN:
                        print(f"🗑️ [DRY-RUN] Eliminar directorio: {dirpath}")
                    else:
                        size = sum(
                            f.stat().st_size for f in dirpath.rglob("*") if f.is_file()
                        )
                        shutil.rmtree(dirpath)
                        removed_count += 1
                        removed_size += size
                        print(f"🗑️ Eliminado: {dirpath}")
                except Exception as e:
                    errors.append(f"{dirpath}: {e}")

        elif ptype == "archivos":
            for filepath in root_path.rglob(pattern):
                try:
                    if DRY_RUN:
                        print(f"🗑️ [DRY-RUN] Eliminar archivo: {filepath}")
                    else:
                        size = filepath.stat().st_size
                        filepath.unlink()
                        removed_count += 1
                        removed_size += size
                        print(f"🗑️ Eliminado: {filepath}")
                except Exception as e:
                    errors.append(f"{filepath}: {e}")

    return removed_count, removed_size, errors


def main():
    global DRY_RUN

    print("🧹 PRUNING-SCRIPT - Limpiador de Sintonía")
    print("=" * 50)

    if "--dry-run" in sys.argv:
        DRY_RUN = True
        print("🔍 Modo SIMULACIÓN (no se eliminará nada)\n")

    root_dir = os.getcwd()
    print(f"📂 Limpiando: {root_dir}\n")

    count, size, errors = clean_directory(root_dir)

    print("\n" + "=" * 50)
    print(f"✅ Elementos procesados: {count}")
    print(f"💾 Espacio liberado: {get_size_formatted(size)}")

    if errors:
        print(f"\n⚠️ Errores ({len(errors)}):")
        for err in errors[:5]:
            print(f"   • {err}")

    if DRY_RUN:
        print("\n💡 Ejecuta sin --dry-run para eliminar realmente los archivos.")

    return 0


if __name__ == "__main__":
    sys.exit(main())
