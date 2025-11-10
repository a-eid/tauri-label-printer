// Ambient module declarations to help CI/type-check when `@tauri-apps/api` types are
// not installed yet (pnpm install issues). Prefer removing this file once the
// package and its types are reliably installed on CI.

declare module "@tauri-apps/api/tauri" {
  export function invoke<T = unknown>(cmd: string, args?: Record<string, unknown>): Promise<T>;
}

declare module "@tauri-apps/api" {
  export * from "@tauri-apps/api/tauri";
}
