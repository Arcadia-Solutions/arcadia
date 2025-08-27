import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type Peer = components['schemas']['Peer']

export type UserLite = components['schemas']['UserLite']

// TODO: update when we can deserialize the settings field to a rust struct and get the type for
// this field generated automatically
// https://github.com/launchbadge/sqlx/issues/3153#issuecomment-2798756953
type UserSettings = {
  settings: {
    site_appearance: {
      item_detail_layout: 'header' | 'sidebar_right' | 'sidebar_left'
    }
  }
}
export type User = components['schemas']['User'] & UserSettings

export type PublicUser = components['schemas']['PublicUser']

export type Profile = components['schemas']['Profile'] & { user: User }

export type PublicProfile = components['schemas']['PublicProfile']

export const getMe = async (): Promise<Profile> => {
  return (await api.get<Profile>('/me')).data
}

export const getUser = async (userId: number): Promise<PublicProfile> => {
  return (await api.get<PublicProfile>(`/user?id=${userId}`)).data
}

export type UserCreatedUserWarning = components['schemas']['UserCreatedUserWarning']

export type UserWarning = components['schemas']['UserWarning']

export const warnUser = async (warning: UserCreatedUserWarning): Promise<UserWarning> => {
  return (await api.post<UserWarning>('/user/warn', warning)).data
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
