import type { components } from '@/api-schema/schema.js'
import api from './api.ts'

export type Login = components['schemas']['Login']

export type Register = components['schemas']['Register']

export type LoginResponse = components['schemas']['LoginResponse']

export const login = async (form: Login): Promise<LoginResponse> => {
  return (await api.post<LoginResponse>('/auth/login', form)).data
}

export const register = async (form: Register, invitationKey: string) => {
  return (await api.post(`/auth/register?invitation_key=${invitationKey}`, form)).data
}
