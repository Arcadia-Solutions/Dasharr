import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type GetUserStatsQuery = {
  indexer_ids: number[]
  date_from: string
  date_to: string
}

export type UserProfileVec = components['schemas']['UserProfileVec']

export type UserProfileScrapedVec = components['schemas']['UserProfileScrapedVec']

export type MultiIndexerUserStats = Record<number, UserProfileVec>

export const getUserStats = async (query: GetUserStatsQuery): Promise<MultiIndexerUserStats> => {
  const params = new URLSearchParams()
  query.indexer_ids.forEach((id) => {
    params.append('indexer_ids', id.toString())
  })
  params.append('date_from', query.date_from)
  params.append('date_to', query.date_to)
  return (await api.get<MultiIndexerUserStats>(`/user-stats?${params.toString()}`)).data
}

export const scrapeUserStats = async () => {
  return (await api.get('/user-stats/scrape')).data
}
