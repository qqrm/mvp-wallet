import { defineStore } from "pinia"
import { computed, ref } from "vue"

type AuthUser = {
  id: string
  role: "admin" | "user"
}

export const useAuthStore = defineStore("auth", () => {
  const user = ref<AuthUser | null>(null)

  const isAuthenticated = computed(() => Boolean(user.value))

  const setUser = (nextUser: AuthUser | null) => {
    user.value = nextUser
  }

  return {
    user,
    isAuthenticated,
    setUser,
  }
})
