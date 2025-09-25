import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type GetUserStatsQuery = components['schemas']['GetUserStatsQuery']

export type UserProfileVec = components['schemas']['UserProfileVec']

export const getUserStats = async (query: GetUserStatsQuery): Promise<UserProfileVec> => {
  return (await api.get<UserProfileVec>('/user-stats', { params: query })).data
}
