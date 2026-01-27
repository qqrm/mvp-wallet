<script setup lang="ts">
import { UButton, UCard, UInput, USpace, UText } from "@uzum-tech/ui"
</script>

<template>
  <USpace vertical :size="16">
    <UText strong>Receipt Lookup</UText>
    <UCard title="Load by tx_id">
      <USpace vertical :size="12">
        <UInput placeholder="Transaction ID" />
        <UButton disabled>Fetch receipt</UButton>
      </USpace>
    </UCard>
  </USpace>
</template>
