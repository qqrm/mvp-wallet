<script setup lang="ts">
import { computed, onMounted, ref } from "vue"
import { useRouter } from "vue-router"
import { UCard, UGrid, UGridItem, UInput, USelect, USpace, UText } from "@uzum-tech/ui"
import CoralButton from "../shared/ui/CoralButton.vue"
import { useSettingsStore } from "../app/stores/settings"
import { type ApiError } from "../shared/api/client"
import {
  fetchCurrencies,
  fetchDevUsers,
  postAdminOpenCurrencyAccount,
  postAdminCreateUser,
  postAdminTopup,
  type CurrencyItem,
  type DevUserItem,
} from "../shared/api/endpoints"
import { notifyError, notifySuccess } from "../shared/ui/notifications"

const settings = useSettingsStore()
const router = useRouter()

const users = ref<DevUserItem[]>([])
const currencies = ref<CurrencyItem[]>([])
const isLoading = ref(false)
const errorMessage = ref<string | null>(null)

const userOptions = computed(() =>
  users.value.map((u) => ({ label: `${u.display_name} (${u.user_id})`, value: u.user_id })),
)
const currencyOptions = computed(() =>
  currencies.value.map((c) => ({ label: c.code, value: c.code })),
)

const minorUnitsByCurrency = computed(() => new Map(currencies.value.map((c) => [c.code, c.minor_units])))

const resolveUserLabel = (userId: string) => {
  const found = users.value.find((u) => u.user_id === userId)
  return found ? `${found.display_name} (${found.user_id})` : userId
}

const parseMinorAmount = (value: string, minorUnits: number): number | null => {
  const trimmedRaw = value.trim()
  if (!trimmedRaw) return null
  if (trimmedRaw.startsWith("-")) return null

  const normalized = trimmedRaw.startsWith(".") ? `0${trimmedRaw}` : trimmedRaw
  if (!/^\d*\.?\d*$/.test(normalized)) return null
  if (normalized === ".") return null
  if (minorUnits === 0 && normalized.includes(".")) return null

  const [wholeRaw = "", fractionRaw = ""] = normalized.split(".")
  if (wholeRaw === "" && fractionRaw === "") return null
  if (fractionRaw.length > minorUnits) return null

  const whole = wholeRaw === "" ? "0" : wholeRaw
  if (minorUnits === 0) {
    const n = Number(whole)
    return Number.isFinite(n) ? n : null
  }

  const fraction = fractionRaw.padEnd(minorUnits, "0")
  const combined = `${whole}${fraction}`
  const n = Number(combined)
  if (!Number.isFinite(n)) return null
  if (!Number.isSafeInteger(n)) return null
  return n
}

const loadUsers = async () => {
  isLoading.value = true
  errorMessage.value = null
  try {
    const [userRes, curRes] = await Promise.all([
      fetchDevUsers({ baseUrl: settings.apiBaseUrl }),
      fetchCurrencies({ baseUrl: settings.apiBaseUrl }),
    ])
    users.value = userRes.users
    currencies.value = curRes.items

    // Reasonable defaults for form selects.
    if (!createAccountUserId.value && users.value.length) createAccountUserId.value = users.value[0].user_id
    if (!topupUserId.value && users.value.length) topupUserId.value = users.value[0].user_id

    if (!createAccountCurrency.value && currencies.value.length) createAccountCurrency.value = currencies.value[0].code
    if (!topupCurrency.value && currencies.value.length) topupCurrency.value = currencies.value[0].code
  } catch (error) {
    errorMessage.value = (error as ApiError)?.message ?? "Unable to load users."
  } finally {
    isLoading.value = false
  }
}

const openWallet = (userId: string) => {
  void router.push({ path: "/wallet", query: { as: userId } })
}

onMounted(() => {
  void loadUsers()
})

// -------------------- Admin actions --------------------

const newUserId = ref("")
const createUser = async () => {
  const user_id = newUserId.value.trim()
  if (!user_id) return
  isLoading.value = true
  try {
    const resp = await postAdminCreateUser({ user_id }, { baseUrl: settings.apiBaseUrl })
    notifySuccess(resp.created ? `User ${resp.user_id} created.` : `User ${resp.user_id} already exists.`)
    newUserId.value = ""
    await loadUsers()
  } catch (error) {
    notifyError((error as ApiError)?.message ?? "Unable to create user.")
  } finally {
    isLoading.value = false
  }
}

const createAccountUserId = ref("")
const createAccountCurrency = ref("")
const createAccount = async () => {
  const owner_user_id = createAccountUserId.value.trim()
  const currency = createAccountCurrency.value.trim()
  if (!owner_user_id || !currency) return
  isLoading.value = true
  try {
    const resp = await postAdminOpenCurrencyAccount(
      owner_user_id,
      { currency },
      { baseUrl: settings.apiBaseUrl },
    )
    notifySuccess(
      resp.opened
        ? `Currency account opened for ${resolveUserLabel(resp.user_id)} (${resp.currency}).`
        : `Currency account already exists for ${resolveUserLabel(resp.user_id)} (${resp.currency}).`,
    )
  } catch (error) {
    notifyError((error as ApiError)?.message ?? "Unable to create account.")
  } finally {
    isLoading.value = false
  }
}

