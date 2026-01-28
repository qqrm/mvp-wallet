import { apiRequest } from "./client"

export type ApiCallOptions = {
  baseUrl?: string
  headers?: HeadersInit
}

export const fetchHealth = async (baseUrl?: string): Promise<{ status: string }> => {
  const response = await apiRequest<string>({
    baseUrl,
    path: "/health",
    method: "GET",
  })

  const status = typeof response === "string" ? response : "ok"
  return { status }
}

export type CurrencyItem = {
  code: string
  minor_units: number
}

export type ListCurrenciesResponse = {
  items: CurrencyItem[]
}

export const fetchCurrencies = async (options?: ApiCallOptions): Promise<ListCurrenciesResponse> =>
  apiRequest<ListCurrenciesResponse>({
    baseUrl: options?.baseUrl,
    headers: options?.headers,
    path: "/v1/currencies",
    method: "GET",
  })

export type BalanceItem = {
  currency: string
  available_minor: number
  hold_minor: number
}

export type ListBalancesResponse = {
  balances: BalanceItem[]
}

export const fetchWalletBalances = async (
  userId: string,
  options?: ApiCallOptions,
): Promise<ListBalancesResponse> =>
  apiRequest<ListBalancesResponse>({
    baseUrl: options?.baseUrl,
    headers: options?.headers,
    path: `/v1/wallet/${encodeURIComponent(userId)}/balances`,
    method: "GET",
  })

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

export type ListTxsResponse = {
  txs: TxItem[]
}

export const fetchWalletTxs = async (
  userId: string,
  limit = 50,
  options?: ApiCallOptions,
): Promise<ListTxsResponse> => {
  const params = new URLSearchParams()
  params.set("limit", limit.toString())
  return apiRequest<ListTxsResponse>({
    baseUrl: options?.baseUrl,
    headers: options?.headers,
    path: `/v1/wallet/${encodeURIComponent(userId)}/txs?${params.toString()}`,
    method: "GET",
  })
}

export type TransferRequest = {
  to_user_id: string
  currency: string
  amount_minor: number
}

export type PostOpResponse = {
  tx_id: string
  state: string
  currency: string
  amount_minor: number
}

export const postWalletTransfer = async (
  userId: string,
  body: TransferRequest,
  options?: ApiCallOptions,
): Promise<PostOpResponse> =>
  apiRequest<PostOpResponse>({
    baseUrl: options?.baseUrl,
    headers: options?.headers,
    path: `/v1/wallet/${encodeURIComponent(userId)}/transfer`,
    method: "POST",
    jsonBody: body,
  })

export type PlaceholderEndpoint = {
  name: string
  description: string
}

export const placeholderEndpoints: PlaceholderEndpoint[] = [
  { name: "GET /v1/accounts", description: "List accounts for the authenticated user." },
  { name: "GET /v1/transactions/:tx_id", description: "Fetch a transaction receipt." },
]

export type DevUserItem = {
  user_id: string
  display_name: string
}

export type DevUsersResponse = {
  users: DevUserItem[]
}

export const fetchDevUsers = async (options?: ApiCallOptions): Promise<DevUsersResponse> =>
  apiRequest<DevUsersResponse>({
    baseUrl: options?.baseUrl,
    headers: options?.headers,
    path: "/v1/dev/users",
    method: "GET",
  })

// -------------------- Admin (local dev: no auth) --------------------

export type AdminCreateUserRequest = {
  user_id: string
}

export type AdminCreateUserResponse = {
  user_id: string
  created: boolean
}

export const postAdminCreateUser = async (
  body: AdminCreateUserRequest,
  options?: ApiCallOptions,
): Promise<AdminCreateUserResponse> =>
  apiRequest<AdminCreateUserResponse>({
    baseUrl: options?.baseUrl,
    headers: options?.headers,
    path: "/v1/admin/users",
    method: "POST",
    jsonBody: body,
  })

export type AdminCreateAccountRequest = {
  owner_user_id: string
  currency: string
  label: string
}

export type AdminCreateAccountResponse = {
  account_id: string
  status: string
  currency: string
  owner_user_id: string
  created_at: string
}

export const postAdminCreateAccount = async (
  body: AdminCreateAccountRequest,
  options?: ApiCallOptions,
): Promise<AdminCreateAccountResponse> =>
  apiRequest<AdminCreateAccountResponse>({
    baseUrl: options?.baseUrl,
    headers: options?.headers,
    path: "/v1/admin/accounts",
    method: "POST",
    jsonBody: body,
  })

export type AdminTopupRequest = {
  user_id: string
  currency: string
  amount_minor: number
}

export const postAdminTopup = async (
  body: AdminTopupRequest,
  idempotencyKey: string,
  options?: ApiCallOptions,
): Promise<PostOpResponse> =>
  apiRequest<PostOpResponse>({
    baseUrl: options?.baseUrl,
    headers: {
      ...(options?.headers ?? {}),
      "Idempotency-Key": idempotencyKey,
    },
    path: "/v1/admin/topup",
    method: "POST",
    jsonBody: body,
  })
