import sqlite3
import random

def seed_users():
    db_path = 'backend.db'
    print(f"🌱 Conectando a {db_path} para corregir y sembrar usuarios...")
    
    try:
        conn = sqlite3.connect(db_path)
        cursor = conn.cursor()
        
        nombres = ["Ana", "Carlos", "Beatriz", "David", "Elena", "Fernando", "Gloria", "Hector", "Irene", "Javier"]
        apellidos = ["Gomez", "Perez", "Diaz", "Ruiz", "Torres", "Vargas", "Rojas", "Mendez", "Cruz", "Flores"]
        
        # 1. Limpiar usuarios corruptos (Role 'User' que causa error 500)
        print("🧹 Limpiando usuarios con rol inválido...")
        cursor.execute("DELETE FROM users WHERE role = 'User'")
        
        count = 0
        for _ in range(30):
            username = f"{random.choice(nombres)}_{random.choice(apellidos)}_{random.randint(10, 99)}"
            # Hash dummy (no sirve para login real, solo para rellenar la tabla)
            dummy_hash = "$argon2id$v=19$m=19456,t=2,p=1$dummy$dummyhashvalue"
            
            try:
                # Usamos el valor por defecto de la DB para el rol (que es el correcto)
                cursor.execute("INSERT INTO users (username, password_hash) VALUES (?, ?)", (username, dummy_hash))
                count += 1
            except sqlite3.IntegrityError:
                pass # Si ya existe, lo saltamos

        conn.commit()
        print(f"✅ Éxito: Se han creado {count} usuarios nuevos de prueba.")
        conn.close()
        
    except Exception as e:
        print(f"❌ Error: {e}")

if __name__ == "__main__":
    seed_users()