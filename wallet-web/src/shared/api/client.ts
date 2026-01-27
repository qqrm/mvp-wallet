export type ApiError = {
  status: number
  message: string
  code?: string
  details?: unknown
}

export type ApiRequestOptions = {
  baseUrl?: string
  path: string
  method?: string
  headers?: HeadersInit
  jsonBody?: unknown
  signal?: AbortSignal
}

const normalizeBaseUrl = (url: string) => url.replace(/\/$/, "")

export const resolveApiBaseUrl = (): string => {
  const envBase = import.meta.env.VITE_API_BASE?.toString().trim()
  const envUrl = import.meta.env.VITE_API_URL?.toString().trim()
  return normalizeBaseUrl(envBase || envUrl || "http://127.0.0.1:3000")
}

const parseResponse = async <T>(response: Response): Promise<T> => {
  const contentType = response.headers.get("content-type") ?? ""
  if (contentType.includes("application/json")) {
    return (await response.json()) as T
  }
  const text = await response.text()
  return text as T
}

const parseError = async (response: Response): Promise<ApiError> => {
  let message = `${response.status} ${response.statusText}`
  let code: string | undefined
  let details: unknown

  try {
    const contentType = response.headers.get("content-type") ?? ""
    if (contentType.includes("application/json")) {
      const body = (await response.json()) as { error?: string; code?: string; details?: unknown }
      if (body?.error) message = body.error
      if (body?.code) code = body.code
      if (body?.details) details = body.details
    } else {
      const text = await response.text()
      if (text) message = text
    }
  } catch {
    // ignore parsing errors and use fallback message
  }

  return {
    status: response.status,
    message,
    code,
    details,
  }
}

export const apiRequest = async <T>({
  baseUrl,
  path,
  method = "GET",
  headers,
  jsonBody,
  signal,
}: ApiRequestOptions): Promise<T> => {
  const resolvedBase = normalizeBaseUrl(baseUrl ?? resolveApiBaseUrl())
  const url = `${resolvedBase}${path.startsWith("/") ? path : `/${path}`}`
  const requestHeaders = new Headers(headers)

  if (jsonBody !== undefined) {
    requestHeaders.set("Content-Type", "application/json")
  }

  const response = await fetch(url, {
    method,
    headers: requestHeaders,
    body: jsonBody === undefined ? undefined : JSON.stringify(jsonBody),
    signal,
  })

  if (!response.ok) {
    throw await parseError(response)
  }

  if (response.status === 204) {
    return undefined as T
  }

  return parseResponse<T>(response)
}
