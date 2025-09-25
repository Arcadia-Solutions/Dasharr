<template>
  <DataTable :value="indexers">
    <Column field="name" header="Name" />
    <Column>
      <template #body="slotProps">
        <Button icon="pi pi-pencil" size="small" @click="editIndexer(slotProps.data)" />
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
</template>
<script lang="ts" setup>
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { Button, Dialog } from 'primevue'
import { getIndexers, type Indexer, type UpdatedIndexer } from '@/services/api/indexerService'
import { onMounted, ref } from 'vue'
import IndexerSettings from './IndexerSettings.vue'

const indexers = ref<Indexer[]>([])
const indexerBeingEdited = ref<UpdatedIndexer | null>(null)
const indexerSettingsDialogVisible = ref(false)

const editIndexer = (indexer: UpdatedIndexer) => {
  indexerBeingEdited.value = indexer
  indexerSettingsDialogVisible.value = true
}
const indexerCreated = () => {
  indexerBeingEdited.value = null
  indexerSettingsDialogVisible.value = false
}
onMounted(() => {
  getIndexers().then((i) => (indexers.value = i))
})
</script>
