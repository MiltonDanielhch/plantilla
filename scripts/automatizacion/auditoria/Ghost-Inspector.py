#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
GHOST-INSPECTOR - Cazador de Código Fantasma
Detecta dependencias instaladas pero no usadas en el proyecto.
Trigger: Ejecución manual con `just ghost` o semanalmente
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import re
import json
from pathlib import Path


def find_imports_python(root_dir):
    """Encuentra todos los imports en archivos .py"""
    imports = set()
    for filepath in Path(root_dir).rglob("*.py"):
        if "__pycache__" in str(filepath) or "venv" in str(filepath):
            continue
        try:
            with open(filepath, "r", encoding="utf-8", errors="ignore") as f:
                for line in f:
                    match = re.match(
                        r"^\s*(?:import\s+(\w+)|from\s+(\w+)\s+import)", line
                    )
                    if match:
                        module = match.group(1) or match.group(2)
                        imports.add(module)
        except:
            pass
    return imports


def find_imports_js(root_dir):
    """Encuentra todos los imports en archivos .js/.ts"""
    imports = set()
    patterns = ["*.js", "*.jsx", "*.ts", "*.tsx"]

    for pattern in patterns:
        for filepath in Path(root_dir).rglob(pattern):
            if "node_modules" in str(filepath):
                continue
            try:
                with open(filepath, "r", encoding="utf-8", errors="ignore") as f:
                    content = f.read()
                    es6_matches = re.findall(
                        r'(?:import|from)\s+[\'"](\S+)[\'"]', content
                    )
                    commonjs_matches = re.findall(
                        r'require\([\'"](\S+)[\'"]\)', content
                    )
                    imports.update(es6_matches)
                    imports.update(commonjs_matches)
            except:
                pass
    return imports


def get_package_json_deps(root_dir):
    """Lee las dependencias de package.json"""
    pkg_path = Path(root_dir) / "package.json"
    if pkg_path.exists():
        with open(pkg_path, "r") as f:
            pkg = json.load(f)
            deps = set(pkg.get("dependencies", {}).keys())
            dev_deps = set(pkg.get("devDependencies", {}).keys())
            return deps | dev_deps
    return set()


def get_requirements_deps(root_dir):
    """Lee las dependencias de requirements.txt"""
    req_path = Path(root_dir) / "requirements.txt"
    deps = set()
    if req_path.exists():
        with open(req_path, "r") as f:
            for line in f:
                line = line.strip()
                if line and not line.startswith("#"):
                    match = re.match(r"^([a-zA-Z0-9_-]+)", line)
                    if match:
                        deps.add(match.group(1).lower())
    return deps


def main():
    print("[GHOST] GHOST-INSPECTOR - Cazador de Codigo Fantasma")
    print("=" * 50)

    root_dir = os.getcwd()

    print("\n[INFO] Detectando tipo de proyecto...")

    py_imports = find_imports_python(root_dir)
    js_imports = find_imports_js(root_dir)

    all_imports = py_imports | js_imports
    print(f"   [OK] Imports Python encontrados: {len(py_imports)}")
    print(f"   [OK] Imports JS/TS encontrados: {len(js_imports)}")

    ghost_deps = set()
    project_type = None

    pkg_deps = get_package_json_deps(root_dir)
    if pkg_deps:
        project_type = "Node.js"
        print(f"\n[INFO] Dependencias en package.json: {len(pkg_deps)}")
        ghost_deps = pkg_deps - all_imports
        print(f"   [OK] Dependencias usadas en codigo: {len(pkg_deps - ghost_deps)}")

    req_deps = get_requirements_deps(root_dir)
    if req_deps:
        project_type = "Python"
        print(f"\n[INFO] Dependencias en requirements.txt: {len(req_deps)}")
        ghost_deps = req_deps - all_imports
        print(f"   [OK] Dependencias usadas en codigo: {len(req_deps - ghost_deps)}")

    if not project_type:
        print("\n[WARNING] No se detecto requirements.txt ni package.json")
        return 1

    print("\n" + "=" * 50)

    if ghost_deps:
        print(f"[ALERTA] Se detectaron {len(ghost_deps)} dependencia(s) NO USADA(S):\n")
        for dep in sorted(ghost_deps):
            print(f"   [GHOST] {dep}")
        print(
            "\n[INFO] Recomendacion: Ejecuta `pip uninstall` o `npm uninstall` para eliminar."
        )
    else:
        print("[OK] Todas las dependencias estan siendo usadas! No hay fantasmas.")

    return 0 if not ghost_deps else 1


if __name__ == "__main__":
    sys.exit(main())
