<script setup lang="ts">
import { computed, onMounted, ref } from "vue"
import { getUserToken, setUserToken } from "../api/auth"
import {
  getWalletBalances,
  getWalletTxs,
  idempotencyKey,
  postWalletTransfer,
  type BalanceItem,
  type TxItem,
} from "../api/client"

const userId = ref("42")
const userToken = ref(getUserToken())

const balances = ref<BalanceItem[]>([])
const txs = ref<TxItem[]>([])
const loading = ref(false)
const errorText = ref<string | null>(null)

const transferTo = ref("99")
const transferCurrency = ref("UZS")
const transferAmount = ref(25000)

const totalAvailable = computed(() => {
  return balances.value.reduce((acc, b) => acc + (b.available_minor || 0), 0)
})

function money(n: number): string {
  return n.toLocaleString("en-US")
}

function saveToken() {
  setUserToken(userToken.value)
}

async function refresh() {
  loading.value = true
  errorText.value = null
  try {
    saveToken()
    const b = await getWalletBalances(userId.value)
    balances.value = b.balances

    const t = await getWalletTxs(userId.value, 50)
    txs.value = t.txs
  } catch (e: any) {
    errorText.value = e?.message ?? String(e)
  } finally {
    loading.value = false
  }
}

async function doTransfer() {
  loading.value = true
  errorText.value = null
  try {
    saveToken()
    await postWalletTransfer(
      userId.value,
      transferTo.value,
      transferCurrency.value,
      Number(transferAmount.value),
      idempotencyKey("xfer"),
    )
    await refresh()
  } catch (e: any) {
    errorText.value = e?.message ?? String(e)
    loading.value = false
  }
}

onMounted(refresh)
</script>

<template>
  <section class="card">
    <div class="card-title">User view (requires USER token)</div>

    <div class="row">
      <div class="label">User ID</div>
      <input class="input" v-model="userId" />
      <button class="btn" :disabled="loading" @click="refresh">Refresh</button>
    </div>

    <div class="row">
      <div class="label">User token</div>
      <input class="input" v-model="userToken" placeholder="user:<user_id>:<sig>" />
      <button class="btn" :disabled="loading" @click="saveToken">Save</button>
    </div>

    <div v-if="errorText" class="error">{{ errorText }}</div>

    <div class="divider"></div>

    <div class="balances" v-if="balances.length > 0">
      <div class="balance" v-for="b in balances" :key="b.currency">
        <div class="bal-cur">{{ b.currency }}</div>
        <div class="bal-num">{{ money(b.available_minor) }}</div>
        <div class="bal-sub">hold: {{ money(b.hold_minor) }}</div>
      </div>
    </div>
    <div class="muted" v-else>No balances yet</div>

    <div class="total">
      <div class="muted">Total available (sum, minor):</div>
      <div class="total-num">{{ money(totalAvailable) }}</div>
    </div>

    <div class="divider"></div>

    <div class="muted">Transfer</div>
    <div class="row">
      <input class="input" v-model="transferTo" placeholder="To user id" />
      <input class="input" v-model="transferCurrency" placeholder="Currency (UZS)" />
      <input class="input" type="number" v-model="transferAmount" placeholder="Amount minor" />
      <button class="btn-primary" :disabled="loading" @click="doTransfer">Send</button>
    </div>

    <div class="divider"></div>

    <div class="card-title">Transactions</div>

    <div v-if="txs.length === 0" class="muted">No transactions yet</div>

    <div class="txs" v-else>
      <div class="tx" v-for="t in txs" :key="t.tx_id">
        <div class="tx-left">
          <div class="tx-desc">{{ t.description }}</div>
          <div class="tx-meta">
            <span class="pill">{{ t.tx_type }}</span>
            <span class="pill">{{ t.state }}</span>
            <span class="muted">{{ t.created_at }}</span>
          </div>
        </div>

        <div class="tx-right">
          <div class="tx-amt" :class="{ neg: t.amount_minor < 0 }">
            {{ t.amount_minor < 0 ? "-" : "+" }}{{ money(Math.abs(t.amount_minor)) }} {{ t.currency }}
          </div>
          <div class="muted tx-id">{{ t.tx_id }}</div>
        </div>
      </div>
    </div>
  </section>
</template>
