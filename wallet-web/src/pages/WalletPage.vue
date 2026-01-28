<script setup lang="ts">
import { computed, h, ref } from "vue"
import {
  UCard,
  UDataTable,
  UGrid,
  UGridItem,
  UInput,
  UInputNumber,
  USelect,
  USpace,
  UText,
} from "@uzum-tech/ui"
import CoralButton from "../shared/ui/CoralButton.vue"
import { notifyError, notifySuccess } from "../shared/ui/notifications"

type BalanceRow = {
  currency: string
  available: number
  reserved: number
}

type TxRow = {
  id: string
  type: string
  currency: string
  amount: number
  counterparty: string
  status: string
}

const balances = ref<BalanceRow[]>([
  { currency: "UZS", available: 1_250_000, reserved: 40_000 },
  { currency: "USD", available: 320.5, reserved: 0 },
])

const transactions = ref<TxRow[]>([
  { id: "tx_9f1c...a2", type: "Transfer", currency: "UZS", amount: -120_000, counterparty: "user_0192", status: "Posted" },
  { id: "tx_1a02...7b", type: "Topup", currency: "UZS", amount: 500_000, counterparty: "admin", status: "Posted" },
  { id: "tx_77bd...11", type: "Transfer", currency: "USD", amount: -25.0, counterparty: "user_0041", status: "Pending" },
])

const copyToClipboard = async (value: string) => {
  if (!value) return
  try {
    await navigator.clipboard.writeText(value)
    notifySuccess("Copied")
    return
  } catch {
    // fallback for non-secure contexts
    try {
      const textarea = document.createElement("textarea")
      textarea.value = value
      textarea.style.position = "fixed"
      textarea.style.left = "-9999px"
      textarea.style.top = "-9999px"
      document.body.appendChild(textarea)
      textarea.focus()
      textarea.select()
      const ok = document.execCommand("copy")
      document.body.removeChild(textarea)
      if (ok) notifySuccess("Copied")
      else notifyError("Copy failed")
    } catch {
      notifyError("Copy failed")
    }
  }
}

const txColumns = [
  {
    title: "ID",
    key: "id",
    render: (row: TxRow) =>
      h("div", { class: "tx-cell" }, [
        h(
          "span",
          {
            class: "tx-id",
            title: row.id,
          },
          row.id,
        ),
        h(
          "button",
          {
            class: "copy-btn",
            type: "button",
            onClick: (e: MouseEvent) => {
              e.preventDefault()
              e.stopPropagation()
              void copyToClipboard(row.id)
            },
          },
          "Copy",
        ),
      ]),
  },
  { title: "Type", key: "type" },
  { title: "Currency", key: "currency" },
  { title: "Amount", key: "amount" },
  { title: "Counterparty", key: "counterparty" },
  { title: "Status", key: "status" },
]

const transferRecipientId = ref("")
const transferCurrency = ref("UZS")
const transferAmount = ref<number | null>(null)

const currencyOptions = [
  { label: "UZS", value: "UZS" },
  { label: "USD", value: "USD" },
]

const canSend = computed(() => {
  const recipient = transferRecipientId.value.trim()
  const amount = transferAmount.value
  return recipient.length > 0 && typeof amount === "number" && amount > 0
})

const formatAmount = (currency: string, value: number) => {
  try {
    return new Intl.NumberFormat(undefined, {
      style: "currency",
      currency,
      maximumFractionDigits: currency === "UZS" ? 0 : 2,
    }).format(value)
  } catch {
    return `${value} ${currency}`
  }
}

const totalByCurrency = computed(() =>
  balances.value.map((b) => ({
    currency: b.currency,
    total: b.available + b.reserved,
  })),
)

const handleSend = () => {
  // UI demo only; wire to API when wallet transfers are ready.
}
</script>

