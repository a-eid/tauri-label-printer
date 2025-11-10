import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { fileURLToPath } from "url";
import { dirname, resolve } from "path";

const host = process.env.TAURI_DEV_HOST;

const __dirname = dirname(fileURLToPath(import.meta.url));

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [react()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,

  // 2. ensure Vite can resolve the @tauri-apps/api package reliably under pnpm layouts
  resolve: {
    alias: {
      // point the package id to the node_modules copy so Rollup/Vite can resolve the subpath
      "@tauri-apps/api": resolve(__dirname, "node_modules", "@tauri-apps", "api"),
    },
  },

  // 3. pre-bundle these deps to avoid resolution issues in the build step
  optimizeDeps: {
    include: ["@tauri-apps/api/tauri", "@tauri-apps/api"],
  },

  // 4. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
