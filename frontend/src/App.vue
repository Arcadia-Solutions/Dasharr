<template>
  <NavBar v-if="route.path !== '/login'" />
  <Toast position="top-right" group="tr" />
  <div id="main-content-wrapper">
    <RouterView />
  </div>
</template>
<script setup lang="ts">
import { useRoute } from 'vue-router'
import { Toast } from 'primevue'
import NavBar from './components/NavBar.vue'

const route = useRoute()
</script>

<style scoped>
#main-content-wrapper {
  padding: 0 5%;
  display: flex;
  justify-content: center;
}
</style>
