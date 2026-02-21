#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
LEARNING-LOGGER - Registrador de Aprendizaje IA
Aprende de los patrones de código y comportamiento del usuario.
Trigger: Análisis automático
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import json
import re
from pathlib import Path
from datetime import datetime
from collections import defaultdict, Counter


class LearningLogger:
    def __init__(self):
        self.db_path = Path("logs/ai_learning.json")
        self.data = self.load_data()

    def load_data(self):
        if self.db_path.exists():
            with open(self.db_path, "r") as f:
                return json.load(f)
        return {
            "patterns": {},
            "file_stats": {},
            "code_frequency": {},
            "errors": [],
            "last_updated": None,
        }

    def save_data(self):
        self.data["last_updated"] = datetime.now().isoformat()
        self.db_path.parent.mkdir(parents=True, exist_ok=True)
        with open(self.db_path, "w") as f:
            json.dump(self.data, f, indent=2)

    def analyze_file_patterns(self):
        print("[LEARNING] Analizando patrones de código...")

        imports = Counter()
        functions = Counter()
        classes = Counter()

        for ext in ["*.py", "*.js", "*.ts"]:
            for filepath in Path(".").rglob(ext):
                if "node_modules" in str(filepath) or "venv" in str(filepath):
                    continue

                try:
                    with open(filepath, "r", encoding="utf-8") as f:
                        content = f.read()

                    imp_matches = re.findall(
                        r"from\s+(\w+)\s+import|import\s+(\w+)", content
                    )
                    for m in imp_matches:
                        imp = m[0] or m[1]
                        imports[imp] += 1

                    func_matches = re.findall(r"def\s+(\w+)", content)
                    for f in func_matches:
                        functions[f] += 1

                    class_matches = re.findall(r"class\s+(\w+)", content)
                    for c in class_matches:
                        classes[c] += 1

                except:
                    pass

        self.data["patterns"]["top_imports"] = imports.most_common(20)
        self.data["patterns"]["top_functions"] = functions.most_common(20)
        self.data["patterns"]["top_classes"] = classes.most_common(20)

        return imports, functions, classes

    def analyze_errors(self):
        print("[LEARNING] Aprendiendo de errores...")

        log_files = list(Path("logs").glob("*.log"))

        error_patterns = Counter()

        for log_file in log_files:
            try:
                with open(log_file, "r", encoding="utf-8", errors="ignore") as f:
                    for line in f:
                        if "ERROR" in line or "Exception" in line:
                            words = line.split()
                            for word in words:
                                if len(word) > 5:
                                    error_patterns[word] += 1
            except:
                pass

        self.data["patterns"]["common_errors"] = error_patterns.most_common(10)

    def generate_insights(self):
        insights = []

        top_imports = dict(self.data["patterns"].get("top_imports", []))
        if top_imports:
            insights.append(
                f"Imports más usados: {', '.join(list(top_imports.keys())[:5])}"
            )

        top_funcs = dict(self.data["patterns"].get("top_functions", []))
        common_funcs = [
            f
            for f, c in sorted(top_funcs.items(), key=lambda x: x[1], reverse=True)[:5]
        ]
        if common_funcs:
            insights.append(f"FUNCIONES mas comunes: {', '.join(common_funcs)}")

        return insights

    def learn_from_user(self):
        print("[LEARNING] Aprendiendo del usuario...")

        git_log = os.popen("git log --oneline -20 2>/dev/null").read()

        commit_types = Counter()
        for line in git_log.split("\n"):
            if line:
                words = line.split()
                if len(words) > 1:
                    commit_types[
                        words[1] if words[0].startswith("[") else "update"
                    ] += 1

        self.data["patterns"]["commit_style"] = dict(commit_types.most_common(10))

    def dashboard(self):
        print("\n" + "=" * 60)
        print("[AI] DASHBOARD DE APRENDIZAJE")
        print("=" * 60)

        print(
            f"\n[INFO] Última actualización: {self.data.get('last_updated', 'Nunca')}"
        )

        insights = self.generate_insights()
        if insights:
            print("\n[💡 INSIGHTS APRENDIDOS]")
            for insight in insights:
                print(f"  • {insight}")

        print("\n[📊 PATRONES DETECTADOS]")

        top_imports = self.data["patterns"].get("top_imports", [])
        if top_imports:
            print("  Top Imports:")
            for imp, count in top_imports[:5]:
                print(f"    - {imp}: {count}")

        top_funcs = self.data["patterns"].get("top_functions", [])
        if top_funcs:
            print("  Top Funciones:")
            for func, count in top_funcs[:5]:
                print(f"    - {func}(): {count}")

        print("\n" + "=" * 60)

    def full_learn(self):
        self.analyze_file_patterns()
        self.analyze_errors()
        self.learn_from_user()
        self.save_data()
        self.dashboard()


import re


def main():
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--learn", action="store_true", help="Ejecutar aprendizaje")
    parser.add_argument("--dashboard", action="store_true", help="Mostrar dashboard")
    args = parser.parse_args()

    print("[AI] LEARNING-LOGGER - Registrador de Aprendizaje IA")
    print("=" * 60)

    logger = LearningLogger()

    if args.learn:
        logger.full_learn()
        print("\n[OK] Aprendizaje completado")
    elif args.dashboard:
        logger.dashboard()
    else:
        logger.dashboard()

    return 0


if __name__ == "__main__":
    sys.exit(main())
