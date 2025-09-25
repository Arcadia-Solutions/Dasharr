<template>
  <div class="indexer-settings">
    <Form @submit="submit" class="form">
      <div class="auth-item" v-for="(authItem, name) in authData" :key="name">
        <span class="name">{{ name }}:</span>
        <span class="explanation" v-if="authItem.explanation">{{ authItem.explanation }}</span>
        <InputText v-model="authItem.value" :placeholder="name" />
      </div>
      <div class="wrapper-center">
        <Button type="submit" label="Submit" size="small" style="margin-top: 20px" />
      </div>
    </Form>
  </div>
</template>
<script lang="ts" setup>
import {
  type UpdatedIndexer,
  type AuthItem,
  editIndexer,
  type Indexer,
} from '@/services/api/indexerService'
import { InputText, Button } from 'primevue'
import { Form } from '@primevue/forms'
import { onMounted, ref } from 'vue'

const props = defineProps<{
  indexer: UpdatedIndexer
}>()

const emit = defineEmits<{
  indexerCreated: [Indexer]
}>()

const authData = ref<Record<string, AuthItem>>()

const submit = () => {
  if (authData.value) {
    editIndexer({ id: props.indexer.id, auth_data: authData.value }).then((indexer) =>
      emit('indexerCreated', indexer),
    )
  }
}

onMounted(() => {
  authData.value = props.indexer.auth_data
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
