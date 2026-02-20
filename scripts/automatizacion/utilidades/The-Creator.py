#!/usr/bin/env python3
"""
THE-CREATOR - Generador de Boilerplate
Crea estructura de carpetas y archivos base para nuevos módulos.
Trigger: Ejecución manual cuando necesitas crear un nuevo componente
"""

import os
import sys
from pathlib import Path
from datetime import datetime

TEMPLATE_PY = '''"""
{MODULE_NAME} - Módulo de {MODULE_NAME}
Descripción del módulo...
"""

def main():
    print("🧪 {MODULE_NAME} ejecutado")
    return 0

if __name__ == "__main__":
    sys.exit(main())
'''

TEMPLATE_RS = """
// {MODULE_NAME}.rs
// Módulo: {MODULE_NAME}

fn main() {{
    println!("🧪 {MODULE_NAME} ejecutado");
}}
"""

TEMPLATE_ASTRO = """---
// {MODULE_NAME}.astro
import Layout from '../layouts/Layout.astro';
---

<Layout title="{MODULE_NAME}">
  <h1>{MODULE_NAME}</h1>
</Layout>
"""

TEMPLATE_TEST = '''"""
Tests para {MODULE_NAME}
"""
import pytest

def test_example():
    assert True
'''

TEMPLATE_README = """# {MODULE_NAME}

## Descripción
Módulo {MODULE_NAME} para el proyecto.

## Uso
```python
from {module_name_lower} import main
main()
```

## Estado
🟡 En desarrollo
"""

STRUCTURE_PY = [
    ("__init__.py", "# {module_name_lower}\n"),
    ("main.py", TEMPLATE_PY),
    ("test_main.py", TEMPLATE_TEST),
    ("README.md", TEMPLATE_README),
]

STRUCTURE_RS = [
    ("src/{module_name_lower}.rs", TEMPLATE_RS),
    ("README.md", TEMPLATE_README),
]

STRUCTURE_ASTRO = [
    ("src/pages/{module_name_lower}.astro", TEMPLATE_ASTRO),
    ("README.md", TEMPLATE_README),
]


def detect_type(name):
    name_lower = name.lower()
    if "frontend" in name_lower or "ui" in name_lower or "page" in name_lower:
        return "astro", STRUCTURE_ASTRO
    elif "backend" in name_lower or "api" in name_lower:
        return "rs", STRUCTURE_RS
    else:
        return "py", STRUCTURE_PY


def create_module(module_name, module_type=None, target_dir=None):
    if not module_name:
        print("❌ Debes especificar un nombre de módulo")
        return 1

    module_name_snake = module_name.replace("-", "_").replace(" ", "_").lower()
    module_name_title = module_name.replace("-", " ").title().replace(" ", "")

    if target_dir is None:
        target_dir = os.getcwd()

    if module_type is None:
        module_type, structure = detect_type(module_name)
    else:
        _, structure = {
            "py": ("py", STRUCTURE_PY),
            "rs": ("rs", STRUCTURE_RS),
            "astro": ("astro", STRUCTURE_ASTRO),
        }.get(module_type, ("py", STRUCTURE_PY))

    module_path = Path(target_dir) / module_name_snake

    if module_path.exists():
        print(f"❌ El módulo '{module_name_snake}' ya existe")
        return 1

    module_path.mkdir(parents=True)
    print(f"📁 Creando módulo: {module_name_snake} ({module_type})")

    for filename, template in structure:
        filepath = module_path / filename

        filepath.parent.mkdir(parents=True, exist_ok=True)

        content = filename.replace("{module_name_lower}", module_name_snake)
        content = content.replace("{MODULE_NAME}", module_name_title)

        if ".py" in filepath.suffix:
            content = TEMPLATE_PY.replace("{MODULE_NAME}", module_name_title)
        elif ".rs" in filepath.suffix:
            content = TEMPLATE_RS.replace("{MODULE_NAME}", module_name_title)
        elif ".astro" in filepath.suffix:
            content = TEMPLATE_ASTRO.replace("{MODULE_NAME}", module_name_title)

        content = content.replace("{module_name_lower}", module_name_snake)

        with open(filepath, "w") as f:
            f.write(content)

        print(f"   ✅ {filename}")

    print(f"\n🎉 Módulo '{module_name_snake}' creado exitosamente!")
    return 0


def main():
    print("🧩 THE-CREATOR - Generador de Boilerplate")
    print("=" * 50)

    if len(sys.argv) < 2:
        print(" Uso: python The-Creator.py <nombre-modulo> [tipo]")
        print(" Tipos: py (Python), rs (Rust), astro (Astro)")
        print(" Ejemplo: python The-Creator.py usuarios")
        print("          python The-Creator.py mi-componente astro")
        return 1

    module_name = sys.argv[1]
    module_type = sys.argv[2] if len(sys.argv) > 2 else None

    return create_module(module_name, module_type)


if __name__ == "__main__":
    sys.exit(main())
