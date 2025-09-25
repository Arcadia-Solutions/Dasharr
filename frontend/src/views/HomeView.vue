<template>
  <main>
    <SearchForm @gotResults="gotResults" />
    <IndexerStatsDetail v-if="userStats" :userStats />
  </main>
</template>
<script setup lang="ts">
import IndexerStatsDetail from '@/components/user-stats/IndexerStatsDetail.vue'
import SearchForm from '@/components/user-stats/SearchForm.vue'
import type { UserProfileVec } from '@/services/api/userStatsService'
import { ref } from 'vue'

const userStats = ref<UserProfileVec>()

const gotResults = (stats: UserProfileVec) => {
  userStats.value = stats
}
</script>
