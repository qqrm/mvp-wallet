<script setup lang="ts">
import { UCard, UGrid, UGridItem, USpace, UText, UButton } from "@uzum-tech/ui"
</script>

<template>
  <USpace vertical :size="16">
    <UText strong>Admin</UText>
    <UGrid :cols="3" :x-gap="16" :y-gap="16">
      <UGridItem>
        <UCard title="Users">
          <USpace vertical>
            <UText depth="3">User management will be wired here.</UText>
            <UButton size="small" disabled>Invite user</UButton>
          </USpace>
        </UCard>
      </UGridItem>
      <UGridItem>
        <UCard title="Accounts">
          <USpace vertical>
            <UText depth="3">Account lifecycle actions coming next.</UText>
            <UButton size="small" disabled>Create account</UButton>
          </USpace>
        </UCard>
      </UGridItem>
      <UGridItem>
        <UCard title="Topup">
          <USpace vertical>
            <UText depth="3">Admin topups placeholder.</UText>
            <UButton size="small" disabled>Run topup</UButton>
          </USpace>
        </UCard>
      </UGridItem>
    </UGrid>
  </USpace>
</template>
