import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { fileURLToPath } from "url";
import { dirname, resolve } from "path";
import { createRequire } from "module";

const host = process.env.TAURI_DEV_HOST;

const __dirname = dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);
let tauriResolved: string | undefined;
let tauriPkgResolved: string | undefined;
try {
  tauriResolved = require.resolve("@tauri-apps/api/tauri");
} catch (e) {
  // ignore, will fallback to package dir
}
try {
  tauriPkgResolved = require.resolve("@tauri-apps/api");
} catch (e) {
  // ignore
}

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
      // Resolve the tauri subpath to the actual installed file so Rollup can load it
      ...(tauriResolved ? { "@tauri-apps/api/tauri": tauriResolved } : {}),
      ...(tauriPkgResolved
        ? { "@tauri-apps/api": tauriPkgResolved }
        : { "@tauri-apps/api": resolve(__dirname, "node_modules", "@tauri-apps", "api") }),
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
