import sqlite3

# Configuración
DB_PATH = 'backend.db'
TARGET_USER = 'milton'  # <--- Asegúrate que este es tu usuario

try:
    # Conectar
    conn = sqlite3.connect(DB_PATH)
    cursor = conn.cursor()
    
    # Ejecutar ascenso
    print(f"Buscando a {TARGET_USER}...")
    cursor.execute("UPDATE users SET role = 'admin' WHERE username = ?", (TARGET_USER,))
    
    if cursor.rowcount > 0:
        conn.commit()
        print(f"✅ ÉXITO: El usuario '{TARGET_USER}' ahora es ADMIN.")
    else:
        print(f"❌ ERROR: No se encontró el usuario '{TARGET_USER}'. Revisa el nombre.")
        
    conn.close()

except Exception as e:
    print(f"Error crítico: {e}")
