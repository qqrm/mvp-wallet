<script setup lang="ts">
import { computed, h } from "vue"
import { useRoute, useRouter, RouterLink } from "vue-router"
import { UMenu, UText, type MenuOption } from "@uzum-tech/ui"

type NavItem = {
  key: string
  label: string
  testId: string
}

const navItems: NavItem[] = [
  { key: "/", label: "Dashboard", testId: "sidebar-link-dashboard" },
  { key: "/admin", label: "Admin", testId: "sidebar-link-admin" },
  { key: "/wallet", label: "Wallet", testId: "sidebar-link-wallet" },
  { key: "/receipt", label: "Receipt", testId: "sidebar-link-receipt" },
]

const menuOptions: MenuOption[] = navItems.map((item) => ({
  key: item.key,
  label: item.label,
}))

const testIdByKey = new Map(navItems.map((item) => [item.key, item.testId]))

const route = useRoute()
const router = useRouter()

const activeKey = computed(() => route.path)

const renderLabel = (option: MenuOption) =>
  h(
    RouterLink,
    {
      to: option.key as string,
      class: "nav-link",
      "data-testid": testIdByKey.get(option.key as string),
    },
    { default: () => option.label as string },
  )

const handleUpdate = (value: string | number) => {
  router.push(String(value))
}
</script>

<template>
  <div class="sidebar">
    <div class="brand">
      <div class="brand-mark">U</div>
      <UText strong>Uzum Wallet</UText>
    </div>
    <UMenu :options="menuOptions" :render-label="renderLabel" :value="activeKey" @update:value="handleUpdate" />
  </div>
</template>

<style scoped>
.sidebar {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.brand {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 4px;
}

.brand-mark {
  display: grid;
  place-items: center;
  width: 32px;
  height: 32px;
  border-radius: 8px;
  font-weight: 600;
}

.nav-link {
  display: inline-flex;
  align-items: center;
  width: 100%;
  color: inherit;
  text-decoration: none;
}
</style>
