<script setup lang="ts">
import { t } from '../../composables/useI18n';

const emit = defineEmits(['close', 'login']);

const handleAddAccountClick = () => {
  emit('login');
};
</script>

<template>
  <div class="modal-backdrop" @click.self="emit('close')">
    <div class="modal-container login-modal">
      <header class="login-modal-header">
        <span class="header-title">{{ t('instance.login_modal_title') }}</span>
        <div class="close-control" @click="emit('close')">
          <svg width="16" height="16" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M3 3L9 9M9 3L3 9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
      </header>
      <div class="login-modal-body">
        <p class="login-text">{{ t('instance.login_modal_text') }}</p>
        <button class="btn-add-account" @click="handleAddAccountClick">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
            <circle cx="12" cy="7" r="4"></circle>
          </svg>
          {{ t('instance.login_modal_btn') }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-backdrop {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: color-mix(in srgb, var(--color-black) 40%, transparent);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 999;
}

.login-modal {
  width: 480px;
  background-color: var(--bg-shell);
  border-radius: 16px;
  border: 1px solid var(--border-line);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  box-shadow: 0 10px 40px color-mix(in srgb, var(--color-black) 50%, transparent);
}

.login-modal-header {
  padding: 0 24px;
  min-height: 80px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid var(--border-line);
}

.header-title {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--text-main);
}

.close-control {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
  cursor: pointer;
  transition: background-color 0.2s, color 0.2s, transform 0.2s;
}

.close-control:hover {
  background-color: var(--danger);
  color: var(--color-white);
}

.close-control:active {
  transform: scale(0.85);
}

.login-modal-body {
  padding: 32px 24px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 24px;
}

.login-text {
  margin: 0;
  font-size: 1.05rem;
  color: var(--text-muted);
  text-align: center;
  font-weight: 600;
}

.btn-add-account {
  padding: 12px 24px;
  background-color: var(--accent);
  color: var(--color-black);
  border: none;
  border-radius: 12px;
  font-size: 1rem;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: all 0.25s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.btn-add-account:hover {
  background-color: color-mix(in srgb, var(--accent) 85%, var(--color-black));
}

.btn-add-account:active {
  transform: scale(0.85);
}
</style>
