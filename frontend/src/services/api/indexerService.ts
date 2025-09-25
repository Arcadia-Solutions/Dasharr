import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type Indexer = components['schemas']['Indexer']

export type UpdatedIndexer = components['schemas']['UpdatedIndexer']

export type AuthItem = components['schemas']['AuthItem']

export const editIndexer = async (indexer: UpdatedIndexer): Promise<Indexer> => {
  return (await api.put<Indexer>('/indexers', indexer)).data
}

export const toggleIndexer = async (id: number) => {
  return (await api.put(`/indexers/${id}/toggle`)).data
}

export const getIndexers = async (): Promise<Indexer[]> => {
  return (await api.get<Indexer[]>('/indexers')).data
}
