<template>
  <main style="width: 100%">
    <SearchForm @gotResults="gotResults" @selectedValuesUpdated="(val) => (selectedValues = val)" />
    <IndexerStatsIncreases v-if="userStats" :userStats :selectedValues style="margin-top: 40px" />
    <IndexerStatsEvolutions v-if="userStats && indexerMetadata" :userStats :indexerMetadata :selectedValues style="margin-top: 40px" />
  </main>
</template>
<script setup lang="ts">
import IndexerStatsEvolutions from '@/components/user-stats/IndexerStatsEvolutions.vue'
import IndexerStatsIncreases from '@/components/user-stats/IndexerStatsIncreases.vue'
import SearchForm from '@/components/user-stats/SearchForm.vue'
import type { UserProfileScrapedVec, MultiIndexerUserStats } from '@/services/api/userStatsService'
import type { IndexerEnriched } from '@/services/api/indexerService'
import { ref } from 'vue'

const userStats = ref<MultiIndexerUserStats>()
const indexerMetadata = ref<Record<number, IndexerEnriched>>({})
const selectedValues = ref<(keyof UserProfileScrapedVec)[]>(['uploaded', 'downloaded'])

const gotResults = (stats: MultiIndexerUserStats, metadata: Record<number, IndexerEnriched>) => {
  userStats.value = stats
  indexerMetadata.value = metadata
}
</script>
