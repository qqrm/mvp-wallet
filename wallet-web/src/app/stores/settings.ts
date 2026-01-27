import { defineStore } from "pinia"
import { ref, watch } from "vue"
import { resolveApiBaseUrl } from "../../shared/api/client"

export type ThemeMode = "system" | "light" | "dark"

type StoredSettings = {
  apiBaseUrl: string
  themeMode: ThemeMode
}

const STORAGE_KEY = "wallet-web.settings"

const loadSettings = (): StoredSettings => {
  const fallback: StoredSettings = {
    apiBaseUrl: resolveApiBaseUrl(),
    themeMode: "system",
  }

  if (typeof window === "undefined") return fallback

  try {
    const raw = window.localStorage.getItem(STORAGE_KEY)
    if (!raw) return fallback
    const parsed = JSON.parse(raw) as Partial<StoredSettings>
    return {
      apiBaseUrl: parsed.apiBaseUrl?.toString().trim() || fallback.apiBaseUrl,
      themeMode: parsed.themeMode ?? fallback.themeMode,
    }
  } catch {
    return fallback
  }
}

const persistSettings = (settings: StoredSettings) => {
  if (typeof window === "undefined") return
  window.localStorage.setItem(STORAGE_KEY, JSON.stringify(settings))
}

export const useSettingsStore = defineStore("settings", () => {
  const initial = loadSettings()
  const apiBaseUrl = ref(initial.apiBaseUrl)
  const themeMode = ref<ThemeMode>(initial.themeMode)

  watch(apiBaseUrl, (value) => {
    const trimmed = value.trim()
    if (trimmed !== value) apiBaseUrl.value = trimmed
  })

  watch(
    [apiBaseUrl, themeMode],
    () => {
      persistSettings({ apiBaseUrl: apiBaseUrl.value, themeMode: themeMode.value })
    },
    { deep: true },
  )

  const setThemeMode = (mode: ThemeMode) => {
    themeMode.value = mode
  }

  const resetApiBaseUrl = () => {
    apiBaseUrl.value = resolveApiBaseUrl()
  }

  return {
    apiBaseUrl,
    themeMode,
    setThemeMode,
    resetApiBaseUrl,
  }
})
