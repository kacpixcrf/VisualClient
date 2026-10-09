<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { getVersion } from '@tauri-apps/api/app';
import { t } from '../../composables/useI18n';

const emit = defineEmits(['close']);
const activeTab = ref('appearance');
const appVersion = ref('');

onMounted(async () => {
  try {
    appVersion.value = await getVersion();
  } catch (e) {
    appVersion.value = '0.1.0';
  }
});

defineExpose({ activeTab });
</script>

<template>
  <div class="modal-backdrop" @click.self="emit('close')">
    <div class="modal-container">
      <header class="modal-header">
        <div class="header-left">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="3"></circle>
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
          </svg>
          <span class="header-title">{{ t('settings.title') }}</span>
        </div>
        <div class="close-control" @click="emit('close')">
          <svg width="16" height="16" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M3 3L9 9M9 3L3 9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
      </header>
      
      <div class="modal-body">
        <aside class="settings-sidebar">
          <div class="sidebar-tabs">
            <div class="settings-tab" :class="{ active: activeTab === 'appearance' }" @click="activeTab = 'appearance'">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="m9.06 11.9 8.07-8.06a2.85 2.85 0 1 1 4.03 4.03l-8.06 8.08"></path>
                <path d="M7.07 14.94c-1.66 0-3 1.35-3 3.02 0 1.33-2.5 1.52-2 2.02 1.08 1.08 2.49 2.02 4 2.02 2.2 0 4-1.8 4-4.04a3.01 3.01 0 0 0-3-3.02z"></path>
              </svg>
              <span>{{ t('settings.appearance') }}</span>
            </div>
            
            <div class="settings-tab" :class="{ active: activeTab === 'language' }" @click="activeTab = 'language'">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="m5 8 6 6"></path>
                <path d="m4 14 6-6 2-3"></path>
                <path d="M2 5h12"></path>
                <path d="M7 2h1"></path>
                <path d="m22 22-5-10-5 10"></path>
                <path d="M14 18h6"></path>
              </svg>
              <span>{{ t('settings.language') }}</span>
            </div>
          </div>
          
          <div class="sidebar-footer">
            <div class="footer-info">
              <img src="../../assets/icon.svg" alt="Icon" class="footer-icon" />
              <span class="footer-text">Visual Client v{{ appVersion }}</span>
            </div>
          </div>
        </aside>
        
        <main class="settings-content">
          <slot :activeTab="activeTab" />
        </main>
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

.modal-container {
  width: 75vw;
  height: 75vh;
  background-color: var(--bg-shell);
  border-radius: 20px;
  border: 1px solid var(--border-line);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  box-shadow: 0 10px 40px color-mix(in srgb, var(--color-black) 50%, transparent);
}

.modal-header {
  padding: 24px 32px;
  min-height: 80px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid var(--border-line);
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
  color: var(--text-main);
}

.header-title {
  font-size: 1.25rem;
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

.modal-body {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.settings-sidebar {
  width: 220px;
  background-color: var(--bg-shell);
  padding: 16px 12px;
  display: flex;
  flex-direction: column;
  gap: 8px;
  position: relative;
}

.settings-sidebar::after {
  content: "";
  position: absolute;
  top: 24px;
  bottom: 24px;
  right: 0;
  width: 1px;
  background-color: var(--border-line);
}

.sidebar-tabs {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.settings-tab {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 14px;
  border-radius: 14px;
  color: var(--text-muted);
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s ease, color 0.2s ease, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.settings-tab:active {
  transform: scale(0.85);
}

.settings-tab:hover {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
  color: var(--text-main);
}

.settings-tab.active {
  background-color: color-mix(in srgb, var(--accent) 15%, transparent);
  color: var(--accent);
}



.sidebar-footer {
  margin-top: auto;
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding: 10px 14px;
  color: var(--text-muted);
}

.footer-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.footer-icon {
  width: 24px;
  height: 24px;
  object-fit: contain;
  cursor: pointer;
  transition: transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.footer-icon:active {
  transform: scale(0.85);
}

.footer-text {
  font-size: 0.85rem;
  font-weight: 600;
  white-space: nowrap;
}

.settings-content {
  flex: 1;
  padding: 32px;
  overflow-y: auto;
}


</style>
