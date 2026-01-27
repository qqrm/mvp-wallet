import { defineConfig } from "vite"
import vue from "@vitejs/plugin-vue"

// Dev proxy:
// - browser calls the Vite dev server (localhost:5173)
// - Vite proxies /v1/* to the API container (api:3000)
// This avoids CORS and keeps `VITE_API_BASE` empty.
export default defineConfig({
  plugins: [vue()],
  server: {
    host: true,
    port: 5173,
    proxy: {
      "/v1": {
        target: "http://api:3000",
        changeOrigin: true,
      },
      "/swagger-ui": {
        target: "http://api:3000",
        changeOrigin: true,
      },
      "/api-doc": {
        target: "http://api:3000",
        changeOrigin: true,
      },
      "/health": {
        target: "http://api:3000",
        changeOrigin: true,
      },
    },
  },
})
