<template>
  <div>
    <DataTable :value="indexers">
      <Column field="name" header="Name" />
      <Column field="last_scraped_at" header="Last scraped">
        <template #body="slotProps">
          {{ timeAgo(slotProps.data.last_scraped_at) }}
        </template>
      </Column>
      <Column>
        <template #body="slotProps">
          <div class="actions">
            <ToggleSwitch
              v-model="slotProps.data.enabled"
              @update:modelValue="
                (newVal: boolean) =>
                  toggleIndexerEnabledStatus(newVal, slotProps.data.id, slotProps.data.name)
              "
            />
            <Button icon="pi pi-pencil" size="small" @click="editIndexer(slotProps.data.id)" />
          </div>
        </template>
      </Column>
      <Dialog v-model:visible="indexerSettingsDialogVisible" @hide="indexerIdBeingEdited = null">
        <IndexerSettings
          v-if="indexerIdBeingEdited !== null"
          :indexerId="indexerIdBeingEdited"
          @indexer-edited="indexerEdited"
        />
      </Dialog>
    </DataTable>
    <div class="wrapper-center" style="margin-top: 15px">
      <Button
        label="Scrape user stats now"
        size="small"
        @click="scrapeUserStatsNow"
        :loading="scrapingUserStats"
        v-tooltip.top="'The request might take a while as some indexers have slow response times'"
      />
    </div>
  </div>
</template>
<script lang="ts" setup>
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { ToggleSwitch, Button, Dialog } from 'primevue'
import {
  toggleIndexer,
  getIndexersEnriched,
  type IndexerEnriched,
} from '@/services/api/indexerService'
import { onMounted, ref } from 'vue'
import IndexerSettings from './IndexerSettings.vue'
import { showToast } from '@/main'
import { scrapeUserStats } from '@/services/api/userStatsService'
import { timeAgo } from '@/services/helpers'

const scrapingUserStats = ref(false)

const indexers = ref<IndexerEnriched[]>([])
const indexerIdBeingEdited = ref<number | null>(null)
const indexerSettingsDialogVisible = ref(false)

const editIndexer = (indexerId: number) => {
  indexerIdBeingEdited.value = indexerId
  indexerSettingsDialogVisible.value = true
}
const toggleIndexerEnabledStatus = (newVal: boolean, id: number, name: string) => {
  toggleIndexer(id)
    .then(() => {
      showToast(
        '',
        `Successfully ${newVal ? 'enabled' : 'disabled'} the indexer ${name}`,
        'success',
        2000,
      )
    })
    .catch(() => {
      showToast('', `Error toggling the indexer ${name}`, 'error', 2000)
      indexers.value = indexers.value.map((obj) =>
        obj.id === id ? { ...obj, enabled: !obj.enabled } : obj,
      )
    })
}
const indexerEdited = () => {
  indexerIdBeingEdited.value = null
  indexerSettingsDialogVisible.value = false
}
const scrapeUserStatsNow = async () => {
  scrapingUserStats.value = true
  scrapeUserStats()
    .then(() => fetchIndexers())
    .finally(() => {
      scrapingUserStats.value = false
    })
}
const fetchIndexers = async () => {
  getIndexersEnriched(false).then((i) => (indexers.value = i))
}
onMounted(async () => {
  await fetchIndexers()
})
</script>
<style scoped>
.actions {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  > * {
    margin-left: 10px;
  }
}
</style>
