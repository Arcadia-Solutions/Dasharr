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
import HC, { type Options } from 'highcharts'
import type { IndexerStats, UserProfileScrapedVec } from '@/services/api-schema'
import ContentContainer from '../ContentContainer.vue'

const GIB_KEYS: (keyof UserProfileScrapedVec)[] = ['uploaded', 'downloaded', 'seed_size', 'uploaded_real', 'downloaded_real']

const props = defineProps<{
  indexerStats: IndexerStats[]
  selectedValues: (keyof UserProfileScrapedVec)[]
  indexerNames: Map<number, string>
}>()

const FIVE_MIN = 5 * 60 * 1000
const roundTs = (t: number) => Math.round(t / FIVE_MIN) * FIVE_MIN

function chartOptions(value: keyof UserProfileScrapedVec): Options {
  const isGib = GIB_KEYS.includes(value)

  const series: Options['series'] = props.indexerStats
    .filter((indexer) => indexer.profile[value]?.length)
    .map((indexer) => {
      const raw = indexer.profile[value] as (number | null)[]
      const data: [number, number][] = indexer.scraped_at.map((d, i) => [
        roundTs(new Date(d).getTime()),
        isGib ? (raw[i] ?? 0) / 1024 / 1024 / 1024 : (raw[i] as number),
      ])
      return {
        type: 'line' as const,
        name: props.indexerNames.get(indexer.indexer_id) ?? `Indexer ${indexer.indexer_id}`,
        data,
      }
    })

  return {
    chart: { type: 'line', backgroundColor: 'transparent', height: 300 },
    title: { text: value },
    xAxis: { type: 'datetime' },
    yAxis: { title: { text: isGib ? 'GiB' : undefined } },
    tooltip: {
      shared: true,
      formatter: function () {
        if (!this.points) return ''
        const sorted = [...this.points].sort((a, b) => (b.y ?? 0) - (a.y ?? 0))
        const header = `<b>${HC.dateFormat('%A, %b %e, %H:%M', this.x as number)}</b>`
        const rows = sorted.map(
          (p) => `<br/><span style="color:${p.color}">\u25CF</span> ${p.series.name}: <b>${isGib ? (p.y ?? 0).toFixed(2) + ' GiB' : p.y}</b>`,
        )
        return header + rows.join('')
      },
    },
    legend: { enabled: series.length > 1 },
    credits: { enabled: false },
    series,
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
