<script setup lang="ts">
import { t } from '../../../composables/useI18n';

defineProps<{
  hasWorlds: boolean;
}>();
</script>

<template>
  <div>
    <div v-if="!hasWorlds" class="empty-message">{{ t('instance.no_worlds') }}</div>
    <div v-else class="worlds-list">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.worlds-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
  flex: 1;
}

.empty-message {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  color: var(--text-muted);
  font-style: italic;
  font-size: 1.1rem;
}
</style>
