#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
AI-ASSISTANT - Asistente IA Integrado
Combina todos los analizadores IA en un asistente conversacional.
Trigger: Manual o análisis automático
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import subprocess
from pathlib import Path
from datetime import datetime

ANALYZERS = {
    "code": "scripts/automatizacion/auditoria/code-analyzer.py",
    "refactor": "scripts/automatizacion/auditoria/refactor-advisor.py",
    "performance": "scripts/automatizacion/auditoria/performance-profiler.py",
    "security": "scripts/automatizacion/seguridad/security-advisor.py",
    "shield": "scripts/automatizacion/seguridad/Shield.py",
    "ghost": "scripts/automatizacion/auditoria/Ghost-Inspector.py",
    "vuln": "scripts/automatizacion/seguridad/vulnerability-scanner.py",
}


def run_analyzer(name):
    path = ANALYZERS.get(name)
    if not path:
        return f"Analizador '{name}' no encontrado"

    if not Path(path).exists():
        return f"Archivo no encontrado: {path}"

    result = subprocess.run(
        ["python", path], capture_output=True, text=True, timeout=60
    )

    return result.stdout + result.stderr


def analyze_all():
    results = {}

    print("[AI] Ejecutando análisis completo...\n")

    for name, path in ANALYZERS.items():
        print(f"[RUN] {name}...")
        results[name] = run_analyzer(name)

    return results


def generate_report(results):
    report = []
    report.append("# INFORME DE ANÁLISIS IA")
    report.append(f"\n**Fecha:** {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    report.append("\n---\n")

    summary = []

    for name, output in results.items():
        report.append(f"## {name.upper()}")
        report.append("```")

        lines = output.split("\n")
        relevant = [
            l
            for l in lines
            if any(
                x in l.upper()
                for x in ["ISSUE", "ERROR", "WARN", "ALERT", "SCORE", "OK"]
            )
        ]

        if relevant:
            report.extend(relevant[:20])
        else:
            report.append(output[:500])

        report.append("```\n")

        if "ISSUE" in output or "ERROR" in output or "ALERT" in output:
            summary.append(name)

    if summary:
        report.insert(3, f"**⚠️ Áreas con problemas:** {', '.join(summary)}\n")
    else:
        report.insert(3, "**✅ Todo limpio**\n")

    return "\n".join(report)


def interactive_mode():
    print("\n" + "=" * 60)
    print("[AI] ASISTENTE IA - Modo Interactivo")
    print("=" * 60)
    print("\nComandos disponibles:")
    print("  analyze <tipo>  - Analizar algo específico")
    print("  analyze all     - Análisis completo")
    print("  code            - Análisis de código")
    print("  security        - Análisis de seguridad")
    print("  performance     - Análisis de rendimiento")
    print("  refactor        - Sugerencias de refactorización")
    print("  exit            - Salir")
    print()

    while True:
        try:
            cmd = input("🤖 > ").strip().lower()

            if cmd == "exit":
                print("[OK] Saliendo...")
                break

            if cmd == "analyze all":
                results = analyze_all()
                print(generate_report(results))
            elif cmd.startswith("analyze "):
                analyzer = cmd.split()[1]
                print(f"\n[RUN] Ejecutando {analyzer}...")
                print(run_analyzer(analyzer))
            elif cmd in ANALYZERS:
                print(f"\n[RUN] Ejecutando {cmd}...")
                print(run_analyzer(cmd))
            else:
                print(f"[WARN] Comando desconocido: {cmd}")

        except KeyboardInterrupt:
            print("\n[OK] Saliendo...")
            break
        except Exception as e:
            print(f"[ERROR] {e}")


def main():
    import argparse

    parser = argparse.ArgumentParser(description="AI Assistant")
    parser.add_argument(
        "--analyze",
        metavar="TIPO",
        help="Tipo de análisis (code, security, performance, refactor, all)",
    )
    parser.add_argument("--interactive", action="store_true", help="Modo interactivo")
    parser.add_argument(
        "--report", action="store_true", help="Generar reporte completo"
    )

    args = parser.parse_args()

    print("[AI] ASISTENTE IA - Laboratorio Master 3026")
    print("=" * 60)

    if args.interactive:
        interactive_mode()
        return 0

    if args.analyze:
        if args.analyze == "all":
            results = analyze_all()
            print(generate_report(results))
        else:
            print(run_analyzer(args.analyze))
        return 0

    if args.report:
        results = analyze_all()
        report = generate_report(results)

        report_path = Path("logs/ai_report.md")
        report_path.parent.mkdir(parents=True, exist_ok=True)

        with open(report_path, "w") as f:
            f.write(report)

        print(f"\n[OK] Reporte guardado: {report_path}")
        return 0

    print("\n[INFO] Uso:")
    print("  just ai-analyze code       - Analizar código")
    print("  just ai-analyze security   - Analizar seguridad")
    print("  just ai-analyze all       - Análisis completo")
    print("  just ai-report           - Generar reporte")
    print("  just ai-interactive       - Modo interactivo")

    return 0


if __name__ == "__main__":
    sys.exit(main())
