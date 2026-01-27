<script setup lang="ts">
import { UCard } from "@uzum-tech/ui"
</script>

<template>
  <UCard class="page-container">
    <slot />
  </UCard>
</template>

<style scoped>
.page-container {
  min-height: calc(100vh - 120px);
}
</style>
