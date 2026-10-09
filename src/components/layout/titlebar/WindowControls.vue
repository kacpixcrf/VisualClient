<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';

const appWindow = getCurrentWindow();
const isMaximized = ref(false);

const updateMaximized = async () => {
  isMaximized.value = await appWindow.isMaximized();
};

let unlistenResized: (() => void) | null = null;

onMounted(async () => {
  await updateMaximized();
  unlistenResized = await appWindow.onResized(() => {
    updateMaximized();
  });
});

onUnmounted(() => {
  if (unlistenResized) unlistenResized();
});

const minimize = () => appWindow.minimize();

const toggleMaximize = async () => {
  await appWindow.toggleMaximize();
  updateMaximized();
};

const close = () => appWindow.close();
</script>

<template>
  <div class="window-controls">
    <div class="window-control" @click="minimize">
      <svg width="16" height="16" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/01/svg">
        <rect x="2" y="5" width="8" height="2" rx="1" fill="currentColor"/>
      </svg>
    </div>
    <div class="window-control" @click="toggleMaximize">
      <svg v-if="!isMaximized" width="16" height="16" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/01/svg">
        <rect x="2.5" y="2.5" width="7" height="7" rx="1" stroke="currentColor" stroke-width="1.5"/>
      </svg>
      <svg v-else width="16" height="16" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/01/svg">
        <path d="M4.5 4.5V2.5C4.5 1.94772 4.94772 1.5 5.5 1.5H9.5C10.0523 1.5 10.5 1.94772 10.5 2.5V6.5C10.5 7.05228 10.0523 7.5 9.5 7.5H7.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
        <rect x="1.5" y="4.5" width="6" height="6" rx="1" stroke="currentColor" stroke-width="1.5"/>
      </svg>
    </div>
    <div class="window-control close-control" @click="close">
      <svg width="16" height="16" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/01/svg">
        <path d="M3 3L9 9M9 3L3 9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </div>
  </div>
</template>

<style scoped>
.window-controls {
  display: flex;
  align-items: center;
  gap: 4px;
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

.close-control:hover {
  background-color: var(--danger);
  color: white;
}
</style>
