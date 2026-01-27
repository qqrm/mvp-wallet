<script setup lang="ts">
import { computed } from "vue"
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

// Uzum UI os-theme: https://uzum-ui.kapitalbank.uz/en-US/os-theme (providers + system theme via useOsTheme).
const resolvedTheme = computed(() => {
  if (settings.themeMode === "dark") return darkTheme
  if (settings.themeMode === "light") return lightTheme
  return osThemeRef.value === "dark" ? darkTheme : lightTheme
})
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
