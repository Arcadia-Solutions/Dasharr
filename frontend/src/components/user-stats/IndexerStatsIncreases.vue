<template>
  <div>
    <div class="wrapper-center title">Increase on the selected period</div>
    <div class="items">
      <ContentContainer v-for="name in selectedValues" :key="name" class="item">
        <span class="value">{{ postProcessStat(name) }}</span>
        <span class="name">{{ name }}</span>
      </ContentContainer>
    </div>
  </div>
</template>
<script lang="ts" setup>
import type { UserProfileScrapedVec, MultiIndexerUserStats } from '@/services/api/userStatsService'
import ContentContainer from '../ContentContainer.vue'
import { bytesToReadable } from '@/services/helpers'

const props = defineProps<{
  userStats: MultiIndexerUserStats
  selectedValues: (keyof UserProfileScrapedVec)[]
}>()

const postProcessStat = (value: keyof UserProfileScrapedVec) => {
  let totalIncrease = 0

  Object.values(props.userStats).forEach((userStatsVec) => {
    const profileArray = userStatsVec.profile[value]
    if (profileArray && profileArray.length > 0) {
      const first = (profileArray[0] as number) ?? 0
      const last = (profileArray[profileArray.length - 1] as number) ?? 0
      totalIncrease += last - first
    }
  })

  let result: number | string = totalIncrease
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
