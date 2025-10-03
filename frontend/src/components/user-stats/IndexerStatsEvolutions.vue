<template>
  <div id="indexer-stats-detail">
    <div class="wrapper-center title">Evolution on the selected period</div>
    <div class="charts">
      <ContentContainer v-for="value in selectedValues" :key="value">
        <Chart type="line" :data="chartData(value)" :options="chartOptions(value)" />
      </ContentContainer>
    </div>
  </div>
</template>
<script lang="ts" setup>
import type { UserProfileVec, UserProfileScrapedVec } from '@/services/api/userStatsService'
import Chart from 'primevue/chart'
import ContentContainer from '../ContentContainer.vue'
import 'chartjs-adapter-date-fns'

const props = defineProps<{
  userStats: UserProfileVec
  selectedValues: (keyof UserProfileScrapedVec)[]
}>()
const documentStyle = getComputedStyle(document.documentElement)

const chartOptions = (value: keyof UserProfileScrapedVec) => {
  let unit = ''
  switch (value) {
    case 'uploaded':
    case 'downloaded':
    case 'seed_size':
    case 'uploaded_real':
    case 'downloaded_real':
      unit = 'GiB'
      break
  }
  return {
    scales: {
      x: {
        type: 'time',
        time: {
          unit: 'day',
        },
      },
      y: {
        title: {
          display: true,
          text: unit,
        },
      },
    },
  }
}

const chartData = (value: keyof UserProfileScrapedVec) => {
  let data: number[] = []
  switch (value) {
    case 'uploaded':
    case 'downloaded':
    case 'seed_size':
    case 'uploaded_real':
    case 'downloaded_real':
      data = props.userStats.profile[value].map((val) => (val ?? 0) / 1024 / 1024 / 1024)
      break
    default:
      data = props.userStats.profile[value] as number[]
  }
  return {
    labels: props.userStats.scraped_at.map((date) => new Date(date).toISOString()),
    datasets: [
      {
        // tension: 0.2,
        borderColor: documentStyle.getPropertyValue('--p-button-primary-border-color'),
        label: value,
        data: data,
      },
    ],
  }
}
</script>
<style scoped>
.charts {
  display: flex;
  justify-content: center;
  flex-wrap: wrap;
  > * {
    margin: 0 5px;
    margin-bottom: 10px;
  }
  .p-chart {
    width: 36em;
    height: auto;
  }
}
</style>
