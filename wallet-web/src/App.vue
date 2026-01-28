<script setup lang="ts">
import { computed, watch } from "vue"
import {
  UConfigProvider,
  UGlobalStyle,
  UMessageProvider,
  ULoadingBarProvider,
  darkTheme,
  lightTheme,
  useOsTheme,
} from "@uzum-tech/ui"
import AppLayout from "./app/layout/AppLayout.vue"
import { useSettingsStore } from "./app/stores/settings"

const settings = useSettingsStore()
const osThemeRef = useOsTheme()

const resolvedThemeMode = computed(() => {
  if (settings.themeMode === "dark") return "dark"
  if (settings.themeMode === "light") return "light"
  return osThemeRef.value === "dark" ? "dark" : "light"
})

// Uzum UI os-theme: https://uzum-ui.kapitalbank.uz/en-US/os-theme
const resolvedTheme = computed(() => (resolvedThemeMode.value === "dark" ? darkTheme : lightTheme))

watch(
  resolvedThemeMode,
  (mode) => {
    if (typeof document === "undefined") return
    document.documentElement.setAttribute("data-theme", mode)
  },
  { immediate: true },
)
</script>

<template>
  <UConfigProvider :theme="resolvedTheme">
    <UMessageProvider>
      <ULoadingBarProvider>
        <UGlobalStyle />
        <AppLayout />
      </ULoadingBarProvider>
    </UMessageProvider>
  </UConfigProvider>
</template>
