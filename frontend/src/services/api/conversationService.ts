import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type UserCreatedConversation = components['schemas']['UserCreatedConversation']

export type Conversation = components['schemas']['Conversation']

export const postConversation = async (conversation: UserCreatedConversation) => {
  return (await api.post<Conversation>('/conversation', conversation)).data
}

export type ConversationHierarchy = components['schemas']['ConversationHierarchy']

export type ConversationMessageHierarchy = components['schemas']['ConversationMessageHierarchy']

export const getConversation = async (conversationId: number) => {
  return (await api.get<ConversationHierarchy>(`/conversations?id=${conversationId}`)).data
}

export type ConversationsOverview = components['schemas']['ConversationsOverview']

export type ConversationOverview = components['schemas']['ConversationOverview']

export const getConversations = async (): Promise<ConversationsOverview> => {
  return (await api.get<ConversationsOverview>('/users/conversations')).data
}

export type UserCreatedConversationMessage = components['schemas']['UserCreatedConversationMessage']

export type ConversationMessage = components['schemas']['ConversationMessage']

export const postConversationMessage = async (message: UserCreatedConversationMessage) => {
  return (await api.post<ConversationMessage>('/conversations/messages', message)).data
}
