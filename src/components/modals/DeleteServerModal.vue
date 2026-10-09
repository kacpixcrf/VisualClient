<script setup lang="ts">
import { t } from '../../composables/useI18n';
import IconTrash from '../icons/IconTrash.vue';

const props = defineProps<{
  isOpen: boolean;
}>();

const emit = defineEmits(['close', 'confirm']);
</script>

<template>
  <Transition name="modal">
    <div v-if="isOpen" class="modal-backdrop" @click="emit('close')">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h2 class="modal-title">{{ t('instance.remove_server_title') || 'Remove Server' }}</h2>
          <div class="close-control" @click="emit('close')">
            <svg width="16" height="16" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M3 3L9 9M9 3L3 9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </div>
        </div>
        <p class="modal-text" style="color: var(--text-muted); font-size: 0.95rem;">{{ t('instance.remove_server_desc') || 'Are you sure you want to remove this server? This action cannot be undone.' }}</p>
        <div class="modal-actions">
          <button class="btn btn-secondary" @click="emit('close')">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
            {{ t('instance.cancel') }}
          </button>
          <button class="btn btn-danger" @click="emit('confirm')">
            <IconTrash class="dropdown-icon" />
            {{ t('instance.remove_server') }}
          </button>
        </div>
      </div>
    </div>
  </Transition>
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

.modal-content {
  width: 480px;
  background-color: var(--bg-shell);
  border-radius: 16px;
  border: 1px solid var(--border-line);
  display: flex;
  flex-direction: column;
  box-shadow: 0 10px 40px color-mix(in srgb, var(--color-black) 50%, transparent);
  padding: 24px;
  gap: 20px;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 20px;
}

.modal-title {
  font-size: 1.3rem;
  font-weight: 600;
  color: var(--text-main);
  margin-bottom: 0;
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

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 16px;
}

.btn {
  height: 40px;
  padding: 0 20px;
  border-radius: 12px;
  font-weight: 600;
  font-size: 0.95rem;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  transition: background-color 0.2s ease, color 0.2s ease, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
  border: none;
}

.btn-secondary {
  background-color: transparent;
  color: var(--text-muted);
}

.btn-secondary:hover {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
  color: var(--text-main);
}

.btn-danger {
  background-color: rgba(237, 66, 69, 0);
  color: var(--danger);
  border: 1px solid transparent;
}

.btn-danger:hover {
  background-color: var(--danger);
  color: var(--color-black);
}

.btn:active {
  transform: scale(0.85);
}

.dropdown-icon {
  width: 16px;
  height: 16px;
  stroke-width: 2.5;
  flex-shrink: 0;
}

.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.2s ease;
}
.modal-enter-active .modal-content,
.modal-leave-active .modal-content {
  transition: transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
}
.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}
.modal-enter-from .modal-content,
.modal-leave-to .modal-content {
  transform: scale(0.85);
}
</style>
