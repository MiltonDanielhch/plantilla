#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
MASTER_ORCHESTRATOR - Cerebro Central de Sintonía 3026
Coordina todos los scripts según el estado del sistema.
Trigger: Ejecución manual o cron (cada 5 minutos)
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import subprocess
import json
import requests
from pathlib import Path
from datetime import datetime, timedelta
from collections import defaultdict


class MasterOrchestrator:
    def __init__(self):
        self.config = self.load_config()
        self.state_file = Path("logs/orchestrator_state.json")
        self.state = self.load_state()

    def load_config(self):
        return {
            "health_urls": os.environ.get("HEALTH_URLS", "http://localhost:3000").split(
                ","
            ),
            "disk_warning": 85,
            "disk_critical": 95,
            "memory_warning": 80,
            "memory_critical": 90,
            "auto_recover": os.environ.get("AUTO_RECOVER", "true").lower() == "true",
        }

    def load_state(self):
        if self.state_file.exists():
            with open(self.state_file, "r") as f:
                return json.load(f)
        return {"last_check": None, "issues": [], "recoveries": []}

    def save_state(self):
        self.state_file.parent.mkdir(parents=True, exist_ok=True)
        with open(self.state_file, "w") as f:
            json.dump(self.state, f, indent=2)

    def run_script(self, script_path, description):
        print(f"  [RUN] {description}...")
        try:
            result = subprocess.run(
                ["python", script_path]
                if script_path.endswith(".py")
                else ["bash", script_path],
                capture_output=True,
                text=True,
                timeout=300,
            )
            return result.returncode == 0, result.stdout
        except Exception as e:
            return False, str(e)

    def check_health(self):
        print("[HEALTH] Verificando servicios...")
        issues = []

        for url in self.config["health_urls"]:
            try:
                response = requests.get(url.strip() + "/health", timeout=5)
                if response.status_code >= 500:
                    issues.append(f"Servicio caido: {url}")
            except:
                issues.append(f"Sin conexion: {url}")

        return issues

    def check_system_resources(self):
        print("[RESOURCES] Verificando recursos...")
        import psutil

        issues = []

        disk = psutil.disk_usage("/")
        if disk.percent > self.config["disk_critical"]:
            issues.append(f"Disco critico: {disk.percent}%")
        elif disk.percent > self.config["disk_warning"]:
            issues.append(f"Disco bajo: {disk.percent}%")

        memory = psutil.virtual_memory()
        if memory.percent > self.config["memory_critical"]:
            issues.append(f"Memoria critica: {memory.percent}%")
        elif memory.percent > self.config["memory_warning"]:
            issues.append(f"Memoria baja: {memory.percent}%")

        return issues

    def check_secrets(self):
        print("[SECURITY] Verificando secretos...")
        success, output = self.run_script(
            "scripts/automatizacion/seguridad/Shield.py", "Shield - Buscar secretos"
        )
        if not success or "ALERTA" in output:
            return ["Secretos expuestos detectados"]
        return []

    def check_dependencies(self):
        print("[DEPS] Verificando dependencias...")
        success, output = self.run_script(
            "scripts/automatizacion/auditoria/Ghost-Inspector.py",
            "Ghost Hunter - Dependencias no usadas",
        )
        if not success:
            return ["Problemas con dependencias"]
        return []

    def auto_recover(self, issue):
        print(f"[RECOVER] Intentando recuperar: {issue}")

        if "Memoria" in issue or "caido" in issue.lower():
            self.run_script(
                "scripts/automatizacion/monitoreo/vigilante.sh",
                "Vigilante - Reiniciar app",
            )

        if "Disco" in issue:
            self.run_script(
                "scripts/automatizacion/utilidades/limpieza.sh",
                "Limpieza - Liberar espacio",
            )

        if "secreto" in issue.lower():
            self.notify_admin(f"ALERTA SEGURIDAD: {issue}")

    def notify_admin(self, message):
        print(f"[ALERT] {message}")

        if "TELEGRAM_BOT_TOKEN" in os.environ and "TELEGRAM_CHAT_ID" in os.environ:
            try:
                requests.post(
                    f"https://api.telegram.org/bot{os.environ['TELEGRAM_BOT_TOKEN']}/sendMessage",
                    json={
                        "chat_id": os.environ["TELEGRAM_CHAT_ID"],
                        "text": f"🤖 *SINTONÍA 3026*\n\n{message}",
                        "parse_mode": "Markdown",
                    },
                )
            except:
                pass

    def run_full_audit(self):
        print("\n[AUDIT] Ejecutando auditoría completa...")

        all_issues = []

        all_issues.extend(self.check_health())
        all_issues.extend(self.check_system_resources())
        all_issues.extend(self.check_secrets())
        all_issues.extend(self.check_dependencies())

        self.state["last_check"] = datetime.now().isoformat()
        self.state["issues"] = all_issues
        self.save_state()

        return all_issues

    def deploy_pipeline(self):
        print("\n[DEPLOY] Pipeline de despliegue...")

        print("  1. Ghost Hunter - Limpiar dependencias")
        self.run_script(
            "scripts/automatizacion/auditoria/Ghost-Inspector.py", "Limpiar deps"
        )

        print("  2. Shield - Verificar secretos")
        success, _ = self.run_script(
            "scripts/automatizacion/seguridad/Shield.py", "Verificar secretos"
        )
        if not success:
            print("  [STOP] Secretos detectados. Abortando deploy.")
            return False

        print("  3. Ejecutar deploy")
        self.run_script(
            "scripts/automatizacion/despliegue/deploy_maestro.sh", "Desplegar"
        )

        print("  4. Verificar salud post-deploy")
        issues = self.check_health()
        if issues:
            print(f"  [WARN] Problemas post-deploy: {issues}")
            self.notify_admin(f"Deploy completado con warnings: {issues}")
        else:
            self.notify_admin("Deploy completado exitosamente!")

        return True

    def dashboard(self):
        print("\n" + "=" * 50)
        print("📊 DASHBOARD SINTONÍA 3026")
        print("=" * 50)
        print(f"Última verificación: {self.state.get('last_check', 'Nunca')}")
        print(f"Issues activos: {len(self.state.get('issues', []))}")

        if self.state.get("issues"):
            print("\n⚠️ Issues:")
            for issue in self.state["issues"]:
                print(f"  - {issue}")

        print("\n📈 Recursos:")
        try:
            import psutil

            print(f"  CPU: {psutil.cpu_percent()}%")
            print(f"  Memoria: {psutil.virtual_memory().percent}%")
            print(f"  Disco: {psutil.disk_usage('/').percent}%")
        except:
            print("  (No disponible)")

        print("=" * 50)


