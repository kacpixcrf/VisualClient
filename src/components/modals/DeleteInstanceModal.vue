<script setup lang="ts">
import { t } from '../../composables/useI18n';
import IconTrash from '../icons/IconTrash.vue';

defineProps<{
  name: string
}>();

const emit = defineEmits(['close', 'confirm']);
</script>

<template>
  <div class="modal-backdrop" @click.self="emit('close')">
    <div class="modal-container">
      <header class="modal-header">
        <span class="header-title">{{ t('library.delete_confirm_title') }}</span>
        <div class="close-control" @click="emit('close')">
          <svg width="16" height="16" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M3 3L9 9M9 3L3 9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
      </header>
      
      <div class="modal-body">
        <p class="confirm-text">{{ t('library.delete_confirm_text').replace('{name}', name) }}</p>
      </div>
      
      <footer class="modal-footer">
        <button class="btn btn-cancel" @click="emit('close')">{{ t('library.cancel_btn') }}</button>
        <button class="btn btn-delete" @click="emit('confirm')">
          <IconTrash class="btn-icon" />
          {{ t('library.delete_btn') }}
        </button>
      </footer>
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

.modal-container {
  width: 440px;
  background-color: var(--bg-shell);
  border-radius: 16px;
  border: 1px solid var(--border-line);
  display: flex;
  flex-direction: column;
  box-shadow: 0 10px 40px color-mix(in srgb, var(--color-black) 50%, transparent);
}

.modal-header {
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 20px;
  border-bottom: 1px solid var(--border-line);
  background-color: var(--bg-shell);
  border-radius: 16px 16px 0 0;
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

.modal-body {
  padding: 24px 20px;
}

.confirm-text {
  font-size: 1rem;
  color: var(--text-muted);
  line-height: 1.5;
  margin: 0;
}

.modal-footer {
  padding: 16px 20px;
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  background-color: transparent;
}

.btn {
  height: 44px;
  padding: 0 24px;
  border-radius: 12px;
  font-size: 1rem;
  -webkit-font-smoothing: antialiased;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: background-color 0.2s ease, color 0.2s ease, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
  border: none;
}

.btn:active {
  transform: scale(0.85);
}

.btn-cancel {
  background-color: color-mix(in srgb, var(--color-white) 3%, transparent);
  color: var(--text-main);
}

.btn-cancel:hover {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
}

.btn-icon {
  width: 18px;
  height: 18px;
}

.btn-delete {
  background-color: rgba(237, 66, 69, 0);
  color: var(--danger);
  border: none;
  border-radius: 12px;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 8px;
}

.btn-delete:hover {
  background-color: var(--danger);
  color: var(--color-black);
}
</style>
