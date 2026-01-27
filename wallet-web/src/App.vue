<script setup lang="ts">
import { computed, onMounted, ref } from "vue"
import { getBalances, getTxs, postTopup, postTransfer, type BalanceItem, type TxItem } from "./api/client"

const userId = ref("42")

const balances = ref<BalanceItem[]>([])
const txs = ref<TxItem[]>([])
const loading = ref(false)
const errorText = ref<string | null>(null)

const topupCurrency = ref("UZS")
const topupAmount = ref(100000)

const transferTo = ref("99")
const transferCurrency = ref("UZS")
const transferAmount = ref(25000)

const totalAvailable = computed(() => {
  return balances.value.reduce((acc, b) => acc + (b.available_minor || 0), 0)
})

function money(n: number): string {
  // MVP: просто integer "minor"
  return n.toLocaleString("en-US")
}

async function refresh() {
  loading.value = true
  errorText.value = null
  try {
    const b = await getBalances(userId.value)
    balances.value = b.balances

    const t = await getTxs(userId.value, 50)
    txs.value = t.txs
  } catch (e: any) {
    errorText.value = e?.message ?? String(e)
  } finally {
    loading.value = false
  }
}

function idem(prefix: string) {
  // простой ключ: prefix-user-ts
  return `${prefix}-${userId.value}-${Date.now()}`
}

async function doTopup() {
  loading.value = true
  errorText.value = null
  try {
    await postTopup(userId.value, topupCurrency.value, Number(topupAmount.value), idem("topup"))
    await refresh()
  } catch (e: any) {
    errorText.value = e?.message ?? String(e)
    loading.value = false
  }
}

async function doTransfer() {
  loading.value = true
  errorText.value = null
  try {
    await postTransfer(
      userId.value,
      transferTo.value,
      transferCurrency.value,
      Number(transferAmount.value),
      idem("xfer"),
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
  <div class="page">
    <header class="header">
      <div class="brand">
        <div class="logo-dot"></div>
        <div class="title">Uzum Wallet MVP</div>
      </div>

      <div class="userbox">
        <div class="label">User ID</div>
        <input class="input" v-model="userId" />
        <button class="btn" :disabled="loading" @click="refresh">Refresh</button>
      </div>
    </header>

    <main class="grid">
      <section class="card">
        <div class="card-title">Balances</div>

        <div class="muted" v-if="balances.length === 0">No balances yet</div>

        <div class="balances" v-else>
          <div class="balance" v-for="b in balances" :key="b.currency">
            <div class="bal-cur">{{ b.currency }}</div>
            <div class="bal-num">{{ money(b.available_minor) }}</div>
            <div class="bal-sub">hold: {{ money(b.hold_minor) }}</div>
          </div>
        </div>

        <div class="total">
          <div class="muted">Total available (sum, minor):</div>
          <div class="total-num">{{ money(totalAvailable) }}</div>
        </div>
      </section>

      <section class="card">
        <div class="card-title">Actions</div>

        <div class="row">
          <div class="muted">Top up</div>
        </div>

        <div class="row">
          <input class="input" v-model="topupCurrency" placeholder="Currency (UZS)" />
          <input class="input" type="number" v-model="topupAmount" placeholder="Amount minor" />
          <button class="btn-primary" :disabled="loading" @click="doTopup">Top up</button>
        </div>

        <div class="divider"></div>

        <div class="row">
          <div class="muted">Transfer</div>
        </div>

        <div class="row">
          <input class="input" v-model="transferTo" placeholder="To user id" />
          <input class="input" v-model="transferCurrency" placeholder="Currency (UZS)" />
          <input class="input" type="number" v-model="transferAmount" placeholder="Amount minor" />
          <button class="btn-primary" :disabled="loading" @click="doTransfer">Send</button>
        </div>

        <div class="hint muted">
          Idempotency-Key генерится автоматически (topup- / xfer-).
        </div>
      </section>

      <section class="card span2">
        <div class="card-title">Transactions</div>

        <div v-if="errorText" class="error">
          {{ errorText }}
        </div>

        <div v-if="txs.length === 0" class="muted">
          No transactions yet
        </div>

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
    </main>
  </div>
</template>
