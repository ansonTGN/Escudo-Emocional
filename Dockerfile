# ETAPA 1: Builder (Rust 1.84 para compatibilidad)
FROM rust:1.84-slim-bookworm as builder

# Instalar dependencias de compilación
# libssl-dev y pkg-config son obligatorios para reqwest
# libpoppler-glib-dev es para pdf-extract
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpoppler-glib-dev \
    libglib2.0-dev \
    libc6-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./

# Truco para cachear dependencias: Compilar un main vacío
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copiar el código real y compilar
COPY src ./src
COPY templates ./templates

# "Tocamos" el main para forzar a cargo a recompilar el código real
RUN touch src/main.rs
RUN cargo build --release

# ETAPA 2: Runtime (Imagen ligera final)
FROM debian:bookworm-slim

# Instalar dependencias de ejecución
# libssl3 y ca-certificates son para HTTPS
# libpoppler-glib8 es para leer PDFs
RUN apt-get update && apt-get install -y \
    libssl3 \
    libpoppler-glib8 \
    libglib2.0-0 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# --- CORRECCIÓN AQUÍ ---
# El binario se llama igual que en [package] name = "..." en Cargo.toml
COPY --from=builder /usr/src/app/target/release/escudo-defensa-relacional ./server
COPY --from=builder /usr/src/app/templates ./templates

# Crear carpeta tmp para subidas
RUN mkdir -p /tmp

# Exponer el puerto (informativo para Render)
EXPOSE 8080

# Arrancar renombrado a "server" para simplicidad
CMD ["./server"]