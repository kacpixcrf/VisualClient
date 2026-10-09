<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { isLanguageLoaded, t } from '../../composables/useI18n';
import { updateState, updateProgress } from '../../composables/useUpdater';

const isInitialLoad = ref(true);
const initialProgress = ref(0);

const isVisible = computed(() => isInitialLoad.value || updateState.value !== 'idle');
const displayProgress = computed(() => {
  if (updateState.value !== 'idle' && updateState.value !== 'checking') {
    return updateProgress.value;
  }
  return initialProgress.value;
});

const statusText = computed(() => {
  switch (updateState.value) {
    case 'checking': return t('updater.checking');
    case 'downloading': return t('updater.downloading') + ` ${updateProgress.value}%`;
    case 'restarting': return t('updater.restarting');
    default: return '';
  }
});

onMounted(() => {
  const startTime = Date.now();
  
  const progressInterval = setInterval(() => {
    if (initialProgress.value < 85) {
      initialProgress.value += Math.random() * 8 + 2;
    }
  }, 40);
  
  const check = setInterval(() => {
    if (isLanguageLoaded.value && Date.now() - startTime >= 500 && updateState.value === 'idle') {
      initialProgress.value = 100;
      setTimeout(() => {
        isInitialLoad.value = false;
        clearInterval(check);
        clearInterval(progressInterval);
      }, 200);
    }
  }, 50);
});
</script>

<template>
  <Transition name="fade">
    <div class="splash-screen" v-if="isVisible">
      <div class="splash-content">
        <div class="splash-logo-row">
          <img src="/icon.svg" onerror="this.src='/vite.svg'" alt="VisualClient" class="splash-icon" />
          <div class="splash-title">
            <span>visual</span>
            <span class="text-accent">client</span>
          </div>
        </div>
        
        <div class="progress-container">
          <div class="progress-fill" :style="{ width: displayProgress + '%' }"></div>
        </div>
        
        <div class="updater-status-wrapper">
          <Transition name="fade-text">
            <div class="updater-status" v-if="statusText">{{ statusText }}</div>
          </Transition>
        </div>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.splash-screen {
  position: fixed;
  inset: 0;
  z-index: 99999;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(to bottom, color-mix(in srgb, var(--accent) 15%, var(--bg-shell)), var(--bg-shell) 70%);
}

.splash-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 32px;
  animation: splash-entrance 0.5s cubic-bezier(0.34, 1.56, 0.64, 1) forwards;
  will-change: transform, opacity;
  transform: translateZ(0);
}

.splash-logo-row {
  display: flex;
  align-items: center;
  gap: 4px;
}

@keyframes splash-entrance {
  0% { opacity: 0; transform: scale(0.85) translateY(10px); }
  100% { opacity: 1; transform: scale(1) translateY(0); }
}

.splash-icon {
  width: 48px;
  height: 48px;
  object-fit: contain;
}

.splash-title {
  display: flex;
  gap: 10px;
  font-size: 2.5rem;
  font-weight: 600;
  color: var(--text-main);
  letter-spacing: -0.02em;
}

.text-accent {
  color: var(--accent);
}

.progress-container {
  width: 220px;
  height: 6px;
  background-color: color-mix(in srgb, var(--color-black) 30%, transparent);
  border-radius: 999px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background-color: var(--accent);
  border-radius: 999px;
  transition: width 0.15s ease-out;
}

.updater-status-wrapper {
  height: 20px;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  margin-top: -16px;
}

.updater-status {
  font-size: 0.95rem;
  font-weight: 500;
  color: var(--text-muted);
  letter-spacing: 0.02em;
}

.fade-text-enter-active,
.fade-text-leave-active {
  transition: opacity 0.3s ease, transform 0.3s ease;
}

.fade-text-enter-from,
.fade-text-leave-to {
  opacity: 0;
  transform: translateY(-5px);
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.4s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
