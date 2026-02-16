/// <reference types="astro/client" />

declare namespace App {
  interface Locals {
    user: {
      isAuthenticated: boolean;
      role: string;
      id?: string;
      username?: string;
    };
  }
}
