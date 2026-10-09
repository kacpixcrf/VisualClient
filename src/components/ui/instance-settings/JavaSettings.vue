<script setup lang="ts">
import { ref } from 'vue';
import { t } from '../../../composables/useI18n';
import { Instance } from '../../../composables/useInstances';

const props = defineProps<{ instance: Instance }>();

const isCopied = ref(false);

const copyJavaPath = async () => {
  try {
    await navigator.clipboard.writeText(props.instance.java_path);
    isCopied.value = true;
    setTimeout(() => {
      isCopied.value = false;
    }, 2000);
  } catch (err) {
    console.error('Failed to copy', err);
  }
};
</script>

<template>
  <div class="settings-section">
    <div class="setting-item">
      <div class="setting-info">
        <span class="setting-name">{{ t('instance_settings.java_path') }}</span>
        <span class="setting-desc">{{ t('instance_settings.java_path_desc') }}</span>
      </div>
      <div class="path-input-group">
        <input type="text" readonly :value="instance.java_path" class="setting-input path-input" />
        <button class="btn-copy" @click="copyJavaPath">
          {{ isCopied ? t('instance_settings.copied') : t('instance_settings.copy') }}
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

.path-input-group {
  display: flex;
  gap: 8px;
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

.path-input {
  flex: 1;
  background-color: color-mix(in srgb, var(--color-white) 3%, transparent);
  color: var(--text-muted);
  font-family: monospace;
  pointer-events: none;
  user-select: none;
}

.btn-copy {
  padding: 0 20px;
  background-color: color-mix(in srgb, var(--color-white) 3%, transparent);
  color: var(--text-main);
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: background-color 0.2s ease, color 0.2s ease, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.btn-copy:hover {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
  color: var(--text-main);
}

.btn-copy:active {
  transform: scale(0.85);
}
</style>
