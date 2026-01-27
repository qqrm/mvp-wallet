<script setup lang="ts">
import { computed, onMounted, ref } from "vue"
import { UAlert, UButton, UCard, UDataTable, UGrid, UGridItem, USpace, UText } from "@uzum-tech/ui"
import { fetchHealth } from "../shared/api/endpoints"
import { type ApiError } from "../shared/api/client"
import { useSettingsStore } from "../app/stores/settings"
import { useSessionStore } from "../app/stores/session"
import { notifyError, notifySuccess } from "../shared/ui/notifications"

const settings = useSettingsStore()
const session = useSessionStore()

const healthStatus = ref<string | null>(null)
const healthError = ref<string | null>(null)

const configRows = computed(() => [
  { key: "API Base URL", value: settings.apiBaseUrl },
  { key: "Theme Mode", value: settings.themeMode },
])

const configColumns = [
  { title: "Setting", key: "key" },
  { title: "Value", key: "value" },
]

const loadHealth = async (shouldNotify = false) => {
  session.setLoading(true)
  healthError.value = null
  try {
    const result = await fetchHealth(settings.apiBaseUrl)
    healthStatus.value = result.status
    if (shouldNotify) {
      notifySuccess("Backend responded successfully.")
    }
  } catch (error) {
    const apiError = error as ApiError
    const message = apiError?.message ?? "Unable to reach backend."
    healthError.value = message
    if (shouldNotify) {
      notifyError(message)
    }
  } finally {
    session.setLoading(false)
  }
}

onMounted(() => {
  void loadHealth()
})
</script>

<template>
  <USpace vertical :size="16">
    <UText strong>Dashboard</UText>
    <UGrid :cols="2" :x-gap="16" :y-gap="16">
      <UGridItem>
        <UCard title="Backend Status">
          <USpace vertical :size="12">
            <UAlert v-if="healthError" type="error" title="Backend unreachable">
              {{ healthError }}
            </UAlert>
            <UAlert v-else type="success" title="Backend reachable">
              {{ healthStatus ?? "Checking /health..." }}
            </UAlert>
            <UButton @click="() => loadHealth(true)">Check again</UButton>
          </USpace>
        </UCard>
      </UGridItem>
      <UGridItem>
        <UCard title="Current Configuration">
          <UDataTable :columns="configColumns" :data="configRows" />
        </UCard>
      </UGridItem>
    </UGrid>
  </USpace>
</template>
