<script setup lang="ts">
import { computed, h, ref, watch } from "vue"
import { useRoute } from "vue-router"
import { UCard, UDataTable, UGrid, UGridItem, UInput, USelect, USpace, UText } from "@uzum-tech/ui"
import CoralButton from "../shared/ui/CoralButton.vue"
import { useSessionStore } from "../app/stores/session"
import { useSettingsStore } from "../app/stores/settings"
import { type ApiError } from "../shared/api/client"
import {
  fetchCurrencies,
  fetchWalletBalances,
  fetchWalletTxs,
  postWalletTransfer,
  type CurrencyItem,
} from "../shared/api/endpoints"
import { notifyError, notifySuccess } from "../shared/ui/notifications"

type BalanceRow = {
  currency: string
  availableMinor: bigint
  reservedMinor: bigint
}

type TxRow = {
  id: string
  type: string
  currency: string
  amountMinor: bigint
  counterparty: string
  status: string
}

const DEFAULT_USER_ID = "u01"

const I64_MAX = 9_223_372_036_854_775_807n
const I64_MIN = -9_223_372_036_854_775_808n
const JS_SAFE_MAX = BigInt(Number.MAX_SAFE_INTEGER)
const JS_SAFE_MIN = -JS_SAFE_MAX

const settings = useSettingsStore()
const session = useSessionStore()
const route = useRoute()

const balances = ref<BalanceRow[]>([])
const transactions = ref<TxRow[]>([])
const currencies = ref<CurrencyItem[]>([])
const currentUserId = ref(DEFAULT_USER_ID)

watch(
  () => route.query.as,
  (value) => {
    if (typeof value === "string" && value.trim()) {
      currentUserId.value = value.trim()
      return
    }
    currentUserId.value = DEFAULT_USER_ID
  },
  { immediate: true },
)

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

const numberFormatParts = new Intl.NumberFormat(undefined).formatToParts(1000.1)
const GROUP_SEPARATOR = numberFormatParts.find((part) => part.type === "group")?.value ?? ","
const DECIMAL_SEPARATOR = numberFormatParts.find((part) => part.type === "decimal")?.value ?? "."

const groupDigits = (digits: string) => {
  if (digits.length <= 3) return digits
  const parts: string[] = []
  for (let index = digits.length; index > 0; index -= 3) {
    const start = Math.max(index - 3, 0)
    parts.unshift(digits.slice(start, index))
  }
  return parts.join(GROUP_SEPARATOR)
}

const parseMinorAmount = (value: string, minorUnits: number): bigint | null => {
  const trimmedRaw = value.trim()
  if (!trimmedRaw) return null
  if (trimmedRaw.startsWith("-")) return null

  // Accept: "12", "12.", "12.3", ".5" ; Reject non-decimal chars
  const normalized = trimmedRaw.startsWith(".") ? `0${trimmedRaw}` : trimmedRaw
  if (!/^\d*\.?\d*$/.test(normalized)) return null
  if (normalized === ".") return null
  if (minorUnits === 0 && normalized.includes(".")) return null

  const [wholeRaw = "", fractionRaw = ""] = normalized.split(".")
  if (wholeRaw === "" && fractionRaw === "") return null
  if (fractionRaw.length > minorUnits) return null

  const whole = wholeRaw === "" ? "0" : wholeRaw
  if (minorUnits === 0) return BigInt(whole)

  const fraction = fractionRaw.padEnd(minorUnits, "0")
  return BigInt(`${whole}${fraction}`)
}

const currencyMinorUnits = computed(() => new Map(currencies.value.map((item) => [item.code, item.minor_units])))
const getMinorUnits = (currency: string) => currencyMinorUnits.value.get(currency) ?? 2

const formatMinorAmount = (currency: string, minorValue: bigint) => {
  const minorUnits = getMinorUnits(currency)
  const isNegative = minorValue < 0n
  const absValue = isNegative ? -minorValue : minorValue

  if (minorUnits === 0) {
    return `${isNegative ? "-" : ""}${groupDigits(absValue.toString())}`
  }

  const base = 10n ** BigInt(minorUnits)
  const whole = absValue / base
  const fraction = (absValue % base).toString().padStart(minorUnits, "0")
  return `${isNegative ? "-" : ""}${groupDigits(whole.toString())}${DECIMAL_SEPARATOR}${fraction}`
}

const buildHeaders = (userId: string, extra?: HeadersInit) => {
  const headers = new Headers()
  headers.set("X-Dev-User", userId)
  if (extra) {
    const extraHeaders = new Headers(extra)
    extraHeaders.forEach((value, key) => headers.set(key, value))
  }
  return headers
}

