<script setup lang="ts">
import { t } from '../../../composables/useI18n';
import IconPlus from '../../../components/icons/IconPlus.vue';
import { TAB_INDEX_SERVERS } from '../../../constants';

defineProps<{
  activeTab: number;
}>();

const emit = defineEmits<{
  'update:activeTab': [index: number];
  addServer: [];
  refresh: [];
}>();
</script>

<template>
  <div class="tabs-wrapper">
    <div class="tabs-container">
      <div class="tab-indicator" :style="{ transform: `translateX(${activeTab * 100}%)` }"></div>

      <div
        class="tab-button"
        :class="{ active: activeTab === TAB_INDEX_SERVERS }"
        @click="emit('update:activeTab', TAB_INDEX_SERVERS)"
      >
        <svg class="tab-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect x="2" y="2" width="20" height="8" rx="2" ry="2"></rect>
          <rect x="2" y="14" width="20" height="8" rx="2" ry="2"></rect>
          <line x1="6" y1="6" x2="6.01" y2="6"></line>
          <line x1="6" y1="18" x2="6.01" y2="18"></line>
        </svg>
        <span>{{ t('instance.servers') }}</span>
      </div>

      <div
        class="tab-button"
        :class="{ active: activeTab === 1 }"
        @click="emit('update:activeTab', 1)"
      >
        <svg class="tab-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="10"></circle>
          <line x1="2" y1="12" x2="22" y2="12"></line>
          <path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"></path>
        </svg>
        <span>{{ t('instance.worlds') }}</span>
      </div>
    </div>

    <div class="header-actions">
      <button v-if="activeTab === TAB_INDEX_SERVERS" class="btn-refresh btn-add-server" @click="emit('addServer')">
        <IconPlus class="add-server-icon" />
        <span>{{ t('instance.add_server') }}</span>
      </button>
      <button v-if="activeTab === TAB_INDEX_SERVERS" class="btn-refresh" @click="emit('refresh')">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M21 2v6h-6"></path>
          <path d="M21 13a9 9 0 1 1-3-7.7L21 8"></path>
        </svg>
        <span>{{ t('instance.refresh') }}</span>
      </button>
    </div>
  </div>
</template>

<style scoped>
.tabs-wrapper {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
}

.tabs-container {
  display: flex;
  position: relative;
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
  border-radius: 999px;
  padding: 4px;
  width: fit-content;
  margin-top: 10px;
}

.tab-indicator {
  position: absolute;
  top: 4px;
  bottom: 4px;
  left: 4px;
  width: calc((100% - 8px) / 2);
  background-color: color-mix(in srgb, var(--accent) 15%, transparent);
  border-radius: 999px;
  transition: transform 0.35s cubic-bezier(0.25, 1, 0.5, 1);
}

.tab-button {
  position: relative;
  z-index: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 6px 20px;
  cursor: pointer;
  border-radius: 999px;
  transition: transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.tab-button:active {
  transform: scale(0.85);
}

.tab-button span {
  font-weight: 600;
  color: var(--color-white);
  font-size: 0.9rem;
  transition: color 0.2s ease;
}

.tab-icon {
  width: 14px;
  height: 14px;
  color: var(--text-muted);
  transition: color 0.2s ease;
}

.tab-button.active span,
.tab-button.active .tab-icon {
  color: var(--accent);
}

.header-actions {
  display: flex;
  gap: 12px;
}

.btn-add-server {
  background-color: transparent;
  border: 1px solid var(--border-line);
  border-radius: 12px;
  color: var(--text-main);
  padding: 8px 16px;
  height: auto;
  transition: background-color 0.2s, transform 0.1s cubic-bezier(0.4, 0.0, 0.2, 1);
  will-change: transform;
}

.btn-add-server:hover {
  background-color: color-mix(in srgb, var(--text-main) 10%, color-mix(in srgb, var(--color-white) 3%, transparent));
  color: var(--text-main);
}

.btn-add-server:active {
  transform: scale(0.85);
}

.add-server-icon {
  width: 18px;
  height: 18px;
}

.btn-refresh {
  background-color: transparent;
  border: none;
  border-radius: 999px;
  height: 38px;
  padding: 0 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  color: var(--text-muted);
  font-weight: 600;
  font-size: 0.95rem;
  cursor: pointer;
  transition: all 0.2s ease;
  margin-top: 10px;
}

.btn-refresh:hover {
  background-color: color-mix(in srgb, color-mix(in srgb, var(--bg-shell) 85%, var(--color-white)) 100%, white 8%);
  color: var(--text-main);
}

.btn-refresh:active {
  transform: scale(0.85);
}

.btn-refresh svg {
  width: 16px;
  height: 16px;
}
</style>
