<template>
  <div class="indexer-settings">
    <Form @submit="submit" class="form">
      <div class="auth-item" v-for="(authItem, name) in authData" :key="name">
        <span class="name">{{ name }}:</span>
        <span class="explanation" v-if="authItem.explanation">{{ authItem.explanation }}</span>
        <InputText v-model="authItem.value" :placeholder="name" />
      </div>
      <div class="wrapper-center">
        <Button type="submit" label="Submit" size="small" style="margin-top: 20px" :loading />
      </div>
    </Form>
  </div>
</template>
<script lang="ts" setup>
import { type AuthItem, editIndexer, getIndexerAuthData, type Indexer } from '@/services/api/indexerService'
import { InputText, Button } from 'primevue'
import { Form } from '@primevue/forms'
import { onMounted, ref } from 'vue'
import { showToast } from '@/main'

const props = defineProps<{
  indexerId: number
}>()

const emit = defineEmits<{
  indexerEdited: [Indexer]
}>()

const loading = ref(false)

const authData = ref<Record<string, AuthItem>>()

const submit = () => {
  if (authData.value) {
    loading.value = true
    editIndexer({ id: props.indexerId, auth_data: authData.value })
      .then((indexer) => {
        showToast('', 'Indexer config has been validated and saved', 'success', 2000)
        emit('indexerEdited', indexer)
      })
      .finally(() => (loading.value = false))
  }
}

onMounted(() => {
  getIndexerAuthData(props.indexerId).then((data) => (authData.value = data))
})
</script>
<style scoped>
.form {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
}
.auth-item {
  display: flex;
  flex-direction: column;
  margin-bottom: 15px;
  .name {
    font-weight: bold;
  }
  .explanation {
    font-weight: 300;
  }
}
</style>
