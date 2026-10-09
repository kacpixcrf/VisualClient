<script setup lang="ts">
import { ref, computed } from 'vue';
import { t } from '../../../composables/useI18n';
import { useInstances, Instance } from '../../../composables/useInstances';
import IconTrash from '../../icons/IconTrash.vue';

const props = defineProps<{ instance: Instance }>();
const emit = defineEmits(['requestDelete']);

const { instances, renameInstance } = useInstances();
const instanceNameInput = ref(props.instance.name);

const nameExists = computed(() => {
  const trimmed = instanceNameInput.value.trim();
  if (trimmed === props.instance.name) return false;
  if (trimmed.length === 0) return false;
  return instances.value.some(inst => inst.name === trimmed || inst.id === trimmed);
});

const isNameValid = computed(() => instanceNameInput.value.length >= 3);

const handleNameUpdate = () => {
  if (isNameValid.value && !nameExists.value) {
    renameInstance(props.instance.id, instanceNameInput.value.trim());
  }
};
</script>

<template>
  <div class="settings-section">
    <div class="setting-item">
      <label class="setting-label"><strong>{{ t('instance_settings.name') }}</strong></label>
      <input 
        type="text" 
        v-model="instanceNameInput" 
        class="setting-input" 
        @input="handleNameUpdate"
        maxlength="16"
      />
      <span class="info-msg" v-if="instanceNameInput.length > 0 && instanceNameInput.length < 3">{{ t('create_instance.min_length', { count: 3 }) || 'Minimum 3 characters' }}</span>
      <span class="info-msg" v-else-if="nameExists">{{ t('create_instance.name_exists') || 'This name already exists' }}</span>
    </div>

    <div class="setting-item">
      <div class="setting-info">
        <span class="setting-name">{{ t('instance_settings.delete_title') }}</span>
        <span class="setting-desc">{{ t('instance_settings.delete_desc') }}</span>
      </div>
      <div>
        <button class="btn-delete" @click="emit('requestDelete')">
          <IconTrash class="btn-icon" />
          {{ t('instance_settings.delete_btn') }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-section {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.setting-item {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.setting-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.setting-name {
  font-weight: 600;
  font-size: 1rem;
  color: var(--text-main);
}

.setting-desc {
  font-size: 0.85rem;
  color: var(--text-muted);
}

.setting-label {
  color: var(--text-main);
}

.info-msg {
  color: var(--accent);
  font-size: 0.85rem;
  font-weight: 500;
  margin-left: 4px;
}

.setting-input {
  width: 100%;
  padding: 12px 16px;
  background-color: color-mix(in srgb, var(--color-white) 3%, transparent);
  border: none;
  color: var(--text-main);
  border-radius: 8px;
  font-family: inherit;
  font-size: 1rem;
  transition: border-color 0.2s;
}

.setting-input:focus {
  outline: none;
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
}

.btn-delete {
  background-color: rgba(237, 66, 69, 0);
  color: var(--danger);
  border: none;
  border-radius: 12px;
  padding: 10px 24px;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: background-color 0.2s ease, color 0.2s ease, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.btn-icon {
  width: 18px;
  height: 18px;
}

.btn-delete:hover {
  background-color: var(--danger);
  color: var(--color-black);
}

.btn-delete:active {
  transform: scale(0.85);
}
</style>