<template>
  <div class="wallet">
    <div class="wallet-header">
      <div>
        <h1 class="page-title">Wallet</h1>
        <p class="page-subtitle">Balances, transfers, and recent activity.</p>
      </div>

      <div class="wallet-totals">
        <div v-for="row in totalByCurrency" :key="row.currency" class="total-pill">
          <span class="total-currency">{{ row.currency }}</span>
          <span class="total-value">{{ formatAmount(row.currency, row.total) }}</span>
        </div>
      </div>
    </div>

    <UGrid :cols="12" :x-gap="16" :y-gap="16">
      <UGridItem :span="8" class="stack">
        <UCard title="Balances">
          <UGrid :cols="2" :x-gap="12" :y-gap="12" class="balances-grid">
            <UGridItem v-for="b in balances" :key="b.currency">
              <div class="balance-card">
                <div class="balance-top">
                  <div class="balance-currency">{{ b.currency }}</div>
                  <div class="balance-badge">Active</div>
                </div>

                <div class="balance-amount">{{ formatAmount(b.currency, b.available) }}</div>

                <div class="balance-meta">
                  <div class="meta-item">
                    <div class="meta-key">Reserved</div>
                    <div class="meta-value">{{ formatAmount(b.currency, b.reserved) }}</div>
                  </div>
                  <div class="meta-item">
                    <div class="meta-key">Total</div>
                    <div class="meta-value">{{ formatAmount(b.currency, b.available + b.reserved) }}</div>
                  </div>
                </div>
              </div>
            </UGridItem>
          </UGrid>
        </UCard>

        <UCard title="Recent transactions" class="mt16">
          <UText depth="3" class="muted">Demo data. Replace with API-backed history when ready.</UText>
          <div class="mt12">
            <UDataTable :columns="txColumns" :data="transactions" />
          </div>
        </UCard>
      </UGridItem>

      <UGridItem :span="4" class="stack">
        <UCard title="Transfer" class="wallet-accent">
          <USpace vertical :size="12">
            <UInput v-model:value="transferRecipientId" placeholder="Recipient ID" />
            <USelect :options="currencyOptions" v-model:value="transferCurrency" />
            <UInputNumber v-model:value="transferAmount" placeholder="Amount" />
            <div class="actions-row">
              <CoralButton :disabled="!canSend" test-id="wallet-send" @click="handleSend">Send</CoralButton>
            </div>
          </USpace>
        </UCard>

        <UCard title="Security" class="mt16">
          <div class="kv">
            <div class="kv-row">
              <div class="kv-key">Session</div>
              <div class="kv-value">Local demo</div>
            </div>
            <div class="kv-row">
              <div class="kv-key">2FA</div>
              <div class="kv-value">Not configured</div>
            </div>
          </div>
        </UCard>
      </UGridItem>
    </UGrid>
  </div>
</template>

<style scoped>
.wallet {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.wallet-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
}

.wallet-totals {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  justify-content: flex-end;
}

.total-pill {
  display: inline-flex;
  align-items: baseline;
  gap: 10px;
  padding: 10px 12px;
  border-radius: 999px;
  border: 1px solid var(--border);
  background: var(--surface-2);
}

.total-currency {
  font-size: 12px;
  font-weight: 700;
  color: var(--text);
}

.total-value {
  font-size: 12px;
  font-weight: 700;
  color: var(--text-muted);
}

.stack {
  display: flex;
  flex-direction: column;
}

.mt16 {
  margin-top: 16px;
}

.mt12 {
  margin-top: 12px;
}

.balances-grid {
  margin-top: 8px;
}

.balance-card {
  border-radius: var(--radius);
  border: 1px solid var(--border);
  background: var(--surface-2);
  padding: 14px;
  box-shadow: var(--shadow-sm);
}

.balance-top {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.balance-currency {
  font-size: 13px;
  font-weight: 800;
  letter-spacing: -0.01em;
}

.balance-badge {
  font-size: 11px;
  font-weight: 700;
  padding: 4px 10px;
  border-radius: 999px;
  color: rgba(255, 255, 255, 0.95);
  background: var(--brand-primary);
}

.balance-amount {
  margin-top: 12px;
  font-size: 22px;
  font-weight: 800;
  letter-spacing: -0.02em;
}

.balance-meta {
  margin-top: 12px;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 10px;
}

.meta-item {
  padding: 10px 12px;
  border-radius: 12px;
  border: 1px solid var(--border);
  background: var(--surface);
}

.meta-key {
  font-size: 11px;
  color: var(--text-muted);
}

.meta-value {
  margin-top: 4px;
  font-size: 12px;
  font-weight: 700;
}

.actions-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.tx-cell {
  display: flex;
  align-items: center;
  gap: 10px;
  min-width: 0;
}

.tx-id {
  display: inline-block;
  max-width: 180px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  user-select: all;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  font-size: 12px;
  font-weight: 700;
}

.copy-btn {
  height: 26px;
  padding: 0 10px;
  border-radius: 10px;
  border: 1px solid var(--border);
  background: var(--surface);
  color: var(--text);
  font-size: 12px;
  font-weight: 700;
  cursor: pointer;
}

.copy-btn:hover {
  border-color: rgba(112, 0, 255, 0.24);
}

.copy-btn:active {
  transform: translateY(1px);
}

@media (max-width: 1024px) {
  .wallet-header {
    flex-direction: column;
  }
}
</style>
