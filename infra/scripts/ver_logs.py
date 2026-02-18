import sqlite3

def ver_auditoria():
    try:
        conn = sqlite3.connect('backend.db')
        cursor = conn.cursor()
        
        cursor.execute("SELECT * FROM audit_logs ORDER BY id DESC")
        logs = cursor.fetchall()
        
        print(f"\n📋 Auditoría del Sistema ({len(logs)} registros encontrados):")
        print("-" * 70)
        print(f"{'ID':<4} | {'ADMIN':<15} | {'ACCIÓN':<12} | {'OBJETIVO':<15} | {'FECHA'}")
        print("-" * 70)
        for log in logs:
            # Estructura: id, admin_username, action, target, timestamp
            print(f"{log[0]:<4} | {log[1]:<15} | {log[2]:<12} | {log[3]:<15} | {log[4]}")
        print("-" * 70)
        
        conn.close()
    except Exception as e:
        print(f"❌ Error leyendo base de datos: {e}")
        print("Nota: Asegúrate de que la migración 0004 se haya ejecutado.")

if __name__ == "__main__":
    ver_auditoria()