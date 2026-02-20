#!/usr/bin/env python3
"""
BULK-RENAMER-REPLACER - Reemplazo Masivo
Busca y reemplaza texto en múltiples archivos simultáneamente.
Trigger: Ejecución manual con `just util-bulk`
"""

import os
import re
import sys
from pathlib import Path


def get_extensions():
    return [
        ".py",
        ".js",
        ".ts",
        ".jsx",
        ".tsx",
        ".rs",
        ".astro",
        ".html",
        ".css",
        ".json",
        ".md",
        ".yaml",
        ".yml",
        ".toml",
        ".txt",
    ]


def preview_replace(root_dir, search, replace, extensions):
    files_affected = []
    total_matches = 0

    for ext in extensions:
        for filepath in Path(root_dir).rglob(f"*{ext}"):
            if "__pycache__" in str(filepath) or "node_modules" in str(filepath):
                continue
            try:
                with open(filepath, "r", encoding="utf-8", errors="ignore") as f:
                    content = f.read()
                    matches = len(re.findall(re.escape(search), content))
                    if matches > 0:
                        files_affected.append((str(filepath), matches))
                        total_matches += matches
            except:
                pass

    return files_affected, total_matches


def execute_replace(root_dir, search, replace, extensions):
    files_changed = 0
    total_replacements = 0
    errors = []

    for ext in extensions:
        for filepath in Path(root_dir).rglob(f"*{ext}"):
            if "__pycache__" in str(filepath) or "node_modules" in str(filepath):
                continue
            try:
                with open(filepath, "r", encoding="utf-8", errors="ignore") as f:
                    content = f.read()

                new_content = content.replace(search, replace)

                if new_content != content:
                    with open(filepath, "w", encoding="utf-8") as f:
                        f.write(new_content)
                    count = content.count(search)
                    files_changed += 1
                    total_replacements += count
                    print(f"   ✅ {filepath}: {count} reemplazo(s)")
            except Exception as e:
                errors.append(f"{filepath}: {e}")

    return files_changed, total_replacements, errors


def main():
    print("📦 BULK-RENAMER-REPLACER")
    print("=" * 50)

    if len(sys.argv) < 3:
        print(" Uso: python Bulk-Renamer-Replacer.py <buscar> <reemplazar> [--preview]")
        print(" Ejemplo: python Bulk-Renamer-Replacer.py 'OldClass' 'NewClass'")
        print("          python Bulk-Renamer-Replacer.py 'foo' 'bar' --preview")
        return 1

    search = sys.argv[1]
    replace = sys.argv[2]
    preview_mode = "--preview" in sys.argv

    root_dir = os.getcwd()
    extensions = get_extensions()

    print(f"🔍 Buscando: '{search}'")
    print(f"🔄 Reemplazando con: '{replace}'")
    print(f"📂 En directorio: {root_dir}")
    print()

    files, matches = preview_replace(root_dir, search, replace, extensions)

    if not files:
        print("✅ No se encontraron coincidencias.")
        return 0

    print(f"📄 Archivos afectados: {len(files)}")
    print(f"🔢 Total de coincidencias: {matches}\n")

    for filepath, count in files[:10]:
        print(f"   • {filepath}: {count}")

    if len(files) > 10:
        print(f"   ... y {len(files) - 10} más")

    print()

    if preview_mode:
        print("💡 Modo PREVIEW. Usa sin --preview para aplicar cambios.")
        return 0

    confirm = input("¿Aplicar cambios? (s/n): ").strip().lower()
    if confirm != "s":
        print("❌ Cancelado.")
        return 0

    print("\n⚡ Aplicando cambios...\n")
    changed, replacements, errors = execute_replace(
        root_dir, search, replace, extensions
    )

    print("\n" + "=" * 50)
    print(f"✅ Archivos modificados: {changed}")
    print(f"🔄 Total de reemplazos: {replacements}")

    if errors:
        print(f"\n⚠️ Errores: {len(errors)}")
        for err in errors[:3]:
            print(f"   • {err}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
