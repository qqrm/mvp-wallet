<script setup lang="ts">
import { computed, h, ref, watch } from "vue"
import { useRoute, useRouter } from "vue-router"
import { UButton, UCard, UDataTable, UGrid, UGridItem, UInput, USelect, USpace } from "@uzum-tech/ui"
import CoralButton from "../shared/ui/CoralButton.vue"
import { notifyError, notifySuccess } from "../shared/ui/notifications"
import { useSettingsStore } from "../app/stores/settings"
import {
  type CurrencyItem,
  fetchCurrencies,
  fetchWalletBalances,
  fetchWalletTxs,
  postWalletTransfer,
} from "../shared/api/endpoints"
import type { ApiError } from "../shared/api/client"

type BalanceRow = {
  currency: string
  availableMinor: number
  reservedMinor: number
}

type TxRow = {
  id: string
  type: string
  currency: string
  amountMinor: number
  counterparty: string
  status: string
}

const settings = useSettingsStore()
const route = useRoute()
const router = useRouter()
const balances = ref<BalanceRow[]>([])
const transactions = ref<TxRow[]>([])
const currencies = ref<CurrencyItem[]>([])
const isRefreshing = ref(false)
const isSending = ref(false)
const authToken = ref("")

const selectedUserId = computed(() => {
  const queryValue = route.query.as
  if (Array.isArray(queryValue)) {
    return queryValue[0]?.toString().trim() || "u01"
  }
  return queryValue?.toString().trim() || "u01"
})

const readAuthToken = () => {
  if (typeof window === "undefined") return ""
  try {
    return window.localStorage.getItem("wallet-web.authToken")?.trim() ?? ""
  } catch {
    return ""
  }
}

authToken.value = readAuthToken()

const isDevMode = computed(() => authToken.value.length === 0)

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

const currencyMinorUnits = computed<Record<string, number>>(() =>
  currencies.value.reduce((acc, item) => {
    acc[item.code] = item.minor_units
    return acc
  }, {} as Record<string, number>),
)

const formatMinorAmount = (currency: string, minorValue: number) => {
  const minorUnits = currencyMinorUnits.value[currency] ?? 2
  const sign = minorValue < 0 ? "-" : ""
  const absolute = Math.abs(minorValue)
  const raw = absolute.toString()
  if (minorUnits <= 0) {
    return `${sign}${raw} ${currency}`
  }
  const padded = raw.padStart(minorUnits + 1, "0")
  const whole = padded.slice(0, -minorUnits)
  const fraction = padded.slice(-minorUnits)
  const formattedWhole = new Intl.NumberFormat(undefined, { maximumFractionDigits: 0 }).format(Number(whole))
  return `${sign}${formattedWhole}.${fraction} ${currency}`
}

const normalizeMinorDigits = (value: string, minorUnits: number) => {
  const trimmed = value.trim()
  if (!trimmed) return null
  const normalized = trimmed.startsWith(".") ? `0${trimmed}` : trimmed
  if (!/^\d+(\.\d*)?$/.test(normalized)) return null
  const [whole, fraction = ""] = normalized.split(".")
  if (minorUnits === 0 && fraction.length > 0) return null
  if (fraction.length > minorUnits) return null
  const padded = fraction.padEnd(minorUnits, "0")
  return `${whole}${padded}`
}

const isAmountTooLarge = (value: string, minorUnits: number) => {
  const digits = normalizeMinorDigits(value, minorUnits)
  if (!digits) return false
  const normalized = digits.replace(/^0+(?=\d)/, "") || "0"
  const max = Number.MAX_SAFE_INTEGER.toString()
  if (normalized.length > max.length) return true
  if (normalized.length < max.length) return false
  return normalized > max
}

const parseMinorAmount = (value: string, minorUnits: number) => {
  const digits = normalizeMinorDigits(value, minorUnits)
  if (!digits) return null
  const minor = Number(digits)
  if (!Number.isSafeInteger(minor)) return null
  return minor
}

const buildHeaders = (userId: string, extra?: Record<string, string>) => {
  const headers = new Headers(extra)
  if (authToken.value) {
    headers.set("Authorization", `Bearer ${authToken.value}`)
  } else {
    headers.set("X-Dev-User", userId)
  }
  return headers
}

const resolveAuthErrorMessage = (error: ApiError, fallback: string) => {
  if (error.status === 401 || error.status === 403) {
    if (!authToken.value) {
      return "Backend requires auth. Set wallet-web.authToken in localStorage or pass ?token=... once."
    }
    return "Authorization failed. Check your token."
  }
  return error.message || fallback
}

const loadCurrencies = async (userId: string) => {
  const response = await fetchCurrencies({ baseUrl: settings.apiBaseUrl, headers: buildHeaders(userId) })
  currencies.value = response.items
}

