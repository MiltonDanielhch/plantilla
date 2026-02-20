# l "Watchdog" de Sintonía (Script de Ejecución Automática)
# Este es mi favorito para enseñar y aprender. Es un script que se queda "escuchando" tus archivos. En cuanto guardas un cambio, él ejecuta automáticamente tus tests o tu auditoría.

# Python
# import os
# import time
# from watchdog.observers import Observer
# from watchdog.events import FileSystemEventHandler

# class MyHandler(FileSystemEventHandler):
#     def on_modified(self, event):
#         if event.src_path.endswith(".py"): # O la extensión de tu proyecto
#             print(f"🔄 Cambio detectado en {event.src_path}. Ejecutando Auditoría...")
#             os.system("python script_auditoria.py") # Llama a tu script anterior

# observer = Observer()
# observer.schedule(MyHandler(), path='.', recursive=True)
# observer.start()

# try:
#     while True:
#         time.sleep(1)
# except KeyboardInterrupt:
#     observer.stop()
# observer.join()