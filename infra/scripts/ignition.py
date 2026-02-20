#!/usr/bin/env python3
"""
IGNITION - Inicio de Proyecto
Crea la estructura base de un nuevo proyecto con Sintonía 3026.
Trigger: Cuando creas un nuevo proyecto desde cero
"""

import os
import sys
from pathlib import Path
from datetime import datetime


def create_structure(root_name):
    project_path = Path(root_name)
    project_path.mkdir(exist_ok=True)

    print(f"🚀 IGNITION - Creando proyecto: {root_name}")
    print("=" * 50)

    (project_path / "src").mkdir(exist_ok=True)
    (project_path / "tests").mkdir(exist_ok=True)
    (project_path / "docs").mkdir(exist_ok=True)
    (project_path / "scripts").mkdir(exist_ok=True)
    (project_path / "logs").mkdir(exist_ok=True)

    gitignore = """# Dependencies
node_modules/
target/
__pycache__/
.venv/
venv/

# Build
dist/
build/

# Logs
*.log
logs/

# Environment
.env
.env.local

# IDE
.vscode/
.idea/
*.swp

# OS
.DS_Store
Thumbs.db
"""

    with open(project_path / ".gitignore", "w") as f:
        f.write(gitignore)
    print("   ✅ .gitignore")

    readme = f"""# {root_name}

## Descripción
Proyecto generado con Sintonía 3026.

## Estado
🟡 En desarrollo - {datetime.now().strftime("%Y-%m-%d")}

## Estructura
```
{root_name}/
├── src/         # Código fuente
├── tests/      # Pruebas
├── docs/        # Documentación
├── scripts/     # Scripts de automatización
└── logs/        # Archivos de log
```

## Comandos
```bash
just run-backend  # Iniciar backend
just run-frontend # Iniciar frontend
just check        # Verificar código
```
"""

    with open(project_path / "README.md", "w") as f:
        f.write(readme)
    print("   ✅ README.md")

    print("\n🎉 Proyecto creado exitosamente!")
    print(f"\n💡 Próximo paso: cd {root_name} && git init")

    return 0


def main():
    print("🚀 IGNITION - Inicio de Proyecto")
    print("=" * 50)

    if len(sys.argv) < 2:
        print(" Uso: python ignition.py <nombre-proyecto>")
        print(" Ejemplo: python ignition.py mi-proyecto")
        return 1

    project_name = sys.argv[1]
    return create_structure(project_name)


if __name__ == "__main__":
    sys.exit(main())
