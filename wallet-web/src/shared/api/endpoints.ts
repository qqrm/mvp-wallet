import { apiRequest } from "./client"

export const fetchHealth = async (baseUrl?: string): Promise<{ status: string }> => {
  const response = await apiRequest<string>({
    baseUrl,
    path: "/health",
    method: "GET",
  })

  const status = typeof response === "string" ? response : "ok"
  return { status }
}

export type PlaceholderEndpoint = {
  name: string
  description: string
}

export const placeholderEndpoints: PlaceholderEndpoint[] = [
  { name: "GET /v1/accounts", description: "List accounts for the authenticated user." },
  { name: "GET /v1/transactions/:tx_id", description: "Fetch a transaction receipt." },
]

export type CurrencyItem = {
  code: string
  minor_units: number
}

export type ListCurrenciesResponse = {
  items: CurrencyItem[]
}

export type BalanceItem = {
  currency: string
  available_minor: number
  hold_minor: number
}

export type ListBalancesResponse = {
  balances: BalanceItem[]
}

export type TxItem = {
  tx_id: string
  tx_type: string
  state: string
  currency: string
  amount_minor: number
  description: string
}

export type ListTxsResponse = {
  txs: TxItem[]
}

export type TransferRequest = {
  to_user_id: string
  currency: string
  amount_minor: number
}

export const fetchCurrencies = async (baseUrl?: string, headers?: HeadersInit): Promise<ListCurrenciesResponse> =>
  apiRequest<ListCurrenciesResponse>({
    baseUrl,
    path: "/v1/currencies",
    method: "GET",
    headers,
  })

export const fetchWalletBalances = async (
  baseUrl: string | undefined,
  userId: string,
  headers?: HeadersInit,
): Promise<ListBalancesResponse> =>
  apiRequest<ListBalancesResponse>({
    baseUrl,
    path: `/v1/wallet/${encodeURIComponent(userId)}/balances`,
    method: "GET",
    headers,
  })

export const fetchWalletTxs = async (
  baseUrl: string | undefined,
  userId: string,
  limit: number,
  headers?: HeadersInit,
): Promise<ListTxsResponse> =>
  apiRequest<ListTxsResponse>({
    baseUrl,
    path: `/v1/wallet/${encodeURIComponent(userId)}/txs?limit=${limit}`,
    method: "GET",
    headers,
  })

export const postWalletTransfer = async (
  baseUrl: string | undefined,
  userId: string,
  body: TransferRequest,
  headers?: HeadersInit,
): Promise<unknown> =>
  apiRequest<unknown>({
    baseUrl,
    path: `/v1/wallet/${encodeURIComponent(userId)}/transfer`,
    method: "POST",
    headers,
    jsonBody: body,
  })
