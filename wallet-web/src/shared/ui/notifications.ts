import type { MessageApi } from "@uzum-tech/ui"

let messageApi: MessageApi | null = null

export const registerMessageApi = (api: MessageApi | null) => {
  messageApi = api
}

export const notifySuccess = (message: string) => {
  messageApi?.success(message)
}

export const notifyError = (message: string) => {
  messageApi?.error(message)
}

export const notifyInfo = (message: string) => {
  messageApi?.info(message)
}
