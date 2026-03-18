<template>
  <main style="width: 100%">
    <SearchForm @gotResults="gotResults" @selectedValuesUpdated="(val) => (selectedValues = val)" @selectedIndexersUpdated="(val) => (indexerNames = val)" />
    <IndexerStatsIncreases v-if="indexerStats" :indexerStats :selectedValues :indexerNames style="margin-top: 40px" />
    <IndexerStatsEvolutions v-if="indexerStats" :indexerStats :selectedValues :indexerNames style="margin-top: 40px" />
  </main>
</template>
<script setup lang="ts">
import IndexerStatsEvolutions from '@/components/user-stats/IndexerStatsEvolutions.vue'
import IndexerStatsIncreases from '@/components/user-stats/IndexerStatsIncreases.vue'
import SearchForm from '@/components/user-stats/SearchForm.vue'
import type { UserProfileScrapedVec, IndexerStats } from '@/services/api-schema'
import { ref } from 'vue'

const indexerStats = ref<IndexerStats[]>()
const selectedValues = ref<(keyof UserProfileScrapedVec)[]>(['uploaded', 'downloaded'])
const indexerNames = ref(new Map<number, string>())

const gotResults = (stats: IndexerStats[]) => {
  indexerStats.value = stats
}
</script>
