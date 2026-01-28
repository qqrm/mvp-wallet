<script setup lang="ts">
import { computed, onMounted, ref } from "vue"
import { useRouter } from "vue-router"
import { UCard, UGrid, UGridItem, USpace, UText } from "@uzum-tech/ui"
import CoralButton from "../shared/ui/CoralButton.vue"
import { useSettingsStore } from "../app/stores/settings"
import { type ApiError } from "../shared/api/client"
import { fetchDevUsers, type DevUserItem } from "../shared/api/endpoints"

const DEFAULT_USER_ID = "u01"

const settings = useSettingsStore()
const router = useRouter()

const users = ref<DevUserItem[]>([])
const isLoading = ref(false)
const errorMessage = ref<string | null>(null)

const headers = computed(() => new Headers({ "X-Dev-User": DEFAULT_USER_ID }))

const loadUsers = async () => {
  isLoading.value = true
  errorMessage.value = null
  try {
    const response = await fetchDevUsers({ baseUrl: settings.apiBaseUrl, headers: headers.value })
    users.value = response.users
  } catch (error) {
    errorMessage.value = (error as ApiError)?.message ?? "Unable to load users."
  } finally {
    isLoading.value = false
  }
}

const openWallet = (userId: string) => {
  void router.push({ path: "/wallet", query: { as: userId } })
}

onMounted(() => {
  void loadUsers()
})
</script>

<template>
  <div class="admin">
    <div class="admin-header">
      <div>
        <h1 class="page-title">Admin</h1>
        <p class="page-subtitle">Operational tools (demo UI only).</p>
      </div>
      <div class="admin-badge">Restricted</div>
    </div>

    <UGrid :cols="3" :x-gap="16" :y-gap="16">
      <UGridItem>
        <UCard title="Users">
          <USpace vertical :size="12">
            <UText depth="3">Select a user to open their wallet.</UText>
            <div v-if="errorMessage" class="error-text">{{ errorMessage }}</div>
            <div v-else-if="isLoading" class="muted">Loading users...</div>
            <div v-else-if="!users.length" class="muted">No users found.</div>
            <ul v-else class="user-list">
              <li v-for="user in users" :key="user.user_id" class="user-row">
                <div class="user-meta">
                  <div class="user-name">{{ user.display_name }}</div>
                  <div class="user-id">{{ user.user_id }}</div>
                </div>
                <CoralButton test-id="admin-open-wallet" @click="openWallet(user.user_id)">
                  Open wallet
                </CoralButton>
              </li>
            </ul>
          </USpace>
        </UCard>
      </UGridItem>

      <UGridItem>
        <UCard title="Accounts">
          <USpace vertical :size="12">
            <UText depth="3">Create accounts and manage limits.</UText>
            <CoralButton disabled test-id="admin-create-account">Create account</CoralButton>
          </USpace>
        </UCard>
      </UGridItem>

      <UGridItem>
        <UCard title="Topup">
          <USpace vertical :size="12">
            <UText depth="3">Backoffice topups and reversals.</UText>
            <CoralButton disabled test-id="admin-topup">Run topup</CoralButton>
          </USpace>
        </UCard>
      </UGridItem>
    </UGrid>
  </div>
</template>

<style scoped>
.admin {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.admin-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
}

.admin-badge {
  display: inline-flex;
  align-items: center;
  height: 28px;
  padding: 0 12px;
  border-radius: 999px;
  font-size: 12px;
  font-weight: 800;
  color: rgba(255, 255, 255, 0.95);
  background: rgba(239, 68, 68, 0.92);
  box-shadow: 0 14px 28px rgba(239, 68, 68, 0.18);
}

.user-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin: 0;
  padding: 0;
  list-style: none;
}

.user-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 12px;
  border-radius: 12px;
  border: 1px solid var(--border);
  background: var(--surface-2);
}

.user-meta {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.user-name {
  font-weight: 700;
}

.user-id {
  font-size: 12px;
  color: var(--muted);
}

.muted {
  color: var(--muted);
}

.error-text {
  color: #ef4444;
  font-size: 13px;
  font-weight: 600;
}

@media (max-width: 1024px) {
  .admin-header {
    flex-direction: column;
  }
}
</style>
