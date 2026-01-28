<script setup lang="ts">
import { computed, h } from "vue"
import { useRoute, useRouter, RouterLink } from "vue-router"
import { UButton, UInput, UMenu, USelect, USpace, UText, type MenuOption } from "@uzum-tech/ui"
import { useSettingsStore, type ThemeMode } from "../stores/settings"

type NavItem = {
  key: string
  label: string
  testId: string
}

const navItems: NavItem[] = [
  { key: "/", label: "Dashboard", testId: "sidebar-link-dashboard" },
  { key: "/wallet", label: "Wallet", testId: "sidebar-link-wallet" },
  { key: "/receipt", label: "Receipt", testId: "sidebar-link-receipt" },
  { key: "/admin", label: "Admin", testId: "sidebar-link-admin" },
]

const menuOptions: MenuOption[] = navItems.map((item) => ({
  key: item.key,
  label: item.label,
}))

const testIdByKey = new Map(navItems.map((item) => [item.key, item.testId]))

const route = useRoute()
const router = useRouter()

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

const activeKey = computed(() => route.path)

const renderLabel = (option: MenuOption) => {
  const key = option.key as string
  const isActive = key === activeKey.value

  return h(
    RouterLink,
    {
      to: key,
      class: ["nav-link", isActive ? "nav-link--active" : ""],
      "data-testid": testIdByKey.get(key),
    },
    {
      default: () => [
        h("span", { class: ["nav-dot", isActive ? "nav-dot--active" : ""] }),
        h("span", { class: "nav-text" }, option.label as string),
      ],
    },
  )
}

const handleUpdate = (value: string | number) => {
  router.push(String(value))
}
</script>

<template>
  <div class="sidebar">
    <div class="brand">
      <div class="brand-mark" aria-hidden="true">U</div>
      <div class="brand-text">
        <div class="brand-title-row">
          <div class="brand-title">Uzum Wallet</div>
          <span class="env-pill" title="Frontend demo">Demo</span>
        </div>
        <UText depth="3" class="brand-subtitle">Web console</UText>
        <div class="brand-api">API: <span class="mono">{{ settings.apiBaseUrl }}</span></div>
      </div>
    </div>

    <UMenu
      class="nav-menu"
      :options="menuOptions"
      :render-label="renderLabel"
      :value="activeKey"
      @update:value="handleUpdate"
    />

    <div class="system">
      <div class="system-title">System</div>
      <USpace vertical :size="10">
        <UInput
          v-model:value="settings.apiBaseUrl"
          placeholder="API base URL"
          clearable
          :input-props="apiInputProps"
        />
        <div class="system-row">
          <UButton size="small" @click="settings.resetApiBaseUrl">Reset</UButton>
          <USelect
            data-testid="theme-switch"
            :options="themeOptions"
            :value="settings.themeMode"
            @update:value="handleThemeUpdate"
            placeholder="Theme"
            class="theme-select"
          />
        </div>
      </USpace>
    </div>

    <div class="sidebar-footer">
      <div class="footer-chip">
        <span class="footer-dot" />
        <span class="footer-text">os-theme enabled</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.sidebar {
  display: flex;
  flex-direction: column;
  height: 100%;
  gap: 18px;
}

.brand {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 10px 14px;
  border-radius: var(--radius);
  background: var(--surface);
  border: 1px solid var(--border);
  box-shadow: var(--shadow-sm);
}

.brand-mark {
  position: relative;
  display: grid;
  place-items: center;
  width: 38px;
  height: 38px;
  border-radius: 12px;
  font-weight: 800;
  letter-spacing: -0.02em;
  color: #fff;
  background: var(--brand-primary);
  box-shadow: 0 14px 28px rgba(112, 0, 255, 0.18);
}

.brand-mark::after {
  content: "";
  position: absolute;
  right: -2px;
  bottom: -2px;
  width: 10px;
  height: 10px;
  border-radius: 999px;
  background: var(--accent-coral);
  box-shadow: 0 0 0 3px var(--surface);
}

.brand-text {
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-width: 0;
}

.brand-title-row {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 0;
}

.brand-title {
  font-size: 14px;
  font-weight: 700;
  line-height: 1.2;
}

.brand-subtitle {
  font-size: 12px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.brand-api {
  font-size: 12px;
  color: var(--text-muted);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.env-pill {
  display: inline-flex;
  align-items: center;
  height: 20px;
  padding: 0 10px;
  border-radius: 999px;
  font-size: 11px;
  font-weight: 800;
  color: rgba(255, 255, 255, 0.95);
  background: var(--brand-primary);
  border: 1px solid rgba(255, 255, 255, 0.2);
  flex: 0 0 auto;
}

.mono {
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  font-weight: 600;
}

.nav-menu {
  flex: 1;
}

.system {
  padding: 12px;
  border-radius: var(--radius);
  background: var(--surface);
  border: 1px solid var(--border);
  box-shadow: var(--shadow-sm);
}

.system-title {
  font-size: 12px;
  font-weight: 800;
  color: var(--text-muted);
  margin-bottom: 10px;
}

.system-row {
  display: flex;
  align-items: center;
  gap: 10px;
}

.theme-select {
  flex: 1;
  min-width: 0;
}

.nav-link {
  position: relative;
  display: inline-flex;
  align-items: center;
  gap: 10px;
  width: 100%;
  padding: 10px 10px;
  border-radius: 12px;
  text-decoration: none;
  color: inherit;
}

.nav-link:hover {
  background: rgba(112, 0, 255, 0.05);
}

.nav-link--active {
  background: rgba(112, 0, 255, 0.10);
}

.nav-dot {
  width: 8px;
  height: 8px;
  border-radius: 999px;
  background: rgba(148, 163, 184, 0.65);
}

.nav-dot--active {
  background: var(--accent-coral);
  box-shadow: 0 0 0 4px rgba(238, 78, 1, 0.16);
}

.nav-text {
  font-size: 13px;
  font-weight: 600;
}

.sidebar-footer {
  padding: 0 8px;
}

.footer-chip {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 8px 10px;
  border-radius: 999px;
  background: var(--surface);
  border: 1px solid var(--border);
}

.footer-dot {
  width: 8px;
  height: 8px;
  border-radius: 999px;
  background: var(--brand-primary);
}

.footer-text {
  font-size: 12px;
  color: var(--text-muted);
}
</style>
