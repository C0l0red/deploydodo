import { useMutation, type UseMutationOptions } from '@tanstack/react-query'
import { api, handleMutation, type HttpError } from './client'
import type {
  AdminResponse,
  CreateAdminRequest,
  CreateLocalServerRequest,
  CreateLocalServerResponse,
  CreateRemoteServerRequest,
  LoginRequest,
  LoginResponse,
  StartJobResponse,
} from './Api'

type MutationOptions<Req, Res> = Omit<UseMutationOptions<Res, HttpError, Req>, 'mutationFn'>

export function useCreateLocalServer(
  options?: MutationOptions<CreateLocalServerRequest, CreateLocalServerResponse>,
) {
  return useMutation({
    ...options,
    mutationFn: async (request: CreateLocalServerRequest) =>
      handleMutation(() => api.createLocalServer(request)),
  })
}

export function useCreateAdmin(options?: MutationOptions<CreateAdminRequest, AdminResponse>) {
  return useMutation({
    ...options,
    mutationFn: async (request: CreateAdminRequest) =>
      handleMutation(() => api.createAdmin(request)),
  })
}

export function useCreateRemoteServer(
  options?: MutationOptions<CreateRemoteServerRequest, StartJobResponse>,
) {
  return useMutation({
    ...options,
    mutationFn: async (request: CreateRemoteServerRequest) =>
      handleMutation(() => api.createRemoteServer(request)),
  })
}

export function useLogin(
  options?: MutationOptions<LoginRequest, LoginResponse>,
) {
  return useMutation({
    ...options,
    mutationFn: async (request: LoginRequest) =>
      handleMutation(() => api.login(request)),
  })
}
