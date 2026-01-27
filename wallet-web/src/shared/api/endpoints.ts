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
