#!/usr/bin/env python3
"""
SNAPSHOT-SCRIPT - Auto-Backup & Sync
Crea comprimidos .zip con la fecha/hora para backups rápidos fuera de Git.
Trigger: Antes de cambios arriesgados o manualmente
"""

import os
import sys
import zipfile
import shutil
from datetime import datetime
from pathlib import Path

EXCLUDE = {
    ".git",
    "node_modules",
    "target",
    "__pycache__",
    ".venv",
    "venv",
    "dist",
    "build",
    ".env",
    "*.log",
    ".DS_Store",
    "Thumbs.db",
}


def create_snapshot(root_dir, snapshot_dir=None, name=None):
    if snapshot_dir is None:
        snapshot_dir = Path(root_dir) / "snapshots"

    snapshot_dir = Path(snapshot_dir)
    snapshot_dir.mkdir(parents=True, exist_ok=True)

    if name is None:
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        name = f"snapshot_{timestamp}"

    zip_path = snapshot_dir / f"{name}.zip"

    print(f"📦 SNAPSHOT: {name}")
    print(f"📂 Origen: {root_dir}")
    print(f"📂 Destino: {zip_path}")
    print()

    files_added = 0
    dirs_scanned = 0

    with zipfile.ZipFile(zip_path, "w", zipfile.ZIP_DEFLATED) as zipf:
        for root, dirs, files in os.walk(root_dir):
            dirs[:] = [d for d in dirs if d not in EXCLUDE]
            dirs_scanned += 1

            for file in files:
                if file in EXCLUDE or any(
                    file.endswith(ext) for ext in [".pyc", ".log", ".lock"]
                ):
                    continue

                filepath = Path(root) / file
                arcname = filepath.relative_to(root_dir)

                try:
                    zipf.write(filepath, arcname)
                    files_added += 1
                    if files_added % 50 == 0:
                        print(f"   📝 {files_added} archivos...")
                except Exception as e:
                    print(f"   ⚠️ Error con {filepath}: {e}")

    zip_size = zip_path.stat().st_size
    size_mb = zip_size / (1024 * 1024)

    print()
    print("=" * 50)
    print(f"✅ Snapshot creado exitosamente!")
    print(f"   📦 Archivos comprimidos: {files_added}")
    print(f"   💾 Tamaño: {size_mb:.2f} MB")
    print(f"   📍 Ubicación: {zip_path}")

    return zip_path


def list_snapshots(snapshot_dir=None):
    if snapshot_dir is None:
        snapshot_dir = Path(os.getcwd()) / "snapshots"

    snapshot_dir = Path(snapshot_dir)

    if not snapshot_dir.exists():
        print("📂 No hay snapshots aún.")
        return

    snapshots = sorted(
        snapshot_dir.glob("*.zip"), key=lambda p: p.stat().st_mtime, reverse=True
    )

    print("📜 SNAPSHOTS DISPONIBLES")
    print("=" * 50)

    for snap in snapshots:
        size_mb = snap.stat().st_size / (1024 * 1024)
        mtime = datetime.fromtimestamp(snap.stat().st_mtime)
        print(
            f"   • {snap.name} ({size_mb:.2f} MB) - {mtime.strftime('%Y-%m-%d %H:%M')}"
        )


def restore_snapshot(snapshot_path, target_dir=None):
    snapshot_path = Path(snapshot_path)

    if not snapshot_path.exists():
        print(f"❌ Snapshot no encontrado: {snapshot_path}")
        return 1

    if target_dir is None:
        target_dir = (
            Path(os.getcwd()) / f"restore_{datetime.now().strftime('%Y%m%d_%H%M%S')}"
        )

    target_dir = Path(target_dir)
    target_dir.mkdir(parents=True)

    print(f"🔄 Restaurando {snapshot_path.name}...")

    with zipfile.ZipFile(snapshot_path, "r") as zipf:
        zipf.extractall(target_dir)

    print(f"✅ Restaurado en: {target_dir}")
    return 0


def main():
    print("📸 SNAPSHOT-SCRIPT - Auto-Backup")
    print("=" * 50)

    if len(sys.argv) < 2:
        print(" Uso: python Snapshot-Script.py [comando] [args]")
        print(" Comandos:")
        print("   (sin args)    - Crear snapshot del proyecto actual")
        print("   --list        - Listar snapshots disponibles")
        print("   --restore <archivo> - Restaurar un snapshot")
        print("   --name <nombre>    - Nombre personalizado del snapshot")
        print()
        print(" Ejemplo: python Snapshot-Script.py")
        print("          python Snapshot-Script.py --name 'antes-experimento'")
        print("          python Snapshot-Script.py --list")
        return 1

    if sys.argv[1] == "--list":
        list_snapshots()
        return 0

    if sys.argv[1] == "--restore":
        if len(sys.argv) < 3:
            print("❌ Especifica el snapshot a restaurar")
            return 1
        return restore_snapshot(sys.argv[2])

    name = None
    for i, arg in enumerate(sys.argv):
        if arg == "--name" and i + 1 < len(sys.argv):
            name = sys.argv[i + 1]

    create_snapshot(os.getcwd(), name=name)
    return 0


if __name__ == "__main__":
    sys.exit(main())