const resolveAuthError = (error: ApiError, fallbackMessage: string) => {
  if (error?.status === 401 || error?.status === 403) {
    return "Request rejected. This UI expects localhost dev auth."
  }
  return error?.message ?? fallbackMessage
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
const transferCurrency = ref("UZS")
const transferAmount = ref("")

const currencyOptions = computed(() => currencies.value.map((currency) => ({ label: currency.code, value: currency.code })))

watch(
  currencies,
  (items) => {
    if (!items.length) return
    const first = items[0]
    if (first && !items.some((item) => item.code === transferCurrency.value)) {
      transferCurrency.value = first.code
    }
  },
  { immediate: true },
)

const parsedTransferAmount = computed(() => parseMinorAmount(transferAmount.value, getMinorUnits(transferCurrency.value)))

const canSend = computed(() => {
  const recipient = transferRecipientId.value.trim()
  const amount = parsedTransferAmount.value
  return recipient.length > 0 && amount !== null && amount > 0n && !session.isLoading
})

const totalByCurrency = computed(() =>
  balances.value.map((b) => ({
    currency: b.currency,
    total: b.availableMinor + b.reservedMinor,
  })),
)

const loadWallet = async () => {
  session.setLoading(true)
  try {
    const userId = currentUserId.value
    const headers = buildHeaders(userId)

    const currencyResponse = await fetchCurrencies({ baseUrl: settings.apiBaseUrl, headers })
    currencies.value = currencyResponse.items

    const [balanceResponse, txResponse] = await Promise.all([
      fetchWalletBalances(userId, { baseUrl: settings.apiBaseUrl, headers }),
      fetchWalletTxs(userId, 50, { baseUrl: settings.apiBaseUrl, headers }),
    ])

    balances.value = balanceResponse.balances.map((item) => ({
      currency: item.currency,
      availableMinor: BigInt(item.available_minor),
      reservedMinor: BigInt(item.hold_minor),
    }))
    transactions.value = txResponse.txs.map((item) => ({
      id: item.tx_id,
      type: item.tx_type,
      currency: item.currency,
      amountMinor: BigInt(item.amount_minor),
      counterparty: item.description || "-",
      status: item.state,
    }))
  } catch (error) {
    notifyError(resolveAuthError(error as ApiError, "Unable to load wallet data."))
  } finally {
    session.setLoading(false)
  }
}

watch(
  [() => settings.apiBaseUrl, currentUserId],
  ([nextBaseUrl], [prevBaseUrl]) => {
    if (prevBaseUrl && nextBaseUrl !== prevBaseUrl) {
      currencies.value = []
    }
    void loadWallet()
  },
  { immediate: true },
)

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
    notifyError("Recipient ID is required.")
    return
  }

  const amountMinor = parsedTransferAmount.value
  if (amountMinor === null) {
    notifyError("Enter a valid amount.")
    return
  }
  if (amountMinor <= 0n) {
    notifyError("Amount must be greater than zero.")
    return
  }
  if (amountMinor > I64_MAX || amountMinor < I64_MIN) {
    notifyError("Amount is too large.")
    return
  }
  if (amountMinor > JS_SAFE_MAX || amountMinor < JS_SAFE_MIN) {
    notifyError("Amount is too large.")
    return
  }

  const amountNumber = Number(amountMinor)
  const userId = currentUserId.value

  session.setLoading(true)
  try {
    const idemKey = buildIdempotencyKey()
    await postWalletTransfer(
      userId,
      {
        to_user_id: recipient,
        currency: transferCurrency.value,
        amount_minor: amountNumber,
      },
      {
        baseUrl: settings.apiBaseUrl,
        headers: buildHeaders(userId, { "Idempotency-Key": idemKey }),
      },
    )
    notifySuccess("Transfer submitted.")
    transferRecipientId.value = ""
    transferAmount.value = ""
    await loadWallet()
  } catch (error) {
    notifyError(resolveAuthError(error as ApiError, "Transfer failed."))
  } finally {
    session.setLoading(false)
  }
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
        <div class="total-pill user-pill">
          <span class="total-currency">User</span>
          <span class="total-value">{{ currentUserId }}</span>
        </div>

        <CoralButton class="refresh-btn" :disabled="session.isLoading" test-id="wallet-refresh" @click="loadWallet">
          {{ session.isLoading ? "Refreshing..." : "Refresh" }}
        </CoralButton>

        <div v-for="row in totalByCurrency" :key="row.currency" class="total-pill">
          <span class="total-currency">{{ row.currency }}</span>
          <span class="total-value">{{ formatMinorAmount(row.currency, row.total) }}</span>
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
                    <div class="meta-value">{{ formatMinorAmount(b.currency, b.availableMinor + b.reservedMinor) }}</div>
                  </div>
                </div>
              </div>
            </UGridItem>
          </UGrid>
        </UCard>

        <UCard title="Recent transactions" class="mt16">
          <UText depth="3" class="muted">Latest activity from your wallet.</UText>
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
            <UInput v-model:value="transferAmount" placeholder="Amount" inputmode="decimal" />
            <div class="actions-row">
              <CoralButton :disabled="!canSend" test-id="wallet-send" @click="handleSend">Send</CoralButton>
            </div>
          </USpace>
        </UCard>

        <UCard title="Security" class="mt16">
          <div class="kv">
            <div class="kv-row">
              <div class="kv-key">Auth</div>
              <div class="kv-value">Local dev (X-Dev-User)</div>
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

.user-pill {
  align-items: center;
}

.refresh-btn {
  align-self: center;
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
