<script setup lang="ts">
import { ref } from "vue"
import {
  UButton,
  UCard,
  UGrid,
  UGridItem,
  UInput,
  UInputNumber,
  USelect,
  USpace,
  UText,
} from "@uzum-tech/ui"

const transferCurrency = ref("UZS")
const transferAmount = ref<number | null>(null)

const currencyOptions = [
  { label: "UZS", value: "UZS" },
  { label: "USD", value: "USD" },
]
</script>

<template>
  <USpace vertical :size="16">
    <UText strong>Wallet</UText>
    <UGrid :cols="3" :x-gap="16" :y-gap="16">
      <UGridItem>
        <UCard title="Balances">
          <UText depth="3">Balances summary will render here.</UText>
        </UCard>
      </UGridItem>
      <UGridItem>
        <UCard title="Transfer">
          <USpace vertical :size="12">
            <UInput placeholder="Recipient ID" />
            <USelect :options="currencyOptions" v-model:value="transferCurrency" />
            <UInputNumber v-model:value="transferAmount" placeholder="Amount" />
            <UButton disabled>Send transfer</UButton>
          </USpace>
        </UCard>
      </UGridItem>
      <UGridItem>
        <UCard title="Transactions">
          <UText depth="3">Transaction history placeholder.</UText>
        </UCard>
      </UGridItem>
    </UGrid>
  </USpace>
</template>
