import { defineMiddleware } from "astro:middleware";

export const onRequest = defineMiddleware(async (context, next) => {
  // 1. Estado inicial por defecto (Guest)
  context.locals.user = {
    isAuthenticated: false,
    role: "guest",
  };

  // 2. Leer cookie. Buscamos 'session' (frontend), 'token' o 'auth_token' (backend).
  const token = 
    context.cookies.get("session")?.value || 
    context.cookies.get("token")?.value || 
    context.cookies.get("auth_token")?.value;

  if (token) {
    try {
      const parts = token.split(".");
      if (parts.length === 3) {
        const payload = parts[1];
        // Decodificación robusta Base64Url -> UTF-8
        const base64 = payload.replace(/-/g, "+").replace(/_/g, "/");
        const jsonPayload = decodeURIComponent(
          atob(base64)
            .split("")
            .map((c) => "%" + ("00" + c.charCodeAt(0).toString(16)).slice(-2))
            .join("")
        );

        const decoded = JSON.parse(jsonPayload);

        // 3. Inyectar en Locals
        context.locals.user = {
          isAuthenticated: true,
          id: decoded.user_id || decoded.sub,
          username: decoded.username || decoded.sub,
          role: decoded.role || "User",
        };
      }
    } catch (e) {
      // Si falla la decodificación, el usuario sigue como guest
      console.error("Error decoding token:", e);
    }
  }

  // 4. Protección de rutas básica (Sin bucles complejos)
  const path = context.url.pathname;
  
  // Solo protegemos /dashboard. Si no está autenticado -> Login
  if (path.startsWith("/dashboard") && !context.locals.user.isAuthenticated) {
    return context.redirect("/login");
  }

  return next();
});
