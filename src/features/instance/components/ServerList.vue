<script setup lang="ts">
import { t } from '../../../composables/useI18n';

defineProps<{
  hasServers: boolean;
}>();
</script>

<template>
  <div>
    <div v-if="!hasServers" class="empty-message">{{ t('instance.no_servers') }}</div>
    <div v-else class="servers-list">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.servers-list {
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
