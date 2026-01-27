import { defineStore } from "pinia"
import { ref } from "vue"

export type AlertType = "info" | "success" | "warning" | "error"

export type AppAlert = {
  type: AlertType
  message: string
}

export const useSessionStore = defineStore("session", () => {
  const isLoading = ref(false)
  const alert = ref<AppAlert | null>(null)

  const setLoading = (value: boolean) => {
    isLoading.value = value
  }

  const setAlert = (message: string, type: AlertType = "info") => {
    alert.value = { type, message }
  }

  const clearAlert = () => {
    alert.value = null
  }

  return {
    isLoading,
    alert,
    setLoading,
    setAlert,
    clearAlert,
  }
})
