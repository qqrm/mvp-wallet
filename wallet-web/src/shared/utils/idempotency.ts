import { toBase64Url } from "./base64url"

const randomBytes = (size: number): Uint8Array => {
  const bytes = new Uint8Array(size)
  if (typeof crypto !== "undefined" && "getRandomValues" in crypto) {
    crypto.getRandomValues(bytes)
  } else {
    for (let i = 0; i < size; i += 1) {
      bytes[i] = Math.floor(Math.random() * 256)
    }
  }
  return bytes
}

export const createIdempotencyKey = (prefix = "req"): string => {
  const suffix = toBase64Url(randomBytes(12))
  return `${prefix}-${suffix}`
}
