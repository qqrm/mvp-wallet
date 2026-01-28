<script setup lang="ts">
import { UCard, UGrid, UGridItem, USpace, UText } from "@uzum-tech/ui"
import CoralButton from "../shared/ui/CoralButton.vue"
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
            <UText depth="3">Invite, disable, and rotate credentials.</UText>
            <CoralButton disabled test-id="admin-invite">Invite user</CoralButton>
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

@media (max-width: 1024px) {
  .admin-header {
    flex-direction: column;
  }
}
</style>
