<script setup lang="ts">
import { computed } from 'vue';
import { t } from '../../composables/useI18n';
import { useCreateInstance } from '../../features/create-instance/composables/useCreateInstance';
import LoaderSelector from '../ui/create-instance/LoaderSelector.vue';
import VersionSelector from '../ui/create-instance/VersionSelector.vue';

const emit = defineEmits(['close', 'created']);

const {
  instances,
  name,
  selectedLoader,
  selectedVersion,
  selectedVersionUrl,
  handleCreate,
} = useCreateInstance(() => emit('close'));

const placeholderName = computed(() => {
  const loaderStr = selectedLoader.value || 'vanilla';
  const loaderName = loaderStr.charAt(0).toUpperCase() + loaderStr.slice(1);
  const baseName = `${loaderName} ${selectedVersion.value || ''}`.trim();
  let finalName = baseName;
  let i = 1;
  while (instances.value.some(inst => inst.id === finalName || inst.name === finalName)) {
    finalName = `${baseName} (${i})`;
    i++;
  }
  return finalName;
});

const nameExists = computed(() => {
  if (name.value.trim().length === 0) return false;
  return instances.value.some(inst => inst.id === name.value.trim());
});

const isNameValid = computed(() => name.value.length === 0 || name.value.length >= 3);

const canCreate = computed(() =>
  selectedLoader.value !== '' && selectedVersion.value !== '' && isNameValid.value && !nameExists.value
);
</script>


<template>
  <div class="modal-backdrop" @click.self="emit('close')">
    <div class="modal-container">
      <header class="modal-header">
        <span class="header-title">{{ t('create_instance.title') }}</span>
        <div class="close-control" @click="emit('close')">
          <svg width="16" height="16" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M3 3L9 9M9 3L3 9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
      </header>
      
      <div class="modal-body">

        <div class="form-group">
          <label class="form-label">{{ t('create_instance.name') }}</label>
          <input 
            type="text" 
            v-model="name" 
            :placeholder="placeholderName" 
            class="form-input" 
            maxlength="16"
          />
          <span class="info-msg" v-if="name.length > 0 && name.length < 3">{{ t('create_instance.min_length', { count: 3 }) || 'Minimum 3 characters' }}</span>
          <span class="info-msg" v-else-if="nameExists">{{ t('create_instance.name_exists') || 'This name already exists' }}</span>
        </div>
        <div class="form-group">
          <label class="form-label">{{ t('create_instance.loader') }}</label>
          <LoaderSelector v-model="selectedLoader" />
        </div>
        
        <div class="form-group">
          <label class="form-label">{{ t('create_instance.version') }}</label>
          <VersionSelector v-model="selectedVersion" :loader="selectedLoader" @update:url="u => selectedVersionUrl = u" />
        </div>
      </div>
      
      <footer class="modal-footer">
        <button class="modal-btn" @click="emit('close')">
          {{ t('create_instance.back') }}
        </button>
        <button class="modal-btn primary" :disabled="!canCreate" @click="handleCreate">
          {{ t('create_instance.create') }}
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
  width: 480px;
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
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-label {
  font-weight: 600;
  font-size: 0.95rem;
  color: var(--text-main);
}

.error-msg {
  color: var(--danger);
  font-size: 0.85rem;
  font-weight: 500;
  margin-left: 4px;
}

.info-msg {
  color: var(--accent);
  font-size: 0.85rem;
  font-weight: 500;
  margin-left: 4px;
}

.form-input {
  width: 100%;
  height: 44px;
  background-color: color-mix(in srgb, var(--color-white) 3%, transparent);
  border: none;
  border-radius: 12px;
  padding: 0 16px;
  color: var(--text-main);
  font-family: inherit;
  font-size: 1rem;
  transition: background-color 0.2s ease, color 0.2s ease, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
  outline: none;
}

.form-input::placeholder {
  color: var(--text-muted);
}

.form-input:focus {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
  box-shadow: 0 0 0 4px var(--accent);
}

.modal-footer {
  padding: 16px 20px;
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  background-color: transparent;
  border-radius: 0 0 16px 16px;
}

.modal-btn {
  height: 36px;
  padding: 0 16px;
  background-color: color-mix(in srgb, var(--color-white) 3%, transparent);
  border: 4px solid var(--border-line);
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  font-weight: 600;
  font-size: 1rem;
  color: var(--text-muted);
  transition: background-color 0.2s ease, color 0.2s ease, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
  font-family: inherit;
  outline: none;
}

.modal-btn:hover:not(:disabled) {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
}

.modal-btn:active:not(:disabled) {
  transform: scale(0.85);
}

.modal-btn.primary {
  border-color: var(--accent);
  color: var(--text-main);
  background-color: color-mix(in srgb, var(--accent) 15%, transparent);
}

.modal-btn.primary:hover:not(:disabled) {
  background-color: color-mix(in srgb, var(--accent) 25%, transparent);
}

.modal-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
