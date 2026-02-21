#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
GIT-AUTOMATOR - Git Auto Commit/Push
Hace add, commit con mensaje (opcional con IA) y push automático.
Trigger: Manual con `just git-auto` o post-commit hook
"""

import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

import os
import subprocess
import sys
from datetime import datetime


def run_cmd(cmd, capture=True):
    try:
        if capture:
            result = subprocess.run(
                cmd,
                shell=True,
                capture_output=True,
                text=True,
                encoding="utf-8",
                errors="replace",
            )
            return result.returncode, result.stdout, result.stderr
        else:
            return os.system(cmd), "", ""
    except Exception as e:
        return 1, "", str(e)


def get_status():
    code, stdout, _ = run_cmd("git status --porcelain")
    return stdout.strip()


def get_diff_summary():
    code, stdout, _ = run_cmd("git diff --stat")
    return stdout.strip() or "Sin cambios"


def generate_commit_message():
    code, stdout, _ = run_cmd("git diff --cached --name-only")
    changed_files = stdout.strip().split("\n") if stdout.strip() else []

    if not changed_files:
        return "Sin cambios"

    if len(changed_files) <= 3:
        files_str = ", ".join(
            [
                f.replace("/", ".").replace(".py", "").replace(".rs", "")
                for f in changed_files
            ]
        )
        msg = f"Update: {files_str}"
    else:
        msg = f"Update {len(changed_files)} files"

    return msg


def get_branch():
    code, stdout, _ = run_cmd("git branch --show-current")
    return stdout.strip() or "main"


def git_add():
    print("[INFO] Ejecutando git add...")
    code, stdout, stderr = run_cmd("git add -A")
    if code == 0:
        print("   [OK] Archivos preparados")
    else:
        print(f"   [ERROR] {stderr}")
        return False
    return True


def git_commit(message=None):
    if message is None:
        message = generate_commit_message()

    print(f"[INFO] Commit: {message}")
    code, stdout, stderr = run_cmd(f'git commit -m "{message}"')
    stderr = stderr or ""

    if code == 0:
        print("   [OK] Commit creado")
        return True
    elif "nothing to commit" in stderr.lower():
        print("   [INFO] No hay cambios para commit")
        return None
    else:
        print(f"   [ERROR] {stderr}")
        return False


def git_push(branch=None):
    if branch is None:
        branch = get_branch()

    print(f"[INFO] Push to {branch}...")
    code, stdout, stderr = run_cmd(f"git push origin {branch}")

    if code == 0:
        print("   [OK] Push completado")
        return True
    else:
        print(f"   [ERROR] {stderr}")
        return False


def git_auto(message=None, push=True):
    print("[GIT-AUTOMATOR] Sintonía 3026")
    print("=" * 50)

    status = get_status()
    if not status:
        print("[INFO] No hay cambios para procesar")
        return 0

    print(f"[INFO] Cambios detectados:\n{status[:500]}")
    print()

    if not git_add():
        return 1

    result = git_commit(message)
    if result is False:
        return 1
    elif result is None:
        return 0

    if push:
        if not git_push():
            return 1

    print()
    print("[OK] Git Automator completado!")
    return 0


def main():
    import argparse

    parser = argparse.ArgumentParser(description="Git Automator - Auto commit/push")
    parser.add_argument("-m", "--message", help="Mensaje de commit")
    parser.add_argument("-p", "--no-push", action="store_true", help="No hacer push")
    parser.add_argument(
        "-a", "--amend", action="store_true", help="Enmendar último commit"
    )

    args = parser.parse_args()

    if args.amend:
        code, _, _ = run_cmd("git commit --amend --no-edit")
        if code == 0:
            print("[OK] Commit enmendado")
            if not args.no_push:
                run_cmd("git push --force")
        return code

    push = not args.no_push
    return git_auto(args.message, push)


if __name__ == "__main__":
    sys.exit(main())