const loadWallet = async () => {
  if (isRefreshing.value) return
  isRefreshing.value = true
  const userId = selectedUserId.value
  try {
    if (currencies.value.length === 0) {
      await loadCurrencies(userId)
    }
    const headers = buildHeaders(userId)
    const [balancesResponse, txsResponse] = await Promise.all([
      fetchWalletBalances(userId, { baseUrl: settings.apiBaseUrl, headers }),
      fetchWalletTxs(userId, 50, { baseUrl: settings.apiBaseUrl, headers }),
    ])
    balances.value = balancesResponse.balances.map((item) => ({
      currency: item.currency,
      availableMinor: item.available_minor,
      reservedMinor: item.hold_minor,
    }))
    transactions.value = txsResponse.txs.map((tx) => ({
      id: tx.tx_id,
      type: tx.tx_type,
      currency: tx.currency,
      amountMinor: tx.amount_minor,
      counterparty: tx.description,
      status: tx.state,
    }))
  } catch (error) {
    const apiError = error as ApiError
    const message =
      apiError?.status !== undefined
        ? resolveAuthErrorMessage(apiError, "Failed to load wallet data")
        : error instanceof Error
          ? error.message
          : "Failed to load wallet data"
    notifyError(message)
  } finally {
    isRefreshing.value = false
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
            title: "Copy transaction id",
            "aria-label": "Copy transaction id",
            onClick: (e: MouseEvent) => {
              e.preventDefault()
              e.stopPropagation()
              void copyToClipboard(row.id)
            },
          },
          [
            h(
              "svg",
              {
                class: "copy-icon",
                viewBox: "0 0 24 24",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                "aria-hidden": "true",
              },
              [
                h("path", {
                  d: "M9 9.5C9 8.11929 10.1193 7 11.5 7H18.5C19.8807 7 21 8.11929 21 9.5V16.5C21 17.8807 19.8807 19 18.5 19H11.5C10.1193 19 9 17.8807 9 16.5V9.5Z",
                  stroke: "currentColor",
                  "stroke-width": "1.6",
                }),
                h("path", {
                  d: "M15 7V6C15 4.89543 14.1046 4 13 4H6C4.89543 4 4 4.89543 4 6V13C4 14.1046 4.89543 15 6 15H7",
                  stroke: "currentColor",
                  "stroke-width": "1.6",
                  "stroke-linecap": "round",
                }),
              ],
            ),
          ],
        ),
      ]),
  },
  { title: "Type", key: "type" },
  { title: "Currency", key: "currency" },
  {
    title: "Amount",
    key: "amountMinor",
    render: (row: TxRow) => formatMinorAmount(row.currency, row.amountMinor),
  },
  { title: "Description", key: "counterparty" },
  { title: "Status", key: "status" },
]

const transferRecipientId = ref("")
const transferCurrency = ref("")
const transferAmountRaw = ref("")

const currencyOptions = computed(() => currencies.value.map((item) => ({ label: item.code, value: item.code })))

const canSend = computed(() => {
  const recipient = transferRecipientId.value.trim()
  if (!recipient) return false
  const currency = transferCurrency.value
  const minorUnits = currencyMinorUnits.value[currency]
  if (minorUnits === undefined) return false
  const amountMinor = parseMinorAmount(transferAmountRaw.value, minorUnits)
  return amountMinor !== null && amountMinor > 0
})

const totalByCurrency = computed(() =>
  balances.value.map((b) => ({
    currency: b.currency,
    totalMinor: b.availableMinor + b.reservedMinor,
  })),
)

const ensureTransferCurrency = () => {
  const options = currencyOptions.value
  const [first] = options
  if (!first) return
  if (!options.some((option) => option.value === transferCurrency.value)) {
    transferCurrency.value = first.value
  }
}

const buildIdempotencyKey = () => {
  if (typeof crypto !== "undefined" && "randomUUID" in crypto) {
    return crypto.randomUUID()
  }
  const randomPart = () => Math.random().toString(16).slice(2)
  return `idem-${Date.now().toString(16)}-${randomPart()}-${randomPart()}`
}

const handleSend = async () => {
  const recipient = transferRecipientId.value.trim()
  if (!recipient) {
    notifyError("Enter a recipient ID.")
    return
  }
  const currency = transferCurrency.value
  const minorUnits = currencyMinorUnits.value[currency]
  if (minorUnits === undefined) {
    notifyError("Select a supported currency.")
    return
  }
  const amountMinor = parseMinorAmount(transferAmountRaw.value, minorUnits)
  if (amountMinor === null || amountMinor <= 0) {
    if (isAmountTooLarge(transferAmountRaw.value, minorUnits)) {
      notifyError("Amount is too large.")
      return
    }
    notifyError("Amount has too many decimal places for this currency.")
    return
  }
  const userId = selectedUserId.value
  const idempotencyKey = buildIdempotencyKey()
  if (isSending.value) return
  isSending.value = true
  try {
    await postWalletTransfer(
      userId,
      {
        to_user_id: recipient,
        currency,
        amount_minor: amountMinor,
      },
      {
        baseUrl: settings.apiBaseUrl,
        headers: buildHeaders(userId, { "Idempotency-Key": idempotencyKey }),
      },
    )
    notifySuccess("Transfer submitted.")
    transferRecipientId.value = ""
    transferAmountRaw.value = ""
    await loadWallet()
  } catch (error) {
    const apiError = error as ApiError
    const message =
      apiError?.status !== undefined
        ? resolveAuthErrorMessage(apiError, "Transfer failed")
        : error instanceof Error
          ? error.message
          : "Transfer failed"
    notifyError(message)
  } finally {
    isSending.value = false
  }
}

