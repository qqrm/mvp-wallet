const encodeByteArray = (bytes: Uint8Array): string => {
  let binary = ""
  bytes.forEach((byte) => {
    binary += String.fromCharCode(byte)
  })
  return btoa(binary)
}

export const toBase64Url = (input: Uint8Array | string): string => {
  const bytes = typeof input === "string" ? new TextEncoder().encode(input) : input
  return encodeByteArray(bytes).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "")
}
