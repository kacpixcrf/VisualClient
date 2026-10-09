<script setup lang="ts">
import { t } from '../../../composables/useI18n';
import IconInstance from '../../../components/icons/IconInstance.vue';
import IconPlay from '../../../components/icons/IconPlay.vue';
import IconStop from '../../../components/icons/IconStop.vue';
import IconFolder from '../../../components/icons/IconFolder.vue';
import type { Instance } from '../../../composables/useInstances';

defineProps<{
  instance: Instance;
  isRunning: boolean;
  isStarting: boolean;
}>();

const emit = defineEmits<{
  play: [];
  openSettings: [instance: Instance];
  openFolder: [];
}>();
</script>

<template>
  <header class="instance-header">
    <div class="header-left">
      <div class="library-icon">
        <IconInstance class="library-icon-svg" />
      </div>
      <div class="instance-info">
        <h1 class="title">{{ instance.name }}</h1>
        <div class="instance-loader">
          <svg class="gamepad-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="6" y1="11" x2="10" y2="11"></line>
            <line x1="8" y1="9" x2="8" y2="13"></line>
            <line x1="15" y1="12" x2="15.01" y2="12"></line>
            <line x1="18" y1="10" x2="18.01" y2="10"></line>
            <path d="M17.32 5H6.68a4 4 0 0 0-3.978 3.59c-.006.052-.01.101-.017.152C2.604 9.416 2 14.456 2 16a3 3 0 0 0 3 3c1 0 1.5-.5 2-1l1.414-1.414A2 2 0 0 1 9.828 16h4.344a2 2 0 0 1 1.414.586L17 18c.5.5 1 1 2 1a3 3 0 0 0 3-3c0-1.544-.604-6.584-.685-7.258-.007-.05-.011-.1-.017-.151A4 4 0 0 0 17.32 5z"></path>
          </svg>
          <span class="subtitle">{{ instance.loader.charAt(0).toUpperCase() + instance.loader.slice(1) }} {{ instance.version }}</span>
        </div>
      </div>
    </div>
    <div class="actions">
      <button
        class="btn-play"
        :class="{ 'is-running': isRunning, 'is-starting': isStarting }"
        @click="emit('play')"
      >
        <IconStop v-if="isRunning || isStarting" class="play-icon" />
        <IconPlay v-else class="play-icon" />
        {{ isRunning ? (t('instance.stop') || 'Stop') : isStarting ? (t('instance.starting') || 'Starting...') : (t('instance.play') || 'Play') }}
      </button>
      <button class="btn-settings" @click="emit('openSettings', instance)">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="3"></circle>
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
        </svg>
      </button>
      <button class="btn-settings" @click="emit('openFolder')">
        <IconFolder class="folder-icon" />
      </button>
    </div>
  </header>
</template>

<style scoped>
.instance-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0;
  padding-bottom: 32px;
  border-bottom: 1px solid var(--border-line);
}

.header-left {
  display: flex;
  align-items: center;
  gap: 20px;
}

.library-icon {
  width: 72px;
  height: 72px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  border: 4px solid color-mix(in srgb, var(--bg-shell) 85%, var(--color-white));
  border-radius: 16px;
  background-color: color-mix(in srgb, var(--color-white) 3%, transparent);
}

.library-icon-svg {
  width: 72px;
  height: 72px;
}

.instance-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.title {
  margin: 0;
  font-size: 1.8rem;
  font-weight: 600;
  color: var(--text-main);
}

.instance-loader {
  display: flex;
  align-items: center;
  gap: 6px;
  color: var(--text-muted);
}

.gamepad-icon {
  width: 18px;
  height: 18px;
  color: var(--text-muted);
}

.subtitle {
  font-size: 1rem;
  color: var(--text-muted);
  font-weight: 600;
}

.actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

.btn-play {
  padding: 0 24px;
  height: 44px;
  background-color: var(--success);
  color: var(--color-black);
  border: none;
  border-radius: 16px;
  font-size: 1.1rem;
  font-weight: 600;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: background-color 0.2s ease, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.play-icon {
  width: 20px;
  height: 20px;
}

.btn-play:hover {
  background-color: color-mix(in srgb, var(--success) 85%, var(--color-black));
}

.btn-play:active {
  transform: scale(0.85);
}

.btn-play.is-running {
  background-color: var(--danger);
  color: var(--color-black);
}

.btn-play.is-running:hover {
  background-color: color-mix(in srgb, var(--danger) 85%, black);
  color: var(--color-black);
}

.btn-play.is-starting {
  background-color: color-mix(in srgb, var(--success) 50%, transparent);
  color: color-mix(in srgb, var(--color-black) 70%, transparent);
  cursor: default;
}

.btn-settings {
  width: 48px;
  height: 48px;
  background-color: color-mix(in srgb, var(--bg-shell) 85%, var(--color-white));
  color: var(--text-secondary);
  border: none;
  border-radius: 50%;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.2s ease, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.folder-icon {
  width: 20px;
  height: 20px;
}

.btn-settings:hover {
  background-color: color-mix(in srgb, var(--bg-shell) 95%, var(--color-white));
}

.btn-settings:active {
  transform: scale(0.85);
}
</style>
