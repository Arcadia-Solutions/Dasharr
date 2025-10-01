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
import type { UserProfileScrapedVec, UserProfileVec } from '@/services/api/userStatsService'
import ContentContainer from '../ContentContainer.vue'
import { bytesToReadable } from '@/services/helpers'

const props = defineProps<{
  userStats: UserProfileVec
  selectedValues: (keyof UserProfileScrapedVec)[]
}>()

const postProcessStat = (value: keyof UserProfileScrapedVec) => {
  let result: number | string =
    ((props.userStats.profile[value]?.[props.userStats.profile[value].length - 1] as number) ?? 0) - ((props.userStats.profile[value]?.[0] as number) ?? 0)
  switch (value) {
    case 'uploaded':
    case 'downloaded':
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
  > * {
    margin: 0 5px;
  }
  .item {
    display: flex;
    flex-direction: column;
    align-items: center;
    .value {
      font-weight: bold;
    }
  }
}
</style>
