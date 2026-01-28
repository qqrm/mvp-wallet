<script setup lang="ts">
import { ref } from "vue"
import { UCard, UInput, USpace, UText } from "@uzum-tech/ui"
import CoralButton from "../shared/ui/CoralButton.vue"

const txId = ref("")

const handleLookup = () => {
  // UI demo only; wire to receipt endpoint when ready.
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
        <UInput v-model:value="txId" placeholder="Transaction ID (e.g. tx_...)" />
        <div class="row">
          <CoralButton disabled test-id="receipt-fetch" @click="handleLookup">Fetch receipt</CoralButton>
          <UText depth="3" class="muted">Will be enabled once API is wired.</UText>
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

@media (max-width: 720px) {
  .row {
    flex-direction: column;
    align-items: stretch;
  }
}
</style>
