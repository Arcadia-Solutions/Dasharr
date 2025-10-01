import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type Indexer = components['schemas']['Indexer']

export type IndexerEnriched = components['schemas']['IndexerEnriched']

export type UpdatedIndexer = components['schemas']['UpdatedIndexer']

export type AuthItem = components['schemas']['AuthItem']

export const editIndexer = async (indexer: UpdatedIndexer): Promise<Indexer> => {
  return (await api.put<Indexer>('/indexers', indexer)).data
}

export const toggleIndexer = async (id: number) => {
  return (await api.put(`/indexers/${id}/toggle`)).data
}

export const getIndexerAuthData = async (id: number): Promise<{ [key: string]: AuthItem }> => {
  return (await api.get<{ [key: string]: AuthItem }>(`/indexers/${id}/auth-data`)).data
}

export const getIndexers = async (): Promise<Indexer[]> => {
  return (await api.get<Indexer[]>('/indexers')).data
}

export const getIndexersEnriched = async (
  only_with_available_data: boolean,
): Promise<IndexerEnriched[]> => {
  return (
    await api.get<IndexerEnriched[]>(
      `/indexers/enriched?only_with_available_data=${only_with_available_data}`,
    )
  ).data
}
