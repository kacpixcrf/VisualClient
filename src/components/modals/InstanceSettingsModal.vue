<script setup lang="ts">
import { ref } from 'vue';
import { t } from '../../composables/useI18n';
import { useInstances, Instance } from '../../composables/useInstances';
import IconInstance from '../icons/IconInstance.vue';
import IconJava from '../icons/IconJava.vue';
import IconChevronRight from '../icons/IconChevronRight.vue';
import IconInfo from '../icons/IconInfo.vue';

const { deleteInstance } = useInstances();

const props = defineProps<{
  instance: Instance
}>();

const emit = defineEmits(['close', 'deleted']);

const showDeleteConfirm = ref(false);

const handleDeleteConfirm = async () => {
  await deleteInstance(props.instance.id);
  showDeleteConfirm.value = false;
  emit('deleted');
};

const activeTab = ref('general');

defineExpose({ activeTab, showDeleteConfirm, handleDeleteConfirm });
</script>

<template>
  <div class="modal-backdrop" @click.self="emit('close')">
    <div class="modal-container">
      <header class="modal-header">
        <div class="breadcrumb">
          <div class="instance-avatar">
            <IconInstance />
          </div>
          <span class="breadcrumb-name">{{ instance.name }}</span>
          <span class="breadcrumb-separator">
            <IconChevronRight />
          </span>
          <span class="breadcrumb-title">{{ t('instance_settings.title') }}</span>
        </div>
        <div class="close-control" @click="emit('close')">
          <svg width="16" height="16" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M3 3L9 9M9 3L3 9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
      </header>
      
      <div class="modal-content-wrapper">
        <aside class="settings-sidebar">
          <nav class="sidebar-nav">
            <div 
              class="sidebar-item" 
              :class="{ active: activeTab === 'general' }"
              @click="activeTab = 'general'"
            >
              <IconInfo class="sidebar-icon" />
              {{ t('instance_settings.general') }}
            </div>
            <div 
              class="sidebar-item" 
              :class="{ active: activeTab === 'java' }"
              @click="activeTab = 'java'"
            >
              <IconJava class="sidebar-icon" />
              {{ t('instance_settings.java') }}
            </div>
          </nav>
        </aside>
        
        <main class="settings-content">
          <slot :activeTab="activeTab" :instance="instance" />
        </main>
      </div>
    </div>

    <Transition name="modal">
      <slot name="deleteModal" :showDeleteConfirm="showDeleteConfirm" :onConfirm="handleDeleteConfirm" :onClose="() => showDeleteConfirm = false" />
    </Transition>
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
  width: 800px;
  height: 500px;
  background-color: var(--bg-shell);
  border-radius: 16px;
  border: 1px solid var(--border-line);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  box-shadow: 0 10px 40px color-mix(in srgb, var(--color-black) 50%, transparent);
}

.modal-header {
  padding: 24px 32px 24px 16px;
  min-height: 80px;
  border-bottom: 1px solid var(--border-line);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.breadcrumb {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 1.1rem;
}

.sidebar-avatar {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 24px;
}

.instance-avatar {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.instance-avatar svg {
  width: 16px;
  height: 16px;
}

.breadcrumb-name {
  color: var(--text-muted);
}

.breadcrumb-separator {
  display: flex;
  align-items: center;
  color: var(--text-muted);
}

.breadcrumb-separator svg {
  width: 18px;
  height: 18px;
}

.breadcrumb-title {
  color: white;
  font-weight: 600;
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

.modal-content-wrapper {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.settings-sidebar {
  width: 200px;
  background-color: var(--bg-shell);
  border-right: 1px solid var(--border-line);
  padding: 20px 12px;
}

.sidebar-nav {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.sidebar-item {
  padding: 8px 16px;
  border-radius: 14px;
  cursor: pointer;
  color: var(--text-muted);
  font-weight: 600;
  transition: background-color 0.2s ease, color 0.2s ease, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
  display: flex;
  align-items: center;
  gap: 10px;
}

.sidebar-icon {
  width: 20px;
  height: 20px;
}

.sidebar-item:hover {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
  color: var(--text-main);
}

.sidebar-item:active {
  transform: scale(0.85);
}

.sidebar-item.active {
  background-color: color-mix(in srgb, var(--accent) 15%, transparent);
  color: var(--accent);
}

.settings-content {
  flex: 1;
  padding: 32px;
  overflow-y: auto;
}
</style>
