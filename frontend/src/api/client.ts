import { QueryClient } from '@tanstack/react-query'
import { Api, type HttpResponse } from '@/api/Api'

export const queryClient = new QueryClient()

function createApiClient() {
  return new Api({
    baseUrl: window.origin,
    customFetch: function (input, init) {
      const token = getAuthToken()
      const headers: Record<string, string> = token ? { Authorization: token } : {}

      return fetch(input, {
        ...init,
        headers: {
          ...headers,
          ...init?.headers,
        },
      })
    },
  })
}

export function handleQuery<T>(fn: () => Promise<HttpResponse<T>>) {
  return async function exec(): Promise<T> {
    return fn()
      .then((response) => response.data)
      .catch((response) => {
        const err = new HttpError(response.error.message, response.status)
        console.error(err.toString())
        throw err
      })
  }
}

export async function handleMutation<T>(
  fn: () => Promise<HttpResponse<T, HttpError>>,
) {
  return fn()
    .then((response) => response.data)
    .catch((response) => {
      const err = new HttpError(response.error.message, response.status)
      console.error(err.toString())
      throw err
    })
}

export class HttpError extends Error {
  status: number

  constructor(message: string, status: number, options?: ErrorOptions) {
    super(message, options)
    this.status = status
  }
}

HttpError.prototype.toString = function () {
  return `HttpError { message=${this.message}, status=${this.status} }`
}

export const { api } = createApiClient()

export function getAuthToken() {
  return localStorage.getItem(SESSION_KEY)
}

export function setAuthToken(token: string) {
  localStorage.setItem(SESSION_KEY, token)
}

const SESSION_KEY = 'session_token'