def main():
    import argparse

    parser = argparse.ArgumentParser(description="Master Orchestrator - Sintonía 3026")
    parser.add_argument("--check", action="store_true", help="Verificación completa")
    parser.add_argument(
        "--deploy", action="store_true", help="Ejecutar pipeline de deploy"
    )
    parser.add_argument("--dashboard", action="store_true", help="Mostrar dashboard")
    parser.add_argument("--health", action="store_true", help="Solo verificar salud")
    parser.add_argument(
        "--resources", action="store_true", help="Solo verificar recursos"
    )

    args = parser.parse_args()

    orchestrator = MasterOrchestrator()

    if args.dashboard:
        orchestrator.dashboard()
        return 0

    if args.health:
        issues = orchestrator.check_health()
        if issues:
            print(f"[ERROR] {len(issues)} problemas encontrados")
            for issue in issues:
                print(f"  - {issue}")
            return 1
        print("[OK] Todos los servicios funcionando")
        return 0

    if args.resources:
        issues = orchestrator.check_system_resources()
        if issues:
            print(f"[WARN] {len(issues)} problemas encontrados")
            for issue in issues:
                print(f"  - {issue}")
            return 1
        print("[OK] Recursos OK")
        return 0

    if args.deploy:
        success = orchestrator.deploy_pipeline()
        return 0 if success else 1

    if args.check:
        issues = orchestrator.run_full_audit()

        if issues:
            print(f"\n[ALERT] {len(issues)} issue(s) encontrado(s)")
            for issue in issues:
                print(f"  - {issue}")

            if orchestrator.config["auto_recover"]:
                for issue in issues:
                    orchestrator.auto_recover(issue)

            return 1
        else:
            print("\n[OK] Todo funcionando correctamente!")
            return 0

    orchestrator.dashboard()
    return 0


if __name__ == "__main__":
    sys.exit(main())
