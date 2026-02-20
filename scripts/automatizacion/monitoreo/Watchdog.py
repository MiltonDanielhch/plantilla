#!/usr/bin/env python3
"""
WATCHDOG - Observador de Archivos
Ejecuta acciones automáticamente cuando se modifican archivos.
Trigger: Ejecución manual con `just monitor` (se queda escuchando)
"""

import os
import sys
import time
from pathlib import Path

try:
    from watchdog.observers import Observer
    from watchdog.events import FileSystemEventHandler
except ImportError:
    print("❌ Error: Se requiere 'watchdog'")
    print("   Instala con: pip install watchdog")
    sys.exit(1)


class SintoniaHandler(FileSystemEventHandler):
    def __init__(self, extensions=None, exclude_dirs=None):
        self.extensions = extensions or [".py", ".js", ".ts", ".rs", ".astro"]
        self.exclude_dirs = exclude_dirs or {
            ".git",
            "node_modules",
            "target",
            "__pycache__",
            ".venv",
        }
        self.last_action_time = {}
        self.cooldown = 5

    def should_process(self, path):
        path_obj = Path(path)

        if any(ex in path_obj.parts for ex in self.exclude_dirs):
            return False

        if path_obj.is_dir():
            return False

        return path_obj.suffix in self.extensions

    def on_modified(self, event):
        if self.should_process(event.src_path):
            self.trigger_action(event.src_path, "modificado")

    def on_created(self, event):
        if self.should_process(event.src_path):
            self.trigger_action(event.src_path, "creado")

    def trigger_action(self, filepath, action_type):
        current_time = time.time()

        if filepath in self.last_action_time:
            if current_time - self.last_action_time[filepath] < self.cooldown:
                return

        self.last_action_time[filepath] = current_time

        print(f"\n🔄 [{action_type.upper()}] {filepath}")
        print("   ⚡ Ejecutando verificación rápida...")

        os.system("just check")

        print("\n👁️ Watchdog продолжает наблюдать...")


def main():
    print("👁️ WATCHDOG - Observador de Sintonía 3026")
    print("=" * 50)
    print("🎯 Extensiones observadas: .py, .js, .ts, .rs, .astro")
    print("⏹️  Presiona Ctrl+C para detener\n")

    event_handler = SintoniaHandler()
    observer = Observer()
    observer.schedule(event_handler, path=".", recursive=True)
    observer.start()

    print("👁️ Observando cambios en tiempo real...")

    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        print("\n⏹️ Watchdog detenido.")
        observer.stop()

    observer.join()
    return 0


if __name__ == "__main__":
    sys.exit(main())
