<template>
  <div>
    <DataTable :value="indexers">
      <Column field="name" header="Name" />
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
            <Button icon="pi pi-pencil" size="small" @click="editIndexer(slotProps.data)" />
          </div>
        </template>
      </Column>
      <Dialog v-model:visible="indexerSettingsDialogVisible" @hide="indexerBeingEdited = null">
        <IndexerSettings
          v-if="indexerBeingEdited !== null"
          :indexer="indexerBeingEdited"
          @indexer-created="indexerCreated"
        />
      </Dialog>
    </DataTable>
    <div class="wrapper-center" style="margin-top: 15px">
      <Button
        label="Scrape user stats now"
        size="small"
        @click="scrapeUserStatsNow"
        :loading="scrapingUserStats"
      />
    </div>
  </div>
</template>
<script lang="ts" setup>
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { ToggleSwitch, Button, Dialog } from 'primevue'
import {
  getIndexers,
  type Indexer,
  type UpdatedIndexer,
  toggleIndexer,
} from '@/services/api/indexerService'
import { onMounted, ref } from 'vue'
import IndexerSettings from './IndexerSettings.vue'
import { showToast } from '@/main'
import { scrapeUserStats } from '@/services/api/userStatsService'

const scrapingUserStats = ref(false)

const indexers = ref<Indexer[]>([])
const indexerBeingEdited = ref<UpdatedIndexer | null>(null)
const indexerSettingsDialogVisible = ref(false)

const editIndexer = (indexer: UpdatedIndexer) => {
  indexerBeingEdited.value = indexer
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
const indexerCreated = () => {
  indexerBeingEdited.value = null
  indexerSettingsDialogVisible.value = false
}
const scrapeUserStatsNow = async () => {
  scrapingUserStats.value = true
  scrapeUserStats().finally(() => {
    scrapingUserStats.value = false
  })
}
onMounted(() => {
  getIndexers().then((i) => (indexers.value = i))
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
