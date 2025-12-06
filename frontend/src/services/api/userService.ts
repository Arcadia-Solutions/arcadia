import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type Peer = components['schemas']['Peer']

export type UserLite = components['schemas']['UserLite']

export type User = components['schemas']['User']

export type PublicUser = components['schemas']['PublicUser']

export type Profile = components['schemas']['Profile'] & { user: User }

export type PublicProfile = components['schemas']['PublicProfile']

export const getMe = async (): Promise<Profile> => {
  return (await api.get<Profile>('/users/me')).data
}

export const getUser = async (userId: number): Promise<PublicProfile> => {
  return (await api.get<PublicProfile>(`/users?id=${userId}`)).data
}

export type UserCreatedUserWarning = components['schemas']['UserCreatedUserWarning']

export type UserWarning = components['schemas']['UserWarning']

export const warnUser = async (warning: UserCreatedUserWarning): Promise<UserWarning> => {
  return (await api.post<UserWarning>('/users/warn', warning)).data
}

export type EditedUser = components['schemas']['EditedUser']

export const editUser = async (editedUser: EditedUser) => {
  return (await api.put('/users', editedUser)).data
}

export type SentInvitation = components['schemas']['SentInvitation']

export type Invitation = components['schemas']['Invitation']

export const sendInvitation = async (invitation: SentInvitation) => {
  return (await api.post('/invitations', invitation)).data
}

export type UserSettings = components['schemas']['UserSettings']

export const updateUserSettings = async (settings: UserSettings) => {
  return (await api.put('/users/settings', settings)).data
}

export const getUserSettings = async () => {
  return (await api.get<UserSettings>('/users/settings')).data
}
