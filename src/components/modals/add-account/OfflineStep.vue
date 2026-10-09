<script setup lang="ts">
import { ref, computed } from 'vue';
import { t } from '../../../composables/useI18n';

const emit = defineEmits<{
  create: [name: string];
}>();

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
  <div class="offline-pane">
    <div class="input-group">
      <label class="input-label">{{ t('accounts.nickname_label') }}</label>
      <input
        type="text"
        v-model="nickname"
        :placeholder="t('accounts.placeholder')"
        class="nickname-input"
        @keyup.enter="handleCreate"
        maxlength="16"
      />
      <span class="input-hint">{{ t('accounts.nickname_hint') }}</span>
    </div>
    <div class="modal-footer">
      <button class="btn-add" :disabled="!isValid" @click="handleCreate">
        {{ t('accounts.add_account') }}
      </button>
    </div>
  </div>
</template>

<style scoped>
.offline-pane {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding: 32px 24px;
  width: 100%;
  height: 100%;
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
  border: none;
  border-radius: 12px;
  padding: 0 16px;
  color: var(--text-main);
  font-size: 1.05rem;
  font-family: inherit;
  outline: none;
  transition: background-color 0.2s ease, color 0.2s ease, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.nickname-input:focus {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
  box-shadow: 0 0 0 4px var(--accent);
}

.input-hint {
  font-size: 0.85rem;
  color: var(--text-muted);
  font-weight: 500;
}

.modal-footer {
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
