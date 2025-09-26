<template>
  <main style="width: 100%">
    <SearchForm @gotResults="gotResults" />
    <div class="wrapper-center">
      <MultiSelect
        v-model="selectedValues"
        :options="displayableValues"
        filter
        display="chip"
        placeholder="Charts to display"
      />
    </div>
    <IndexerStatsOverview v-if="userStats" :userStats :selectedValues style="margin-top: 40px" />
    <IndexerStatsGraphs v-if="userStats" :userStats :selectedValues style="margin-top: 40px" />
  </main>
</template>
<script setup lang="ts">
import IndexerStatsGraphs from '@/components/user-stats/IndexerStatsGraphs.vue'
import IndexerStatsOverview from '@/components/user-stats/IndexerStatsOverview.vue'
import SearchForm from '@/components/user-stats/SearchForm.vue'
import { MultiSelect } from 'primevue'
import type { UserProfileScrapedVec, UserProfileVec } from '@/services/api/userStatsService'
import { ref } from 'vue'

const userStats = ref<UserProfileVec>()
const displayableValues = ref<(keyof UserProfileScrapedVec)[]>([
  'downloaded',
  'uploaded',
  'bonus_points',
  'leeching',
  'seeding',
  'ratio',
  'uploaded_torrents',
  'snatched',
  'invited',
  'torrent_comments',
])
const selectedValues = ref<(typeof displayableValues.value)[number][]>(['uploaded', 'downloaded'])

const gotResults = (stats: UserProfileVec) => {
  userStats.value = stats
}
</script>
