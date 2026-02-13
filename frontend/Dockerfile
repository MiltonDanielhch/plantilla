# Etapa 1: Construcción (Node.js)
FROM node:lts-alpine as builder
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm ci
COPY . .
RUN npm run build

# Etapa 2: Servidor Web (Nginx)
FROM nginx:alpine
# Copiamos la configuración por defecto de Nginx (opcional, usamos la default por ahora)
# Copiamos los archivos estáticos generados por Astro (carpeta dist)
COPY --from=builder /app/dist /usr/share/nginx/html

# Exponemos el puerto 80 (estándar web)
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]