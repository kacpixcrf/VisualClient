<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { t } from '../../composables/useI18n';

const genericToast = ref<string | null>(null);
const toastId = ref(0);

const downloadTask = ref<string | null>(null);
const downloadProgress = ref<number>(0);

let unlistenToast: UnlistenFn | null = null;
let unlistenProgress: UnlistenFn | null = null;
let toastTimeout: any = null;
let progressTimeout: any = null;

onMounted(async () => {
  unlistenProgress = await listen<{task: string, progress: number}>('download_progress', (event) => {
    downloadTask.value = event.payload.task;
    downloadProgress.value = event.payload.progress;
    
    if (progressTimeout) clearTimeout(progressTimeout);
    
    if (event.payload.progress >= 100 || event.payload.task.toLowerCase() === 'done') {
      progressTimeout = setTimeout(() => {
        downloadTask.value = null;
      }, 2000);
    }
  });

  unlistenToast = await listen<{message: string}>('show_toast', (event) => {
    genericToast.value = event.payload.message;
    toastId.value++;
    if (toastTimeout) {
      clearTimeout(toastTimeout);
    }
    toastTimeout = setTimeout(() => {
      genericToast.value = null;
    }, 3000);
  });
});

onUnmounted(() => {
  if (unlistenToast) unlistenToast();
  if (unlistenProgress) unlistenProgress();
  if (toastTimeout) clearTimeout(toastTimeout);
  if (progressTimeout) clearTimeout(progressTimeout);
});

const translatedTask = computed(() => {
  if (!downloadTask.value) return '';
  const task = downloadTask.value;
  
  if (task.startsWith('Using existing Java')) return t('downloads.existing_java', { version: task.split(' ')[3] });
  if (task.startsWith('Downloading Java')) return t('downloads.downloading_java', { version: task.split(' ')[2] });
  if (task.startsWith('Extracting Java')) return t('downloads.extracting_java', { version: task.split(' ')[2] });
  
  const libMatch = task.match(/Downloading libraries \((.*)\)/);
  if (libMatch) return t('downloads.libraries', { progress: libMatch[1] });
  
  const assetMatch = task.match(/Downloading assets \((.*)\)/);
  if (assetMatch) return t('downloads.assets', { progress: assetMatch[1] });
  
  const fabMatch = task.match(/Downloading Fabric libraries \((.*)\)/);
  if (fabMatch) return t('downloads.fabric', { progress: fabMatch[1] });
  
  if (task === 'Downloading Forge Installer') return t('downloads.forge_installer');
  if (task === 'Running Forge Installer (this may take a while)') return t('downloads.forge_running');
  if (task === 'Downloading Minecraft') return t('downloads.minecraft');
  if (task.startsWith('Launching ')) return t('downloads.launching', { name: task.replace('Launching ', '') });
  if (task === 'Done') return t('downloads.done');
  
  return task;
});
</script>

<template>
  <TransitionGroup name="toast" tag="div" class="notifications-wrapper">
    <div v-if="downloadTask" key="download-toast" class="notification-toast">
      <div class="toast-header">
        <span class="task-name">{{ translatedTask }}</span>
        <span class="task-pct" v-if="downloadProgress < 100">{{ downloadProgress }}%</span>
      </div>
      <div class="progress-bar-bg" v-if="downloadProgress < 100">
        <div class="progress-bar-fill" :style="{ width: downloadProgress + '%' }"></div>
      </div>
    </div>
    
    <div v-if="genericToast" :key="'toast-' + toastId" class="notification-toast simple-toast">
      <div class="toast-header">
        <span class="task-name">{{ genericToast }}</span>
      </div>
      <div class="progress-bar-bg timeout-bg">
        <div class="progress-bar-fill timeout-fill"></div>
      </div>
    </div>
  </TransitionGroup>
</template>

<style scoped>
.notifications-wrapper {
  position: fixed;
  top: 40px;
  right: 20px;
  display: flex;
  flex-direction: column;
  gap: 12px;
  z-index: 1000;
}

.notification-toast {
  position: relative;
  overflow: hidden;
  width: 320px;
  background-color: var(--bg-shell);
  border: 1px solid var(--border-line);
  border-radius: 12px;
  padding: 16px;
  box-shadow: 0 10px 30px color-mix(in srgb, var(--color-black) 50%, transparent);
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.toast-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 0.95rem;
  font-weight: 600;
  color: var(--text-main);
}

.progress-bar-bg {
  width: 100%;
  height: 6px;
  background-color: color-mix(in srgb, var(--color-white) 3%, transparent);
  border-radius: 3px;
  overflow: hidden;
}

.timeout-bg {
  position: absolute;
  bottom: 0;
  left: 0;
  width: 100%;
  height: 4px;
  border-radius: 0;
  margin: 0;
}

.progress-bar-fill {
  height: 100%;
  background-color: var(--accent);
  transition: width 0.15s linear;
}

.timeout-fill {
  width: 100%;
  transition: none;
  animation: shrink 3s linear forwards;
}

@keyframes shrink {
  from { width: 100%; }
  to { width: 0%; }
}

.toast-enter-active,
.toast-leave-active {
  transition: opacity 0.3s, transform 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.toast-leave-active {
  position: absolute;
}

.toast-move {
  transition: transform 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translateX(20px) scale(0.85);
}
</style>
