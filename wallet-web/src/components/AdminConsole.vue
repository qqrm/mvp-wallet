<script setup lang="ts">
import { computed, onMounted, ref } from "vue"
import { getAdminToken, setAdminToken } from "../api/auth"
import {
  adminCreateAccount,
  adminCloseAccount,
  adminCreateUser,
  adminHoldAuthorize,
  adminHoldCancel,
  adminHoldCapture,
  adminOpenCurrencyAccount,
  adminRefundPayment,
  adminTopup,
  adminUserBalances,
  adminUserTxs,
  getCurrencies,
  idempotencyKey,
} from "../api/client"

const adminToken = ref(getAdminToken())

const loading = ref(false)
const errorText = ref<string | null>(null)
const lastJson = ref<any>(null)

const currencies = ref<{ code: string; minor_units: number }[]>([])
const currencyOptions = computed(() => currencies.value.map((c) => c.code))

// Inputs
const userId = ref("42")
const currency = ref("UZS")
const amountMinor = ref(100000)

const label = ref("main")
const accountId = ref("")

const holdTxId = ref("")
const payTxId = ref("")

function saveToken() {
  setAdminToken(adminToken.value)
}

async function run<T>(fn: () => Promise<T>) {
  loading.value = true
  errorText.value = null
  lastJson.value = null
  try {
    saveToken()
    const out = await fn()
    lastJson.value = out
  } catch (e: any) {
    errorText.value = e?.message ?? String(e)
  } finally {
    loading.value = false
  }
}

async function refreshCurrencies() {
  try {
    const r = await getCurrencies()
    currencies.value = r.currencies ?? []
    if (currencies.value.length > 0 && !currencyOptions.value.includes(currency.value)) {
      currency.value = currencies.value[0].code
    }
  } catch {
    // ignore in UI; admin endpoints still work
  }
}

onMounted(refreshCurrencies)
</script>

<template>
  <section class="card">
    <div class="card-title">Admin console</div>

    <div class="row">
      <div class="muted">Admin token</div>
    </div>

    <div class="row">
      <input class="input" v-model="adminToken" placeholder="admin-dev-token" />
      <button class="btn" :disabled="loading" @click="saveToken">Save</button>
      <button class="btn" :disabled="loading" @click="refreshCurrencies">Reload currencies</button>
    </div>

    <div class="hint muted">
      Admin auth: <code>Authorization: Bearer &lt;ADMIN_TOKEN&gt;</code>
    </div>

    <div class="divider"></div>

    <div class="grid2">
      <div>
        <div class="muted section-title">User + account</div>

        <div class="row">
          <input class="input" v-model="userId" placeholder="User ID" />
          <button class="btn-primary" :disabled="loading" @click="run(() => adminCreateUser({ user_id: userId }))">
            Create user
          </button>
        </div>

        <div class="row">
          <select class="input" v-model="currency">
            <option v-for="c in currencyOptions" :key="c" :value="c">{{ c }}</option>
            <option v-if="currencyOptions.length === 0" value="UZS">UZS</option>
          </select>
          <button class="btn-primary" :disabled="loading" @click="run(() => adminOpenCurrencyAccount(userId, { currency }))">
            Open currency account
          </button>
        </div>

        <div class="row">
          <input class="input" v-model="label" placeholder="Account label" />
          <button class="btn-primary" :disabled="loading" @click="run(() => adminCreateAccount({ owner_user_id: userId, currency, label }))">
            Create account (lifecycle)
          </button>
        </div>

        <div class="row">
          <input class="input" v-model="accountId" placeholder="Account ID (acc_... or numeric)" />
          <button class="btn-primary" :disabled="loading" @click="run(() => adminCloseAccount(accountId, idempotencyKey('acc-close')))">
            Close account
          </button>
        </div>
      </div>

      <div>
        <div class="muted section-title">Money ops</div>

        <div class="row">
          <input class="input" type="number" v-model="amountMinor" placeholder="Amount (minor)" />
          <button class="btn-primary" :disabled="loading" @click="run(() => adminTopup({ user_id: userId, currency, amount_minor: Number(amountMinor) }, idempotencyKey('topup')))">
            Topup
          </button>
        </div>

        <div class="row">
          <input class="input" type="number" v-model="amountMinor" placeholder="Amount (minor)" />
          <button class="btn-primary" :disabled="loading" @click="run(() => adminHoldAuthorize({ user_id: userId, currency, amount_minor: Number(amountMinor) }, idempotencyKey('hold')))">
            Hold authorize
          </button>
        </div>

        <div class="row">
          <input class="input" v-model="holdTxId" placeholder="Hold tx_id" />
          <button class="btn" :disabled="loading" @click="run(() => adminHoldCapture(holdTxId, idempotencyKey('hold-cap')))">
            Capture
          </button>
          <button class="btn" :disabled="loading" @click="run(() => adminHoldCancel(holdTxId, idempotencyKey('hold-cancel')))">
            Cancel
          </button>
        </div>

        <div class="row">
          <input class="input" v-model="payTxId" placeholder="Payment tx_id (for refund)" />
          <button class="btn" :disabled="loading" @click="run(() => adminRefundPayment(payTxId, idempotencyKey('refund')))">
            Refund
          </button>
        </div>
      </div>
    </div>

    <div class="divider"></div>

    <div class="row">
      <button class="btn" :disabled="loading" @click="run(() => adminUserBalances(userId))">Get balances</button>
      <button class="btn" :disabled="loading" @click="run(() => adminUserTxs(userId, 50))">Get txs</button>
    </div>

    <div v-if="errorText" class="error">{{ errorText }}</div>

    <pre v-if="lastJson" class="json">{{ JSON.stringify(lastJson, null, 2) }}</pre>
  </section>
</template>

<style scoped>
.section-title { margin-bottom: 8px; }
.grid2 {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}
@media (max-width: 980px) {
  .grid2 { grid-template-columns: 1fr; }
}
.json {
  margin-top: 12px;
  padding: 12px;
  background: #0b1020;
  color: #e7ecff;
  border-radius: 12px;
  overflow: auto;
  max-height: 320px;
}
code { font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace; }
</style>
