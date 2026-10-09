<script setup lang="ts">
import { t } from '../../../composables/useI18n';

defineProps<{
  loginError: string;
}>();

defineEmits<{
  retry: [];
}>();
</script>

<template>
  <div class="microsoft-pane">
    <template v-if="!loginError">
      <div class="spinner"></div>
      <span>{{ t('accounts.waiting_for_login') }}</span>
    </template>
    <div v-else class="error-box">
      <span class="error-title">{{ t('accounts.login_error_title') }}</span>
      <span class="error-msg">{{ loginError }}</span>
      <button class="btn-retry" @click="$emit('retry')">{{ t('accounts.go_back') }}</button>
    </div>
  </div>
</template>

<style scoped>
.microsoft-pane {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
  gap: 16px;
  color: var(--text-muted);
  font-size: 1.1rem;
  font-weight: 500;
}

.spinner {
  width: 32px;
  height: 32px;
  border: 3px solid color-mix(in srgb, var(--color-white) 6%, transparent);
  border-top-color: var(--accent);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.error-box {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  text-align: center;
}

.error-title {
  color: var(--danger);
  font-weight: 600;
  font-size: 1.2rem;
}

.error-msg {
  color: var(--text-muted);
  font-size: 0.95rem;
  max-width: 80%;
}

.btn-retry {
  padding: 12px 24px;
  background-color: var(--accent);
  color: var(--color-black);
  border: none;
  border-radius: 12px;
  font-size: 1rem;
  -webkit-font-smoothing: antialiased;
  cursor: pointer;
  transition: background-color 0.2s ease, color 0.2s ease, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.btn-retry:hover {
  background-color: color-mix(in srgb, var(--accent) 85%, var(--color-black));
}

.btn-retry:active {
  transform: scale(0.85);
}
</style>
