<template>
  <main style="width: 100%">
    <SearchForm @gotResults="gotResults" @selectedValuesUpdated="(val) => (selectedValues = val)" />
    <IndexerStatsOverview v-if="userStats" :userStats :selectedValues style="margin-top: 40px" />
    <IndexerStatsGraphs v-if="userStats" :userStats :selectedValues style="margin-top: 40px" />
  </main>
</template>
<script setup lang="ts">
import IndexerStatsGraphs from '@/components/user-stats/IndexerStatsGraphs.vue'
import IndexerStatsOverview from '@/components/user-stats/IndexerStatsOverview.vue'
import SearchForm from '@/components/user-stats/SearchForm.vue'
import type { UserProfileScrapedVec, UserProfileVec } from '@/services/api/userStatsService'
import { ref } from 'vue'

const userStats = ref<UserProfileVec>()
const selectedValues = ref<(keyof UserProfileScrapedVec)[]>(['uploaded', 'downloaded'])

const gotResults = (stats: UserProfileVec) => {
  userStats.value = stats
}
</script>
