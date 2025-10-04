<template>
  <div id="settings">
    <Tabs value="0">
      <TabList>
        <Tab value="0">Indexers</Tab>
      </TabList>
      <TabPanels>
        <TabPanel value="0">
          <IndexersTable />
        </TabPanel>
      </TabPanels>
    </Tabs>
  </div>
</template>

<script lang="ts" setup>
import Tabs from 'primevue/tabs'
import TabList from 'primevue/tablist'
import Tab from 'primevue/tab'
import TabPanels from 'primevue/tabpanels'
import TabPanel from 'primevue/tabpanel'
import IndexersTable from '@/components/settings/IndexersTable.vue'
</script>

<style scoped>
#settings {
  width: 100%;
  margin-bottom: 20px;
}
</style>
