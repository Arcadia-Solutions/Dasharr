<template>
  <main>
    <div class="form">
      <InputText placeholder="API key" v-model="apiKey" />
      <Button @click="submit" label="Submit" size="small" style="margin-top: 10px" />
    </div>
  </main>
</template>
<script setup lang="ts">
import { Button, InputText } from 'primevue'
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

const apiKey = ref('')
const submit = () => {
  localStorage.setItem('api_key', apiKey.value)
  router.push('/')
}
</script>

<style scoped>
.form {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
}
</style>
