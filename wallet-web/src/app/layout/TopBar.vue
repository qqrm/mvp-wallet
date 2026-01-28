<script setup lang="ts">
import { computed } from "vue"
import { UButton, UInput, USelect, USpace } from "@uzum-tech/ui"
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
      <div class="title-row">
        <div class="app-title">Wallet</div>
        <span class="env-pill" title="Frontend demo">Demo</span>
      </div>
    </div>

    <USpace align="center" :size="12" wrap class="topbar-actions">
      <UInput
        v-model:value="settings.apiBaseUrl"
        placeholder="API base URL"
        clearable
        :input-props="apiInputProps"
        class="api-input"
      />
      <UButton size="small" @click="settings.resetApiBaseUrl">Reset</UButton>
      <USelect
        data-testid="theme-switch"
        :options="themeOptions"
        :value="settings.themeMode"
        @update:value="handleThemeUpdate"
        placeholder="Theme"
        class="theme-select"
      />
    </USpace>
  </div>
</template>

<style scoped>
.topbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 18px;
}

.topbar-left {
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.title-row {
  display: flex;
  align-items: center;
  gap: 10px;
  min-width: 0;
}

.app-title {
  font-size: 14px;
  font-weight: 800;
  letter-spacing: -0.01em;
}


.env-pill {
  display: inline-flex;
  align-items: center;
  height: 22px;
  padding: 0 10px;
  border-radius: 999px;
  font-size: 12px;
  font-weight: 700;
  color: rgba(255, 255, 255, 0.95);
  background: var(--brand-primary);
  border: 1px solid rgba(255, 255, 255, 0.20);
  flex: 0 0 auto;
}


.topbar-actions {
  align-items: center;
}

.api-input {
  width: 320px;
}

@media (max-width: 920px) {
  .api-input {
    width: 240px;
  }
}

@media (max-width: 720px) {
  .topbar {
    flex-direction: column;
    align-items: stretch;
  }

  .api-input {
    width: 100%;
  }
}
</style>
