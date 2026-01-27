const ADMIN_TOKEN_KEY = "wallet_admin_token"
const USER_TOKEN_KEY = "wallet_user_token"

export function getAdminToken(): string {
  return sessionStorage.getItem(ADMIN_TOKEN_KEY) ?? "admin-dev-token"
}

export function setAdminToken(token: string) {
  sessionStorage.setItem(ADMIN_TOKEN_KEY, token.trim())
}

export function getUserToken(): string {
  return sessionStorage.getItem(USER_TOKEN_KEY) ?? ""
}

export function setUserToken(token: string) {
  sessionStorage.setItem(USER_TOKEN_KEY, token.trim())
}
