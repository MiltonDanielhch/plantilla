-- Crear tabla de Roles
CREATE TABLE roles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Crear tabla de Permisos
CREATE TABLE permissions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE, -- ej: 'users:read', 'users:delete'
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Tabla pivote (Many-to-Many) Roles <-> Permisos
CREATE TABLE role_permissions (
    role_id INTEGER NOT NULL,
    permission_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (role_id, permission_id),
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE,
    FOREIGN KEY (permission_id) REFERENCES permissions(id) ON DELETE CASCADE
);

-- Seed: Roles Iniciales (Compatibilidad con Enum actual)
INSERT INTO roles (name, description) VALUES ('Admin', 'Administrador con acceso total al sistema');
INSERT INTO roles (name, description) VALUES ('User', 'Usuario estándar con acceso limitado');

-- Seed: Permisos Básicos
INSERT INTO permissions (name, description) VALUES ('users:read', 'Ver lista y detalles de usuarios');
INSERT INTO permissions (name, description) VALUES ('users:write', 'Crear y editar usuarios');
INSERT INTO permissions (name, description) VALUES ('users:delete', 'Eliminar usuarios');
INSERT INTO permissions (name, description) VALUES ('audit:read', 'Ver logs de auditoría');
INSERT INTO permissions (name, description) VALUES ('audit:export', 'Exportar datos del sistema');

-- Asignar TODOS los permisos al Admin
INSERT INTO role_permissions (role_id, permission_id) 
SELECT r.id, p.id FROM roles r, permissions p WHERE r.name = 'Admin';

-- Asignar permisos básicos al User
INSERT INTO role_permissions (role_id, permission_id) 
SELECT r.id, p.id FROM roles r, permissions p WHERE r.name = 'User' AND p.name IN ('users:read');