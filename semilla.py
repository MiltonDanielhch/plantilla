import os

def crear_estructura_maestra():
    """
    Genera la arquitectura de carpetas del Código 3026.
    Implementa el INFORME TÉCNICO: ARQUITECTURA DE SOFTWARE UNIVERSAL.
    """
    print("🚀 Iniciando Protocolo de Scaffolding 3026 (Sistema Semilla)...")

    # 1. Definición de la Anatomía del Sistema
    carpetas = [
        # CI/CD
        ".github/workflows",
        
        # Documentación (Cerebro)
        "docs/adr",
        "docs/api",
        "docs/database",
        
        # Infraestructura
        "infra/docker",
        "infra/scripts",
        
        # Backend (Rust/Axum) - El Núcleo
        "backend/src/api/routes",       # Endpoints
        "backend/src/api/handlers",     # Controladores
        "backend/src/core/models",      # Lógica Pura
        "backend/src/core/services",    # Casos de uso
        "backend/src/data/repositories",# Acceso a Datos
        "backend/src/shared",           # Herramientas genéricas
        "backend/tests",                # Pruebas
        
        # Frontend (Astro) - La Vitrina
        "frontend/src/components",
        "frontend/src/layouts",
        "frontend/src/pages",
    ]

    # 2. Archivos Base (Manifiesto y Configuración)
    archivos = {
        "README.md": """# Sistema Semilla 3026

## Misión
Arquitectura de software universal diseñada para liberar el potencial humano.

## Quickstart
1. `python semilla.py` (Scaffolding)
2. `cd backend && cargo run` (Backend)
3. `cd frontend && npm run dev` (Frontend)

## Reglas de Sintonía
- Código limpio y modular.
- Tests antes de deploy.
""",
        ".env.example": """# Mapa de Sintonía de Variables
DATABASE_URL=sqlite://./data/app.db
API_PORT=3000
RUST_LOG=info
""",
        "docs/ALCANCE_MVP.md": "# Alcance del Proyecto\n\n## Problema que resuelve\n...\n\n## Funcionalidades MUST\n...",
    }

    # 3. Ejecución
    for carpeta in carpetas:
        try:
            os.makedirs(carpeta, exist_ok=True)
            with open(os.path.join(carpeta, ".gitkeep"), "w") as f:
                pass
            print(f"✅ [Estructura] Creado: {carpeta}")
        except Exception as e:
            print(f"❌ Error creando {carpeta}: {e}")

    for ruta, contenido in archivos.items():
        if not os.path.exists(ruta):
            try:
                # Asegurar que el directorio exista
                dir_name = os.path.dirname(ruta)
                if dir_name:
                    os.makedirs(dir_name, exist_ok=True)
                with open(ruta, "w", encoding="utf-8") as f:
                    f.write(contenido)
                print(f"📄 [Archivo] Creado: {ruta}")
            except Exception as e:
                print(f"❌ Error creando {ruta}: {e}")

    print("\n🏁 Scaffolding completado. El organismo digital está listo.")

if __name__ == "__main__":
    crear_estructura_maestra()