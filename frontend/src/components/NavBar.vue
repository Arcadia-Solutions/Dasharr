<template>
  <div id="nav-bar-container">
    <div id="nav-bar">
      <div class="left">
        <RouterLink class="app-name" to="/"> Dasharr </RouterLink>
      </div>
      <div class="right">
        <RouterLink to="/settings">
          <Button icon="pi pi-cog" size="small" />
        </RouterLink>
      </div>
    </div>
  </div>
</template>
<script lang="ts" setup>
import { Button } from 'primevue'
</script>
<style scoped>
#nav-bar-container {
  display: flex;
  justify-content: center;
  padding-top: 10px;
  padding-bottom: 20px;
}
#nav-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: var(--background-primary);
  width: 95%;
  padding: 10px;
  border-radius: 10px;
}
.app-name {
  font-weight: bolder;
  font-size: 1.5em;
  text-decoration: none;
  color: var(--p-button-primary-border-color);
}
</style>
