#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import os
import sys
import codecs

# Fix for Windows UTF-8 encoding
if sys.platform == "win32":
    sys.stdout = codecs.getwriter("utf-8")(sys.stdout.buffer, "strict")
    sys.stderr = codecs.getwriter("utf-8")(sys.stderr.buffer, "strict")


def get_size_format(b, factor=1024, suffix="B"):
    """Convierte bytes a formato legible (MB, GB, etc.)"""
    for unit in ["", "K", "M", "G", "T", "P"]:
        if b < factor:
            return f"{b:.2f}{unit}{suffix}"
        b /= factor


def count_lines(file_path):
    try:
        with open(file_path, "r", encoding="utf-8", errors="ignore") as f:
            # Cuenta líneas que no estén vacías
            return sum(1 for line in f if line.strip())
    except:
        return 0


def analyze_full_project(directory, ignore_list=None):
    if ignore_list is None:
        # --- Sincronizado con .gitignore y .dockerignore Maestro ---
        ignore_list = {
            ".git",
            ".gitignore",
            ".dockerignore",
            ".DS_Store",
            ".vscode",
            ".idea",
            "node_modules",
            "target",
            "dist",
            ".astro",
            "__pycache__",
            ".venv",
            ".env",
            "backend.db",
            "backend.db-shm",
            "backend.db-wal",
            "uploads",
            "archive",
            "guia",
            "test-results",
            "playwright-report",
            "AUDITORIA_MASTER.md",
            "1.md",
            "falta.md",
        }

    tree_str = ""
    total_lines = 0
    total_size = 0

    # Extensiones de código sintonizadas con tu stack (Rust, Astro, Python, SQL)
    code_exts = {
        ".py",
        ".rs",
        ".ts",
        ".tsx",
        ".astro",
        ".js",
        ".sql",
        ".toml",
        ".css",
        ".html",
        ".yml",
        ".yaml",
    }

    try:
        # Filtrado de items ignorados
        items = [i for i in sorted(os.listdir(directory)) if i not in ignore_list]
    except PermissionError:
        return "[Acceso Denegado]\n", 0, 0

    for i, item in enumerate(items):
        path = os.path.join(directory, item)
        is_last = i == len(items) - 1
        connector = "└── " if is_last else "├── "

        # Obtener estadísticas de archivos/carpetas
        stats = os.stat(path)

        if os.path.isdir(path):
            # Análisis recursivo de carpetas
            subtree, lines, folder_size = analyze_full_project(path, ignore_list)
            tree_str += f"{connector}{item}/ [{get_size_format(folder_size)}]\n"

            prefix = "    " if is_last else "│   "
            indented_subtree = "".join(
                prefix + line for line in subtree.splitlines(keepends=True)
            )
            tree_str += indented_subtree

            total_lines += lines
            total_size += folder_size
        else:
            file_lines = 0
            item_size = stats.st_size
            if any(item.endswith(ext) for ext in code_exts):
                file_lines = count_lines(path)
                total_lines += file_lines

            total_size += item_size
            info = f"({file_lines} LoC | {get_size_format(item_size)})"
            tree_str += f"{connector}{item} {info}\n"

    return tree_str, total_lines, total_size


def generate_audit():
    # Ejecuta el análisis desde la raíz
    tree_content, total_lines, total_size = analyze_full_project(".")

    report = (
        f"# 🛠️ Auditoría de Software 3026\n\n"
        f"| Métrica | Valor |\n"
        f"| :--- | :--- |\n"
        f"| **Líneas de Código (Netas)** | {total_lines} LoC |\n"
        f"| **Peso Total del Proyecto** | {get_size_format(total_size)} |\n"
        f"| **Estado de Sintonía** | Activa |\n\n"
        f"### Mapa de Arquitectura y Pesos\n"
        f"```text\n{tree_content}```\n"
    )

    # Guarda el resultado en el archivo maestro
    with open("AUDITORIA_MASTER.md", "w", encoding="utf-8") as f:
        f.write(report)
    print(
        f"✅ Sintonía completada: {total_lines} LoC detectadas. Reporte generado en AUDITORIA_MASTER.md"
    )


if __name__ == "__main__":
    generate_audit()