const topupUserId = ref("")
const topupCurrency = ref("")
const topupAmount = ref("")
const runTopup = async () => {
  const user_id = topupUserId.value.trim()
  const currency = topupCurrency.value.trim()
  if (!user_id || !currency) return
  const minorUnits = minorUnitsByCurrency.value.get(currency) ?? 2
  const amount_minor = parseMinorAmount(topupAmount.value, minorUnits)
  if (amount_minor === null || amount_minor <= 0) {
    notifyError("Invalid amount.")
    return
  }

  const idem =
    typeof crypto !== "undefined" && "randomUUID" in crypto
      ? crypto.randomUUID()
      : `web-${Date.now()}-${Math.random()}`

  isLoading.value = true
  try {
    const resp = await postAdminTopup({ user_id, currency, amount_minor }, idem, { baseUrl: settings.apiBaseUrl })
    notifySuccess(`Topup posted for ${resolveUserLabel(user_id)}: ${resp.tx_id}`)
    topupAmount.value = ""
  } catch (error) {
    notifyError((error as ApiError)?.message ?? "Unable to post topup.")
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <div class="admin">
    <div class="admin-header">
      <div>
        <h1 class="page-title">Admin</h1>
        <p class="page-subtitle">Local admin tools (no auth on localhost).</p>
      </div>
    </div>

    <UGrid :cols="3" :x-gap="16" :y-gap="16">
      <UGridItem>
        <UCard title="Users">
          <USpace vertical :size="12">
            <div class="card-head">
              <UText depth="3">Select a user to open their wallet.</UText>
              <CoralButton test-id="admin-refresh" :disabled="isLoading" @click="loadUsers">Refresh</CoralButton>
            </div>
            <div v-if="errorMessage" class="error-text">{{ errorMessage }}</div>
            <div v-else-if="isLoading" class="muted">Loading users...</div>
            <div v-else-if="!users.length" class="muted">No users found.</div>
            <ul v-else class="user-list">
              <li v-for="user in users" :key="user.user_id" class="user-row">
                <div class="user-meta">
                  <div class="user-name">{{ user.display_name }}</div>
                  <div class="user-id">{{ user.user_id }}</div>
                </div>
                <CoralButton test-id="admin-open-wallet" @click="openWallet(user.user_id)">Open wallet</CoralButton>
              </li>
            </ul>

            <div class="form-row">
              <UInput v-model:value="newUserId" data-testid="admin-new-user" placeholder="New user id (e.g. u11)" />
              <CoralButton
                test-id="admin-create-user"
                :disabled="isLoading || !newUserId.trim()"
                @click="createUser"
              >
                Create user
              </CoralButton>
            </div>
          </USpace>
        </UCard>
      </UGridItem>

      <UGridItem>
        <UCard title="Accounts">
          <USpace vertical :size="12">
            <UText depth="3">Create currency accounts for a user.</UText>

            <div class="form-grid">
              <USelect
                v-model:value="createAccountUserId"
                :options="userOptions"
                data-testid="admin-create-account-user"
                placeholder="User"
              />
              <USelect
                v-model:value="createAccountCurrency"
                :options="currencyOptions"
                data-testid="admin-create-account-currency"
                placeholder="Currency"
              />
            </div>

            <CoralButton
              test-id="admin-create-account"
              :disabled="isLoading || !createAccountUserId.trim() || !createAccountCurrency.trim()"
              @click="createAccount"
            >
              Create account
            </CoralButton>
          </USpace>
        </UCard>
      </UGridItem>

      <UGridItem>
        <UCard title="Topup">
          <USpace vertical :size="12">
            <UText depth="3">Post topup to a user account.</UText>

            <div class="form-grid">
              <USelect
                v-model:value="topupUserId"
                :options="userOptions"
                data-testid="admin-topup-user"
                placeholder="User"
              />
              <USelect
                v-model:value="topupCurrency"
                :options="currencyOptions"
                data-testid="admin-topup-currency"
                placeholder="Currency"
              />
              <UInput
                v-model:value="topupAmount"
                data-testid="admin-topup-amount"
                placeholder="Amount (major, e.g. 12.34)"
              />
            </div>

            <CoralButton
              test-id="admin-topup"
              :disabled="isLoading || !topupUserId.trim() || !topupCurrency.trim() || !topupAmount.trim()"
              @click="runTopup"
            >
              Run topup
            </CoralButton>
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

.card-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.user-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.user-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 12px;
  border-radius: 12px;
  border: 1px solid var(--border);
  background: var(--surface-2);
}

.user-meta {
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.user-name {
  font-size: 13px;
  font-weight: 800;
}

.user-id {
  font-size: 12px;
  color: var(--text-muted);
}

.form-row {
  display: grid;
  grid-template-columns: 1fr auto;
  gap: 12px;
  align-items: center;
}

.form-grid {
  display: grid;
  grid-template-columns: 1fr;
  gap: 10px;
}

.error-text {
  color: var(--danger);
  font-weight: 700;
}

.muted {
  color: var(--text-muted);
}
</style>
