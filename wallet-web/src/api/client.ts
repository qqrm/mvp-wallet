import { getAdminToken, getUserToken } from "./auth"

export type BalanceItem = {
  currency: string
  available_minor: number
  hold_minor: number
}

export type TxItem = {
  tx_id: string
  tx_type: string
  state: string
  currency: string
  amount_minor: number
  created_at: string
  posted_at: string
  description: string
}

export type ListBalancesResponse = {
  balances: BalanceItem[]
}

export type ListTxsResponse = {
  txs: TxItem[]
}

export type PostOpResponse = {
  tx_id: string
  state: string
  currency: string
  amount_minor: number
}

export type CurrenciesResponse = {
  currencies: { code: string; minor_units: number }[]
}

export type AdminCreateUserRequest = { user_id: string }
export type AdminCreateUserResponse = { user_id: string; created: boolean }

export type AdminOpenCurrencyAccountRequest = { currency: string }
export type AdminOpenCurrencyAccountResponse = { user_id: string; currency: string; opened: boolean }

export type AdminCreateAccountRequest = { owner_user_id: string; currency: string; label: string }
export type AdminCreateAccountResponse = {
  account_id: string
  status: string
  currency: string
  owner_user_id: string
  created_at: string
}

export type AdminCloseAccountResponse = { account_id: string; status: string; closed_at: string }

export type AdminTopupRequest = { user_id: string; currency: string; amount_minor: number }
export type AdminHoldRequest = { user_id: string; currency: string; amount_minor: number }

// if empty, we rely on Vite proxy with relative URLs
const API_BASE = (import.meta.env.VITE_API_BASE?.toString().trim() || "").replace(/\/$/, "")

type AuthKind = "admin" | "user" | "none"

async function httpJson<T>(path: string, init?: RequestInit, auth: AuthKind = "none"): Promise<T> {
  const url = `${API_BASE}${path}`

  const headers = new Headers(init?.headers ?? {})
  if (!headers.has("Content-Type") && init?.body != null) {
    headers.set("Content-Type", "application/json")
  }

  if (auth === "admin") {
    headers.set("Authorization", `Bearer ${getAdminToken()}`)
  } else if (auth === "user") {
    const t = getUserToken()
    if (t) headers.set("Authorization", `Bearer ${t}`)
  }

  const res = await fetch(url, { ...init, headers })
  if (!res.ok) {
    let msg = `${res.status} ${res.statusText}`
    try {
      const j = await res.json()
      if (j?.error) msg = j.error
    } catch {
      // ignore
    }
    throw new Error(msg)
  }

  if (res.status === 204) return undefined as T
  return (await res.json()) as T
}

export function idempotencyKey(prefix: string): string {
  return `${prefix}-${Date.now()}-${Math.random().toString(16).slice(2)}`
}

// ---------- Public ----------
export async function getCurrencies(): Promise<CurrenciesResponse> {
  return await httpJson(`/v1/currencies`, undefined, "none")
}

// ---------- User endpoints ----------
export async function getWalletBalances(userId: string): Promise<ListBalancesResponse> {
  return await httpJson(`/v1/wallet/${encodeURIComponent(userId)}/balances`, undefined, "user")
}

export async function getWalletTxs(userId: string, limit = 50): Promise<ListTxsResponse> {
  return await httpJson(`/v1/wallet/${encodeURIComponent(userId)}/txs?limit=${limit}`, undefined, "user")
}

export async function postWalletTransfer(
  userId: string,
  toUserId: string,
  currency: string,
  amountMinor: number,
  idem: string,
): Promise<PostOpResponse> {
  return await httpJson(
    `/v1/wallet/${encodeURIComponent(userId)}/transfer`,
    {
      method: "POST",
      headers: {
        "Idempotency-Key": idem,
      },
      body: JSON.stringify({ to_user_id: toUserId, currency, amount_minor: amountMinor }),
    },
    "user",
  )
}

// ---------- Admin endpoints ----------
export async function adminCreateUser(req: AdminCreateUserRequest): Promise<AdminCreateUserResponse> {
  return await httpJson(`/v1/admin/users`, { method: "POST", body: JSON.stringify(req) }, "admin")
}

export async function adminOpenCurrencyAccount(
  userId: string,
  req: AdminOpenCurrencyAccountRequest,
): Promise<AdminOpenCurrencyAccountResponse> {
  return await httpJson(
    `/v1/admin/users/${encodeURIComponent(userId)}/accounts`,
    { method: "POST", body: JSON.stringify(req) },
    "admin",
  )
}

export async function adminCreateAccount(req: AdminCreateAccountRequest): Promise<AdminCreateAccountResponse> {
  return await httpJson(`/v1/admin/accounts`, { method: "POST", body: JSON.stringify(req) }, "admin")
}

export async function adminCloseAccount(accountId: string, idem: string): Promise<AdminCloseAccountResponse> {
  // close endpoint does not use idempotency in backend today; but we allow passing it anyway (ignored)
  return await httpJson(
    `/v1/admin/accounts/${encodeURIComponent(accountId)}/close`,
    { method: "POST", headers: { "Idempotency-Key": idem } },
    "admin",
  )
}

export async function adminTopup(req: AdminTopupRequest, idem: string): Promise<PostOpResponse> {
  return await httpJson(
    `/v1/admin/topup`,
    { method: "POST", headers: { "Idempotency-Key": idem }, body: JSON.stringify(req) },
    "admin",
  )
}

export async function adminHoldAuthorize(req: AdminHoldRequest, idem: string): Promise<PostOpResponse> {
  return await httpJson(
    `/v1/admin/holds`,
    { method: "POST", headers: { "Idempotency-Key": idem }, body: JSON.stringify(req) },
    "admin",
  )
}

export async function adminHoldCapture(txId: string, idem: string): Promise<PostOpResponse> {
  return await httpJson(
    `/v1/admin/holds/${encodeURIComponent(txId)}/capture`,
    { method: "POST", headers: { "Idempotency-Key": idem } },
    "admin",
  )
}

export async function adminHoldCancel(txId: string, idem: string): Promise<PostOpResponse> {
  return await httpJson(
    `/v1/admin/holds/${encodeURIComponent(txId)}/cancel`,
    { method: "POST", headers: { "Idempotency-Key": idem } },
    "admin",
  )
}

export async function adminRefundPayment(txId: string, idem: string): Promise<any> {
  return await httpJson(
    `/v1/admin/payments/${encodeURIComponent(txId)}/refund`,
    { method: "POST", headers: { "Idempotency-Key": idem } },
    "admin",
  )
}

export async function adminUserBalances(userId: string): Promise<ListBalancesResponse> {
  return await httpJson(`/v1/admin/users/${encodeURIComponent(userId)}/balances`, undefined, "admin")
}

export async function adminUserTxs(userId: string, limit = 50): Promise<ListTxsResponse> {
  return await httpJson(`/v1/admin/users/${encodeURIComponent(userId)}/txs?limit=${limit}`, undefined, "admin")
}
