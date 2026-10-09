<script setup lang="ts">
import { t } from '../../../composables/useI18n';

defineProps<{
  title: string;
}>();

defineEmits<{
  addAccount: [];
}>();
</script>

<template>
  <header class="accounts-header">
    <h1 class="view-title">{{ title }}</h1>
    <button class="btn-add-account" @click="$emit('addAccount')">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"></path>
        <polyline points="10 17 15 12 10 7"></polyline>
        <line x1="15" y1="12" x2="3" y2="12"></line>
      </svg>
      {{ t('accounts.add') }}
    </button>
  </header>
</template>

<style scoped>
.accounts-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.view-title {
  margin: 0;
  font-size: 2rem;
  font-weight: 600;
  color: var(--text-main);
}

.btn-add-account {
  padding: 12px 24px;
  background-color: var(--success);
  color: var(--color-black);
  border: none;
  border-radius: 12px;
  font-size: 1.1rem;
  font-weight: 600;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: all 0.25s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.btn-add-account:hover {
  background-color: color-mix(in srgb, var(--success) 85%, var(--color-black));
}

.btn-add-account:active {
  transform: scale(0.85);
}
</style>
