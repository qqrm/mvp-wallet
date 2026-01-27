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
    user_id: string
    balances: BalanceItem[]
}

export type ListTxsResponse = {
    user_id: string
    txs: TxItem[]
}

export type PostOpResponse = {
    tx_id: string
    state: string
    currency: string
    amount_minor: number
}

const API_BASE =
    import.meta.env.VITE_API_BASE?.toString().trim() || "http://localhost:8080"

async function httpJson<T>(url: string, init?: RequestInit): Promise<T> {
    const res = await fetch(url, init)
    if (!res.ok) {
        let msg = `${res.status} ${res.statusText}`
        try {
            const j = await res.json()
            if (j?.error) msg = j.error
        } catch { }
        throw new Error(msg)
    }
    return (await res.json()) as T
}

export async function getBalances(userId: string): Promise<ListBalancesResponse> {
    return await httpJson(`${API_BASE}/v1/wallet/${encodeURIComponent(userId)}/balances`)
}

export async function getTxs(userId: string, limit = 50): Promise<ListTxsResponse> {
    return await httpJson(
        `${API_BASE}/v1/wallet/${encodeURIComponent(userId)}/txs?limit=${limit}`,
    )
}

export async function postTopup(
    userId: string,
    currency: string,
    amountMinor: number,
    idempotencyKey: string,
): Promise<PostOpResponse> {
    return await httpJson(`${API_BASE}/v1/wallet/${encodeURIComponent(userId)}/topup`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            "Idempotency-Key": idempotencyKey,
        },
        body: JSON.stringify({ currency, amount_minor: amountMinor }),
    })
}

export async function postTransfer(
    userId: string,
    toUserId: string,
    currency: string,
    amountMinor: number,
    idempotencyKey: string,
): Promise<PostOpResponse> {
    return await httpJson(`${API_BASE}/v1/wallet/${encodeURIComponent(userId)}/transfer`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            "Idempotency-Key": idempotencyKey,
        },
        body: JSON.stringify({ to_user_id: toUserId, currency, amount_minor: amountMinor }),
    })
}
