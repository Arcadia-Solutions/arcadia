import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type UserCreatedUserApplication = components['schemas']['UserCreatedUserApplication']

export type GetUserApplicationsQuery = components['schemas']['GetUserApplicationsQuery']

export type UserApplication = components['schemas']['UserApplication']

export type UserApplicationStatus = components['schemas']['UserApplicationStatus']

export type UpdateUserApplication = components['schemas']['UpdateUserApplication']

export const postUserApplication = async (application: UserCreatedUserApplication) => {
  return (await api.post('/user-applications', application)).data
}

export const getUserApplications = async (queryParameters: GetUserApplicationsQuery) => {
  return (await api.get('/user-applications', { params: queryParameters })).data
}

export const updateUserApplication = async (update: UpdateUserApplication): Promise<UserApplication> => {
  return (await api.put('/user-applications', update)).data
}
