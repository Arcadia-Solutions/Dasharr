<template>
  {{ userStats }}
</template>
<script lang="ts" setup>
import type { UserProfileVec } from '@/services/api/userStatsService'

defineProps<{
  userStats: UserProfileVec
}>()
</script>
<style scoped></style>
