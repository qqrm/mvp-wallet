<script setup lang="ts"></script>

<template>
  <div class="page-container">
    <div class="page-surface">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.page-container {
  width: 100%;
}

.page-surface {
  max-width: 1600px;
  margin: 0 auto;
  padding: 32px;
  border-radius: var(--radius);
  background: var(--surface);
  border: 1px solid var(--border);
  box-shadow: var(--shadow);
}

@media (max-width: 720px) {
  .page-surface {
    padding: 18px;
    border-radius: 14px;
  }
}
</style>
