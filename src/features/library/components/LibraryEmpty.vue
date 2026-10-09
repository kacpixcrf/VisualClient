<script setup lang="ts">
import { t } from '../../../composables/useI18n';

defineEmits(['create']);
</script>

<template>
  <div class="empty-state">
    <svg class="empty-icon" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
      <rect x="2" y="3" width="20" height="14" rx="2" ry="2"></rect>
      <line x1="8" y1="21" x2="16" y2="21"></line>
      <line x1="12" y1="17" x2="12" y2="21"></line>
    </svg>
    <h2 class="empty-title">{{ t('library.empty') }}</h2>
    <button class="create-btn" @click="$emit('create')">
      <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="plus-icon">
        <path d="M5 12h14" />
        <path d="M12 5v14" />
      </svg>
      <span>{{ t('library.create') }}</span>
    </button>
  </div>
</template>

<style scoped>
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  flex: 1;
  gap: 12px;
  opacity: 0.8;
}

.empty-icon {
  color: var(--text-muted);
  margin-bottom: 8px;
}

.empty-title {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--text-main);
}

.create-btn {
  background-color: var(--success);
  color: var(--color-black);
  border: none;
  padding: 12px 24px;
  border-radius: 999px;
  font-size: 1.1rem;
  font-weight: 600;
  cursor: pointer;
  transition: transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1), background-color 0.2s;
  margin-top: 16px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.create-btn:hover {
  background-color: color-mix(in srgb, var(--success) 85%, var(--color-black));
}

.create-btn:active {
  transform: scale(0.85);
}
</style>