watch(
  () => currencies.value.length,
  () => {
    ensureTransferCurrency()
  },
  { immediate: true },
)

watch(
  () => route.query.token,
  (token) => {
    if (typeof token !== "string" || !token.trim()) return
    const trimmed = token.trim()
    authToken.value = trimmed
    if (typeof window !== "undefined") {
      window.localStorage.setItem("wallet-web.authToken", trimmed)
    }
    const nextQuery = { ...route.query }
    delete nextQuery.token
    void router.replace({ query: nextQuery })
  },
  { immediate: true },
)

watch(
  () => [selectedUserId.value, settings.apiBaseUrl, authToken.value] as const,
  ([, nextApiBase, nextToken], previous) => {
    const previousValues = previous ?? []
    const [, prevApiBase, prevToken] = previousValues
    if (prevApiBase !== undefined && (prevApiBase !== nextApiBase || prevToken !== nextToken)) {
      currencies.value = []
      balances.value = []
      transactions.value = []
    }
    void loadWallet()
  },
  { immediate: true },
)
</script>

<template>
  <div class="wallet">
    <div class="wallet-header">
      <div>
        <h1 class="page-title">Wallet</h1>
        <p class="page-subtitle">Balances, transfers, and recent activity.</p>
        <div class="wallet-meta">
          <span class="user-pill">User: {{ selectedUserId }}</span>
          <span v-if="isDevMode" class="dev-pill">Dev mode</span>
          <UButton size="small" :disabled="isRefreshing" @click="loadWallet">
            {{ isRefreshing ? "Refreshing..." : "Refresh" }}
          </UButton>
        </div>
      </div>

      <div class="wallet-totals">
        <div v-for="row in totalByCurrency" :key="row.currency" class="total-pill">
          <span class="total-currency">{{ row.currency }}</span>
          <span class="total-value">{{ formatMinorAmount(row.currency, row.totalMinor) }}</span>
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

                <div class="balance-amount">{{ formatMinorAmount(b.currency, b.availableMinor) }}</div>

                <div class="balance-meta">
                  <div class="meta-item">
                    <div class="meta-key">Reserved</div>
                    <div class="meta-value">{{ formatMinorAmount(b.currency, b.reservedMinor) }}</div>
                  </div>
                  <div class="meta-item">
                    <div class="meta-key">Total</div>
                    <div class="meta-value">
                      {{ formatMinorAmount(b.currency, b.availableMinor + b.reservedMinor) }}
                    </div>
                  </div>
                </div>
              </div>
            </UGridItem>
          </UGrid>
        </UCard>

        <UCard title="Recent transactions" class="mt16">
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
            <UInput v-model:value="transferAmountRaw" placeholder="Amount" inputmode="decimal" />
            <div class="actions-row">
              <CoralButton :disabled="!canSend || isSending" test-id="wallet-send" @click="handleSend">
                {{ isSending ? "Sending..." : "Send" }}
              </CoralButton>
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

.wallet-meta {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-top: 8px;
}

.user-pill {
  display: inline-flex;
  align-items: center;
  padding: 4px 10px;
  border-radius: 999px;
  border: 1px solid var(--border);
  background: var(--surface);
  font-size: 12px;
  font-weight: 700;
  color: var(--text-muted);
}

.dev-pill {
  display: inline-flex;
  align-items: center;
  padding: 4px 10px;
  border-radius: 999px;
  border: 1px solid rgba(255, 255, 255, 0.18);
  background: var(--brand-primary);
  font-size: 12px;
  font-weight: 700;
  color: rgba(255, 255, 255, 0.95);
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
  width: 26px;
  padding: 0;
  border-radius: 8px;
  border: 1px solid var(--border);
  background: var(--surface-2);
  color: var(--text-muted);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
}

.copy-btn:hover {
  border-color: rgba(112, 0, 255, 0.24);
  color: var(--text);
}

.copy-btn:active {
  transform: translateY(1px);
}

.copy-icon {
  width: 14px;
  height: 14px;
}

@media (max-width: 1024px) {
  .wallet-header {
    flex-direction: column;
  }
}
</style>
