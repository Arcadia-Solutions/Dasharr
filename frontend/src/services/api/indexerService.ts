import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type Indexer = components['schemas']['Indexer']

export type NewIndexer = components['schemas']['NewIndexer']

export type AuthItem = components['schemas']['AuthItem']

export const addIndexer = async (indexer: NewIndexer): Promise<Indexer> => {
  return (await api.post<Indexer>('/indexers', indexer)).data
}
