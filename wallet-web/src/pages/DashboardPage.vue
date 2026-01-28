<script setup lang="ts">
import { computed, onMounted, ref } from "vue"
import { RouterLink } from "vue-router"
import { UAlert, UCard, UGrid, UGridItem, USpace, UText } from "@uzum-tech/ui"
import { fetchHealth } from "../shared/api/endpoints"
import { type ApiError } from "../shared/api/client"
import { useSettingsStore } from "../app/stores/settings"
import { useSessionStore } from "../app/stores/session"
import { notifyError, notifySuccess } from "../shared/ui/notifications"

const settings = useSettingsStore()
const session = useSessionStore()

const healthStatus = ref<string | null>(null)
const healthError = ref<string | null>(null)

const isHealthy = computed(() => !healthError.value)

const configRows = computed(() => [
  { key: "API Base URL", value: settings.apiBaseUrl },
  { key: "Theme Mode", value: settings.themeMode },
])

const loadHealth = async (shouldNotify = false) => {
  session.setLoading(true)
  healthError.value = null
  try {
    const result = await fetchHealth(settings.apiBaseUrl)
    healthStatus.value = result.status
    if (shouldNotify) notifySuccess("Backend responded successfully.")
  } catch (error) {
    const apiError = error as ApiError
    const message = apiError?.message ?? "Unable to reach backend."
    healthError.value = message
    if (shouldNotify) notifyError(message)
  } finally {
    session.setLoading(false)
  }
}

onMounted(() => {
  void loadHealth()
})
</script>

<template>
  <div class="dashboard">
    <div>
      <h1 class="page-title">Dashboard</h1>
      <p class="page-subtitle">System overview and quick navigation.</p>
    </div>

    <UGrid :cols="3" :x-gap="16" :y-gap="16">
      <UGridItem>
        <UCard title="Backend status">
          <USpace vertical :size="12">
            <div class="card-actions">
              <button class="btn btn-coral" type="button" :disabled="session.isLoading" @click="() => loadHealth(true)">
                Refresh status
              </button>
            </div>

            <div class="status-row">
              <span class="dot" :class="isHealthy ? 'dot-success' : 'dot-error'" />
              <div class="status-text">
                <div class="status-title">{{ isHealthy ? "Reachable" : "Unreachable" }}</div>
                <div class="status-subtitle">
                  {{ isHealthy ? (healthStatus ?? "Checking /health...") : healthError }}
                </div>
              </div>
            </div>

            <UAlert v-if="healthError" type="error" title="Backend unreachable">
              Verify API base URL and network access.
            </UAlert>
          </USpace>
        </UCard>
      </UGridItem>

      <UGridItem>
        <UCard title="Configuration">
          <div class="kv">
            <div v-for="row in configRows" :key="row.key" class="kv-row">
              <div class="kv-key">{{ row.key }}</div>
              <div class="kv-value mono">{{ row.value }}</div>
            </div>
          </div>
          <UText depth="3" class="hint">Values are stored in localStorage.</UText>
        </UCard>
      </UGridItem>

      <UGridItem>
        <UCard title="Quick actions">
          <div class="links">
            <RouterLink class="link-card" to="/wallet">
              <div class="link-title">Wallet</div>
              <div class="link-subtitle">Balances and transfers</div>
            </RouterLink>
            <RouterLink class="link-card" to="/receipt">
              <div class="link-title">Receipt</div>
              <div class="link-subtitle">Lookup by tx_id</div>
            </RouterLink>
            <RouterLink class="link-card" to="/admin">
              <div class="link-title">Admin</div>
              <div class="link-subtitle">User and account tools</div>
            </RouterLink>
          </div>
        </UCard>
      </UGridItem>
    </UGrid>
  </div>
</template>

<style scoped>
.dashboard {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.card-actions {
  display: flex;
  justify-content: flex-end;
}

.status-row {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 12px;
  border-radius: 14px;
  border: 1px solid var(--border);
  background: var(--surface-2);
}

.status-text {
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-width: 0;
}

.status-title {
  font-size: 13px;
  font-weight: 800;
}

.status-subtitle {
  font-size: 12px;
  color: var(--text-muted);
  word-break: break-word;
}

.links {
  display: grid;
  grid-template-columns: 1fr;
  gap: 12px;
  margin-top: 8px;
}

.link-card {
  display: block;
  padding: 12px 12px;
  border-radius: 14px;
  border: 1px solid var(--border);
  background: var(--surface-2);
  text-decoration: none;
}

.link-card:hover {
  border-color: rgba(112, 0, 255, 0.22);
}

.link-title {
  font-size: 13px;
  font-weight: 800;
}

.link-subtitle {
  margin-top: 4px;
  font-size: 12px;
  color: var(--text-muted);
}

.hint {
  display: block;
  margin-top: 10px;
}
</style>
