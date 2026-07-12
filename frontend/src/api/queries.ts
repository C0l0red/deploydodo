import { queryOptions, useQuery } from '@tanstack/react-query'
import { api, handleQuery } from '@/api/client'

export const statusQueryOptions = queryOptions({
  queryKey: ['status'],
  queryFn: handleQuery(api.status),
})

export function useStatusQuery() {
  return useQuery(statusQueryOptions)
}

export const serversQueryOptions = queryOptions({
  queryKey: ['servers'],
  queryFn: handleQuery(api.listServers),
})

export function useServersQuery() {
  return useQuery(serversQueryOptions)
}

export const validateSessionOptions = queryOptions({
  queryKey: ['validateSession'],
  queryFn: handleQuery(api.validateSession),
})
