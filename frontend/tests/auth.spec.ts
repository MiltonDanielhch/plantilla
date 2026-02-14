import { test, expect } from '@playwright/test';

const BASE_URL = 'http://localhost:4321';

test.describe('Flujo de Autenticación Sintonía 3026', () => {
  
  // Generamos un usuario aleatorio para no chocar con la DB en cada ejecución
  const randomId = Math.floor(Math.random() * 10000);
  const username = `test_user_${randomId}`;
  const password = 'passwordSeguro123';

  test('Debe permitir registro, login y acceso al dashboard', async ({ page }) => {
    
    // 1. Ir a la página de Registro
    await page.goto(`${BASE_URL}/register`);
    await expect(page).toHaveURL(/.*register/);

    // 2. Llenar formulario de Registro
    await page.fill('input[name="username"]', username);
    await page.fill('input[name="password"]', password);
    
    // 3. Enviar y esperar redirección al Login
    await page.click('button[type="submit"]');
    
    // Esperamos llegar a la URL de login (ajusta según tu lógica de redirección)
    await page.waitForURL(/\/login\/?$/);
    
    // 4. Llenar formulario de Login
    await page.fill('input[name="username"]', username);
    await page.fill('input[name="password"]', password);
    
    // 5. Enviar Login
    await page.click('button[type="submit"]');

    // 6. Validar acceso al Dashboard
    await page.waitForURL(/\/dashboard\/?$/);
    
    // Verificar que aparece el mensaje de bienvenida o el username
    // Buscamos texto que contenga el nombre de usuario
    await expect(page.locator('body')).toContainText(username);
    
    // 7. Probar Logout
    await page.click('button#logout-btn'); // Asegúrate de que tu botón de logout tenga este ID o usa text="Cerrar Sesión"
    
    // Verificar vuelta al login
    await page.waitForURL(/\/login\/?$/);
  });
});