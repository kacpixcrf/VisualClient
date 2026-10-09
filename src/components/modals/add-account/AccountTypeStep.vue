<script setup lang="ts">
import { t } from '../../../composables/useI18n';
import IconMicrosoft from '../../icons/IconMicrosoft.vue';

defineEmits<{
  selectOffline: [];
  selectMicrosoft: [];
}>();
</script>

<template>
  <div class="choose-pane">
    <div class="account-type-card" @click="$emit('selectOffline')">
      <div class="card-icon offline-icon">
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
          <circle cx="12" cy="7" r="4"></circle>
        </svg>
      </div>
      <div class="card-info">
        <span class="card-title">{{ t('accounts.offline') }}</span>
        <span class="card-desc">{{ t('accounts.offline_desc') }}</span>
      </div>
    </div>

    <div class="account-type-card" @click="$emit('selectMicrosoft')">
      <div class="card-icon">
        <IconMicrosoft />
      </div>
      <div class="card-info">
        <span class="card-title">{{ t('accounts.microsoft') }}</span>
        <span class="card-desc">{{ t('accounts.microsoft_desc') }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.choose-pane {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 32px 24px;
  width: 100%;
  height: 100%;
}

.account-type-card {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 20px;
  background-color: color-mix(in srgb, var(--color-white) 3%, transparent);
  border: 1px solid var(--border-line);
  border-radius: 16px;
  cursor: pointer;
  transition: background-color 0.2s ease, color 0.2s ease, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.account-type-card:hover {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
  border-color: var(--text-muted);
}

.account-type-card:active {
  transform: scale(0.85);
}

.card-icon {
  width: 48px;
  height: 48px;
  background-color: var(--bg-shell);
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.offline-icon {
  color: var(--text-main);
}

.card-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.card-title {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--text-main);
}

.card-desc {
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--text-muted);
}
</style>
