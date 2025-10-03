<template>
  <main style="width: 100%">
    <SearchForm @gotResults="gotResults" @selectedValuesUpdated="(val) => (selectedValues = val)" />
    <IndexerStatsIncreases v-if="userStats" :userStats :selectedValues style="margin-top: 40px" />
    <IndexerStatsEvolutions v-if="userStats" :userStats :selectedValues style="margin-top: 40px" />
  </main>
</template>
<script setup lang="ts">
import IndexerStatsEvolutions from '@/components/user-stats/IndexerStatsEvolutions.vue'
import IndexerStatsIncreases from '@/components/user-stats/IndexerStatsIncreases.vue'
import SearchForm from '@/components/user-stats/SearchForm.vue'
import type { UserProfileScrapedVec, UserProfileVec } from '@/services/api/userStatsService'
import { ref } from 'vue'

const userStats = ref<UserProfileVec>()
const selectedValues = ref<(keyof UserProfileScrapedVec)[]>(['uploaded', 'downloaded'])

const gotResults = (stats: UserProfileVec) => {
  userStats.value = stats
}
</script>
