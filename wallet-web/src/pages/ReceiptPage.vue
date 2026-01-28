<script setup lang="ts">
import { computed, ref } from "vue"
import { UCard, UInput, USpace, UText } from "@uzum-tech/ui"
import CoralButton from "../shared/ui/CoralButton.vue"
import { useSettingsStore } from "../app/stores/settings"
import { type ApiError } from "../shared/api/client"
import { fetchAdminTxReceipt } from "../shared/api/endpoints"
import { notifyError } from "../shared/ui/notifications"

const settings = useSettingsStore()

const txId = ref("")
const isLoading = ref(false)
const errorMessage = ref<string | null>(null)
const receipt = ref<unknown | null>(null)

const canFetch = computed(() => txId.value.trim().length > 0 && !isLoading.value)

const handleLookup = async () => {
  const id = txId.value.trim()
  if (!id) return
  isLoading.value = true
  errorMessage.value = null
  receipt.value = null
  try {
    receipt.value = await fetchAdminTxReceipt(id, { baseUrl: settings.apiBaseUrl })
  } catch (e) {
    const msg = (e as ApiError)?.message ?? "Unable to fetch receipt."
    errorMessage.value = msg
    notifyError(msg)
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <div class="receipt">
    <div>
      <h1 class="page-title">Receipt lookup</h1>
      <p class="page-subtitle">Load receipt details by transaction id.</p>
    </div>

    <UCard title="Find by tx_id">
      <USpace vertical :size="12">
        <UInput v-model:value="txId" data-testid="receipt-txid" placeholder="Transaction ID (uuid)" />
        <div class="row">
          <CoralButton :disabled="!canFetch" test-id="receipt-fetch" @click="handleLookup">
            {{ isLoading ? "Fetching..." : "Fetch receipt" }}
          </CoralButton>
          <UText depth="3" class="muted">Uses local admin endpoint (/v1/admin/transactions/:tx_id).</UText>
        </div>

        <div v-if="errorMessage" class="error-text">{{ errorMessage }}</div>

        <div v-if="receipt" class="receipt-json">
          <pre>{{ JSON.stringify(receipt, null, 2) }}</pre>
        </div>
      </USpace>
    </UCard>
  </div>
</template>

<style scoped>
.receipt {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.receipt-json {
  border: 1px solid var(--border);
  background: var(--surface-2);
  border-radius: 12px;
  padding: 12px;
  overflow: auto;
  max-height: 520px;
}

.receipt-json pre {
  margin: 0;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  font-size: 12px;
  line-height: 1.4;
}

@media (max-width: 720px) {
  .row {
    flex-direction: column;
    align-items: stretch;
  }
}
</style>
