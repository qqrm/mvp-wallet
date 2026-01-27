<script setup lang="ts">
import { computed } from "vue"
import { UButton, UInput, USelect, USpace, UText } from "@uzum-tech/ui"
import { useSettingsStore, type ThemeMode } from "../stores/settings"

const settings = useSettingsStore()

const apiInputProps = { "data-testid": "api-base-url-input" } as Record<string, string>

const themeOptions = computed(() => [
  { label: "System", value: "system" },
  { label: "Light", value: "light" },
  { label: "Dark", value: "dark" },
])

const handleThemeUpdate = (value: ThemeMode) => {
  settings.setThemeMode(value)
}
</script>

<template>
  <div class="topbar">
    <div class="topbar-left">
      <UText strong>App Shell</UText>
      <UText depth="3">Uzum UI os-theme foundation</UText>
    </div>
    <USpace align="center" :size="16" wrap>
      <UInput
        v-model:value="settings.apiBaseUrl"
        placeholder="API base URL"
        clearable
        :input-props="apiInputProps"
      />
      <UButton size="small" @click="settings.resetApiBaseUrl">Reset</UButton>
      <USelect
        data-testid="theme-switch"
        :options="themeOptions"
        :value="settings.themeMode"
        @update:value="handleThemeUpdate"
        placeholder="Theme"
      />
    </USpace>
  </div>
</template>

<style scoped>
.topbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 24px;
}

.topbar-left {
  display: flex;
  flex-direction: column;
  gap: 4px;
}
</style>
