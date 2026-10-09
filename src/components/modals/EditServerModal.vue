<script setup lang="ts">
import { ref, watch } from 'vue';
import { t } from '../../composables/useI18n';

const props = defineProps<{
  isOpen: boolean;
  isAddingServer: boolean;
  editServerForm: { originalIp: string; name: string; ip: string; acceptTextures: number | null };
}>();

const emit = defineEmits(['close', 'save', 'update:editServerForm']);

const localForm = ref({ ...props.editServerForm });
const isRpSelectOpen = ref(false);

watch(() => props.editServerForm, (newVal) => {
  localForm.value = { ...newVal };
}, { deep: true });

watch(() => props.isOpen, (newVal) => {
  if (newVal) {
    isRpSelectOpen.value = false;
  }
});

const save = () => {
  emit('update:editServerForm', localForm.value);
  emit('save');
};

const setResourcePack = (val: number | null) => {
  localForm.value.acceptTextures = val;
  isRpSelectOpen.value = false;
};
</script>

<template>
  <Transition name="modal">
    <div v-if="isOpen" class="modal-backdrop" @click="emit('close')">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h2 class="modal-title">{{ isAddingServer ? t('instance.add_server') : t('instance.edit_server') }}</h2>
          <div class="close-control" @click="emit('close')">
            <svg width="16" height="16" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M3 3L9 9M9 3L3 9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </div>
        </div>
        
        <div class="form-group">
          <label>{{ t('instance.server_name') || 'Server Name' }}</label>
          <input type="text" v-model="localForm.name" class="input-field" placeholder="Minecraft Server" />
        </div>
        
        <div class="form-group">
          <label>{{ t('instance.server_address') || 'Address' }}</label>
          <input type="text" v-model="localForm.ip" class="input-field" placeholder="example.visualclient.com.pl" />
        </div>
        
        <div class="form-group">
          <label>{{ t('instance.server_resource_packs') || 'Resource Packs' }}</label>
          <div class="custom-dropdown" @click="isRpSelectOpen = !isRpSelectOpen">
            <div class="custom-dropdown-value">
              {{ localForm.acceptTextures === 1 ? t('instance.rp_enabled') || 'Enabled' : localForm.acceptTextures === 0 ? t('instance.rp_disabled') || 'Disabled' : t('instance.rp_prompt') || 'Prompt' }}
            </div>
            <svg class="custom-dropdown-arrow" :class="{ open: isRpSelectOpen }" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="6 9 12 15 18 9"></polyline>
            </svg>
            
            <Transition name="dropdown-fade">
              <div v-if="isRpSelectOpen" class="custom-dropdown-options" @click.stop>
                <div class="custom-dropdown-item" :class="{ selected: localForm.acceptTextures === null }" @click="setResourcePack(null)">
                  {{ t('instance.rp_prompt') || 'Prompt' }}
                </div>
                <div class="custom-dropdown-item" :class="{ selected: localForm.acceptTextures === 1 }" @click="setResourcePack(1)">
                  {{ t('instance.rp_enabled') || 'Enabled' }}
                </div>
                <div class="custom-dropdown-item" :class="{ selected: localForm.acceptTextures === 0 }" @click="setResourcePack(0)">
                  {{ t('instance.rp_disabled') || 'Disabled' }}
                </div>
              </div>
            </Transition>
          </div>
        </div>
        
        <div class="modal-actions">
          <button class="modal-btn" @click="emit('close')">
            {{ t('instance.cancel') }}
          </button>
          <button class="modal-btn primary" @click="save">
            {{ isAddingServer ? t('instance.add_server') : t('instance.save') }}
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

.form-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-group label {
  font-weight: 600;
  font-size: 0.95rem;
  color: var(--text-main);
}

.input-field {
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

.input-field::placeholder {
  color: var(--text-muted);
  font-weight: 500;
}

.input-field:focus {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
  box-shadow: 0 0 0 4px var(--accent);
}

.custom-dropdown {
  position: relative;
  width: 100%;
  height: 44px;
  background-color: color-mix(in srgb, var(--color-white) 3%, transparent);
  border: none;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
  cursor: pointer;
  color: var(--text-main);
  font-size: 1rem;
  transition: background-color 0.2s ease, color 0.2s ease, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.custom-dropdown:hover {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
}

.custom-dropdown:focus, .custom-dropdown:active, .custom-dropdown.active {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
}

.custom-dropdown-value {
  font-weight: 500;
  pointer-events: none;
}

.custom-dropdown-arrow {
  color: var(--text-muted);
  transition: transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
  pointer-events: none;
}

.custom-dropdown-arrow.open {
  transform: rotate(180deg);
}

.custom-dropdown-options {
  position: absolute;
  top: calc(100% + 8px);
  left: 0;
  width: 100%;
  background-color: color-mix(in srgb, var(--bg-shell) 95%, var(--color-white));
  border: 1px solid var(--border-line);
  border-radius: 12px;
  box-shadow: 0 8px 24px color-mix(in srgb, var(--color-black) 40%, transparent);
  z-index: 100;
  display: flex;
  flex-direction: column;
}

.custom-dropdown-item {
  padding: 10px 16px;
  color: var(--text-muted);
  font-size: 0.95rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
}

.custom-dropdown-item:first-child {
  border-top-left-radius: 11px;
  border-top-right-radius: 11px;
}

.custom-dropdown-item:last-child {
  border-bottom-left-radius: 11px;
  border-bottom-right-radius: 11px;
}

.custom-dropdown-item:hover {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
  color: var(--text-main);
}

.custom-dropdown-item.selected {
  border-color: var(--accent);
  color: var(--color-black);
  background-color: var(--accent);
}

.custom-dropdown-item.selected:hover {
  background-color: color-mix(in srgb, var(--accent) 85%, var(--color-black));
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 16px;
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

.dropdown-fade-enter-active,
.dropdown-fade-leave-active {
  transition: opacity 0.2s, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
}
.dropdown-fade-enter-from,
.dropdown-fade-leave-to {
  opacity: 0;
  transform: translateY(-8px);
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
