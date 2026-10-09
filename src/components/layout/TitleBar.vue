<script setup lang="ts">
import { ref } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { checkForUpdates } from '../../composables/useUpdater';
import { t } from '../../composables/useI18n';
import RunningInstanceTile from './titlebar/RunningInstanceTile.vue';
import WindowControls from './titlebar/WindowControls.vue';
import TitlebarTooltip from './titlebar/TitlebarTooltip.vue';
import { APP_ICON_PATH, APP_ICON_FALLBACK_PATH } from '../../constants';

const emit = defineEmits(['openInstance']);

const appWindow = getCurrentWindow();

const hoveredTooltip = ref('');
const tooltipLeft = ref(0);

const handleMouseOver = (e: MouseEvent) => {
  const target = (e.target as HTMLElement).closest('[data-tooltip]');
  if (target) {
    hoveredTooltip.value = target.getAttribute('data-tooltip') || '';
    const rect = target.getBoundingClientRect();
    tooltipLeft.value = rect.left + rect.width / 2;
  }
};

const handleMouseOut = (e: MouseEvent) => {
  const target = (e.target as HTMLElement).closest('[data-tooltip]');
  const related = e.relatedTarget as Node | null;
  if (target && related && target.contains(related)) {
    return;
  }
  hoveredTooltip.value = '';
};

const handleUpdateCheck = () => {
  checkForUpdates(true);
};

const startDrag = (e: MouseEvent) => {
  const target = e.target as HTMLElement;
  if (!target.closest('.window-control') && !target.closest('.active-instance-tile')) {
    appWindow.startDragging();
  }
};
</script>

<template>
  <div class="titlebar" @mousedown="startDrag" @mouseover="handleMouseOver" @mouseout="handleMouseOut">
    <div class="titlebar-left">
      <img :src="APP_ICON_PATH" :onerror="`this.src='${APP_ICON_FALLBACK_PATH}'`" alt="" class="app-icon" />
      <span class="app-title">visual <span class="text-accent">client</span></span>
      <div class="title-separator"></div>
      <span class="app-author">by kacpixcrf</span>
    </div>

    <div class="titlebar-controls">
      <RunningInstanceTile @open-instance="emit('openInstance', $event)" />

      <div
        class="window-control update-control"
        @click="handleUpdateCheck"
        :data-tooltip="t('updater.check_tooltip')"
      >
        <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" style="pointer-events: none;">
          <path d="M4 12V10a4 4 0 0 1 4-4h12"></path>
          <polyline points="16 2 20 6 16 10"></polyline>
          <path d="M20 12V14a4 4 0 0 1-4 4H4"></path>
          <polyline points="8 14 4 18 8 22"></polyline>
        </svg>
      </div>
      <div class="controls-separator"></div>
      <WindowControls />
    </div>

    <Transition name="tooltip-fade">
      <TitlebarTooltip
        v-if="hoveredTooltip"
        :text="hoveredTooltip"
        :left-position="tooltipLeft"
      />
    </Transition>
  </div>
</template>

<style scoped>
.titlebar {
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 8px 0 16px;
  user-select: none;
  background-color: transparent;
}

.titlebar-left {
  display: flex;
  align-items: center;
  gap: 4px;
}

.app-icon {
  width: 26px;
  height: 26px;
  pointer-events: none;
}

.app-title {
  font-weight: 600;
  font-size: 16px;
  letter-spacing: 0.5px;
  pointer-events: none;
}

.text-accent {
  color: var(--accent);
}

.title-separator {
  width: 1px;
  height: 16px;
  background-color: var(--border-line);
  margin: 0 8px;
}

.app-author {
  font-size: 13px;
  color: var(--text-muted);
  font-weight: 500;
  pointer-events: none;
}

.titlebar-controls {
  display: flex;
  align-items: center;
  gap: 4px;
}

.controls-separator {
  width: 1px;
  height: 16px;
  background-color: var(--border-line);
  margin: 0 4px;
}

.window-control {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
  transition: background-color 0.2s ease, color 0.2s ease, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
  cursor: pointer;
}

.window-control:hover {
  background-color: color-mix(in srgb, var(--color-white) 10%, transparent);
  color: var(--text-main);
}

.window-control:active {
  transform: scale(0.85);
}

.tooltip-fade-enter-active, .tooltip-fade-leave-active {
  transition: opacity 0.2s, transform 0.2s;
}
.tooltip-fade-enter-from, .tooltip-fade-leave-to {
  opacity: 0;
  transform: translate(-50%, -4px);
}
.tooltip-fade-enter-to, .tooltip-fade-leave-from {
  opacity: 1;
  transform: translate(-50%, 0);
}
</style>
