import { QueryClient } from '@tanstack/react-query'
import { Api, type HttpResponse } from '@/api/Api'

export const queryClient = new QueryClient()

function createApiClient() {
  return new Api({
    baseUrl: '/',
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

export function handleQuery<T>(fn: () => Promise<HttpResponse<T, HttpError>>) {
  return async function exec(): Promise<T> {
    const response = await fn()

    if (response.error) {
      throw { ...response.error, status: response.status }
    }
    return response.data
  }
}

export async function handleMutation<T>(fn: () => Promise<HttpResponse<T, HttpError>>) {
  const response = await fn()

  if (response.error) {
    throw { ...response.error, status: response.status }
  }
  return response.data
}

export interface HttpError {
  message: string
  status: number
}

export const { api } = createApiClient()

export function getAuthToken() {
  return localStorage.getItem(SESSION_KEY)
}

export function setAuthToken(token: string) {
  localStorage.setItem(SESSION_KEY, token)
}

const SESSION_KEY = 'session_token'
