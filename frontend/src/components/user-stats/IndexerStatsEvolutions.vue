<template>
  <div id="indexer-stats-detail">
    <div class="wrapper-center title">Evolution on the selected period</div>
    <div class="charts">
      <ContentContainer v-for="value in selectedValues" :key="value">
        <highcharts :options="chartOptions(value)" />
      </ContentContainer>
    </div>
  </div>
</template>
<script lang="ts" setup>
import type { Options } from 'highcharts'
import type { UserProfileVec, UserProfileScrapedVec } from '@/services/api-schema'
import ContentContainer from '../ContentContainer.vue'

const GIB_KEYS: (keyof UserProfileScrapedVec)[] = ['uploaded', 'downloaded', 'seed_size', 'uploaded_real', 'downloaded_real']

const props = defineProps<{
  userStats: UserProfileVec
  selectedValues: (keyof UserProfileScrapedVec)[]
}>()

const seriesColor = getComputedStyle(document.documentElement).getPropertyValue('--p-button-primary-border-color')

function chartOptions(value: keyof UserProfileScrapedVec): Options {
  const isGib = GIB_KEYS.includes(value)
  const raw = props.userStats.profile[value] as (number | null)[]
  const timestamps = props.userStats.scraped_at.map((d) => new Date(d).getTime())

  const data: [number, number][] = timestamps.map((t, i) => [t, isGib ? (raw[i] ?? 0) / 1024 / 1024 / 1024 : (raw[i] as number)])

  return {
    chart: { type: 'line', backgroundColor: 'transparent', height: 300 },
    title: { text: value },
    xAxis: { type: 'datetime' },
    yAxis: { title: { text: isGib ? 'GiB' : undefined } },
    legend: { enabled: false },
    credits: { enabled: false },
    series: [{ type: 'line', name: value, data, color: seriesColor }],
  }
}
</script>
<style scoped>
.charts {
  display: flex;
  justify-content: center;
  flex-wrap: wrap;
  gap: 10px;
}
</style>
