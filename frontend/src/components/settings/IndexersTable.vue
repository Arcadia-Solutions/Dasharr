<template>
  <DataTable :value="getConfigurableIndexers()">
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
import { getConfigurableIndexers } from '@/services/helpers'
import { Button, Dialog } from 'primevue'
import type { NewIndexer } from '@/services/api/indexerService'
import { ref } from 'vue'
import IndexerSettings from './IndexerSettings.vue'

const indexerBeingEdited = ref<NewIndexer | null>(null)
const indexerSettingsDialogVisible = ref(false)

const editIndexer = (indexer: NewIndexer) => {
  indexerBeingEdited.value = indexer
  indexerSettingsDialogVisible.value = true
}
const indexerCreated = () => {
  indexerBeingEdited.value = null
  indexerSettingsDialogVisible.value = false
}
</script>
