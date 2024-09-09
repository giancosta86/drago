import { fileURLToPath, URL } from "node:url";
import { defineConfig } from "vite";

export default defineConfig({
  resolve: {
    alias: {
      "@giancosta86/drago": fileURLToPath(
        new URL("../pkg/drago.js", import.meta.url)
      )
    }
  }
});
