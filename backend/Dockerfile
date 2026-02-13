# Etapa 1: Builder (Compilación)
FROM rust:1-slim-bookworm as builder

WORKDIR /app

# Instalar dependencias de compilación
RUN rm -rf /var/lib/apt/lists/* && apt-get update && apt-get install -y pkg-config libssl-dev

# Cache de dependencias: Copiamos manifiestos primero
COPY Cargo.toml Cargo.lock ./

# Creamos un main dummy para compilar dependencias y cachearlas
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copiamos el código fuente real
COPY . .

# Habilitamos modo offline para SQLx (requiere sqlx-data.json)
ENV SQLX_OFFLINE=true

# Forzamos recompilación del binario real
RUN touch src/main.rs
RUN cargo build --release

# Etapa 2: Runtime (Imagen ligera para producción)
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libssl3 ca-certificates sqlite3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/backend ./backend
ENV PORT=3000
EXPOSE 3000
CMD ["./backend"]