<template>
  <div>
    <div class="wrapper-center title">Increase on the selected period</div>
    <div v-for="indexer in indexerStats" :key="indexer.indexer_id">
      <div v-if="indexerStats.length > 1" class="wrapper-center indexer-label">
        {{ indexerNames.get(indexer.indexer_id) ?? `Indexer ${indexer.indexer_id}` }}
      </div>
      <div class="items">
        <ContentContainer v-for="name in selectedValues.filter((v) => indexer.profile[v]?.length)" :key="name" class="item">
          <span class="value">{{ postProcessStat(name, indexer) }}</span>
          <span class="name">{{ name }}</span>
        </ContentContainer>
      </div>
    </div>
  </div>
</template>
<script lang="ts" setup>
import type { UserProfileScrapedVec, IndexerStats } from '@/services/api-schema'
import ContentContainer from '../ContentContainer.vue'
import { bytesToReadable } from '@/services/helpers'

defineProps<{
  indexerStats: IndexerStats[]
  selectedValues: (keyof UserProfileScrapedVec)[]
  indexerNames: Map<number, string>
}>()

const postProcessStat = (value: keyof UserProfileScrapedVec, indexer: IndexerStats) => {
  const raw = indexer.profile[value] as (number | null)[]
  let result: number | string = ((raw[raw.length - 1] as number) ?? 0) - ((raw[0] as number) ?? 0)
  switch (value) {
    case 'uploaded':
    case 'downloaded':
    case 'seed_size':
    case 'uploaded_real':
    case 'downloaded_real':
      result = bytesToReadable(result)
      break
  }
  return result
}
</script>
<style scoped>
.indexer-label {
  font-weight: bold;
  margin-top: 10px;
}
.items {
  display: flex;
  justify-content: center;
  flex-wrap: wrap;
  .item {
    margin: 5px;
    display: flex;
    flex-direction: column;
    align-items: center;
    .value {
      font-weight: bold;
    }
  }
}
</style>
