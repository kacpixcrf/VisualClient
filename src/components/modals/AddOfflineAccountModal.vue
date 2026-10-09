<script setup lang="ts">
import { ref, computed } from 'vue';

const emit = defineEmits(['close', 'create']);

const nickname = ref('');

const isValid = computed(() => {
  const n = nickname.value;
  return n.length >= 3 && n.length <= 16 && /^[a-zA-Z0-9_]+$/.test(n);
});

const handleCreate = () => {
  if (isValid.value) {
    emit('create', nickname.value);
  }
};
</script>

<template>
  <div class="modal-backdrop" @click.self="emit('close')">
    <div class="modal-container login-modal">
      <header class="login-modal-header">
        <span class="header-title">Add Offline Account</span>
        <div class="close-control" @click="emit('close')">
          <svg width="16" height="16" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M3 3L9 9M9 3L3 9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
      </header>
      
      <div class="login-modal-body">
        <div class="input-group">
          <label class="input-label">Nickname</label>
          <input 
            type="text" 
            v-model="nickname" 
            placeholder="Username..." 
            class="nickname-input"
            @keyup.enter="handleCreate"
            maxlength="16"
          />
          <span class="input-hint">3-16 characters. Letters, numbers, and underscores only.</span>
        </div>
      </div>

      <footer class="modal-footer">
        <button class="btn btn-add" :disabled="!isValid" @click="handleCreate">
          Add account
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
}

.input-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.input-label {
  font-weight: 600;
  color: var(--text-main);
  font-size: 0.95rem;
}

.nickname-input {
  width: 100%;
  height: 48px;
  background-color: color-mix(in srgb, var(--color-white) 3%, transparent);
  border: 1px solid var(--border-line);
  border-radius: 12px;
  padding: 0 16px;
  color: var(--text-main);
  font-size: 1.05rem;
  font-family: inherit;
  outline: none;
  transition: all 0.2s;
}

.nickname-input:focus {
  border-color: var(--accent);
}

.input-hint {
  font-size: 0.85rem;
  color: var(--text-muted);
  font-weight: 500;
}

.modal-footer {
  padding: 16px 24px;
  display: flex;
  justify-content: flex-end;
}

.btn-add {
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

.btn-add:not(:disabled):hover {
  background-color: color-mix(in srgb, var(--accent) 85%, var(--color-black));
}

.btn-add:not(:disabled):active {
  transform: scale(0.85);
}

.btn-add:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
