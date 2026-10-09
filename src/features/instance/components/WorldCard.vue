<script setup lang="ts">
import { t } from '../../../composables/useI18n';
import IconPlay from '../../../components/icons/IconPlay.vue';
import type { World } from '../types';

defineProps<{
  world: World;
  isRunning: boolean;
  isStarting: boolean;
  supportsWorldQuickPlay: boolean;
}>();

const emit = defineEmits<{
  play: [folderName: string];
}>();

const formatLastPlayed = (timestamp: number): string => {
  if (!timestamp) return t('instance.played_just_now');

  const now = Date.now();
  const diffMs = now - timestamp;

  if (diffMs < 0) return t('instance.played_just_now');

  const diffMinutes = Math.floor(diffMs / 60000);
  if (diffMinutes < 1) return t('instance.played_just_now');
  if (diffMinutes < 60) return t('instance.played_minutes_ago', { time: diffMinutes.toString() });

  const diffHours = Math.floor(diffMinutes / 60);
  if (diffHours < 24) return t('instance.played_hours_ago', { time: diffHours.toString() });

  const diffDays = Math.floor(diffHours / 24);
  if (diffDays < 30) return t('instance.played_days_ago', { time: diffDays.toString() });

  const diffMonths = Math.floor(diffDays / 30);
  if (diffMonths < 12) return t('instance.played_months_ago', { time: diffMonths.toString() });

  const diffYears = Math.floor(diffDays / 365);
  return t('instance.played_years_ago', { time: diffYears.toString() });
};
</script>

<template>
  <div class="server-card">
    <img
      v-if="world.icon_base64"
      :src="`data:image/png;base64,${world.icon_base64}`"
      class="item-icon server-icon-img pixelated"
      alt="World Icon"
    />
    <img
      v-else
      src="/noicon.png"
      class="item-icon server-icon-img pixelated"
      alt="Default World Icon"
    />
    <div class="server-card-content">
      <div class="server-details">
        <div class="world-title-row">
          <h4 class="item-name">{{ world.name }}</h4>
          <span class="world-folder-name">({{ world.folder_name }})</span>
        </div>
      </div>
      <div class="server-motd-container">
        <div class="server-motd is-world">{{ formatLastPlayed(world.last_played) }}</div>
      </div>
      <div v-if="supportsWorldQuickPlay" class="server-actions">
        <button
          class="btn-play-server"
          :disabled="isRunning || isStarting"
          @click="emit('play', world.folder_name)"
        >
          <IconPlay class="play-icon-server" />
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.server-card {
  background-color: var(--bg-shell);
  border-radius: 16px;
  padding: 16px;
  display: flex;
  align-items: center;
  gap: 16px;
  cursor: default;
}

.server-card-content {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 24px;
  min-width: 0;
}

.server-details {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 8px;
  width: 180px;
  flex-shrink: 0;
}

.world-title-row {
  display: flex;
  align-items: baseline;
  gap: 8px;
}

.world-folder-name {
  font-size: 0.85rem;
  color: var(--text-muted);
}

.item-name {
  margin: 0;
  font-size: 1.05rem;
  font-weight: 600;
  color: var(--text-main);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.server-motd-container {
  flex: 1;
  display: flex;
  justify-content: flex-start;
}

.server-motd.is-world {
  display: flex;
  align-items: center;
  font-size: 0.85rem;
  color: var(--text-muted);
}

.server-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.btn-play-server {
  width: 38px;
  height: 38px;
  padding: 0;
  border-radius: 50%;
  background-color: var(--success);
  color: var(--color-black);
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  cursor: pointer;
  transition: background-color 0.2s ease, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
  flex-shrink: 0;
  will-change: transform;
}

.btn-play-server:hover:not(:disabled) {
  background-color: color-mix(in srgb, var(--success) 85%, var(--color-black));
}

.btn-play-server:active:not(:disabled) {
  transform: scale(0.85);
}

.btn-play-server:disabled {
  background-color: color-mix(in srgb, var(--bg-shell) 85%, var(--color-white));
  color: var(--text-muted);
  cursor: not-allowed;
  transform: none;
}

.play-icon-server {
  width: 16px;
  height: 16px;
  margin-left: 2px;
  pointer-events: none;
}

.item-icon {
  width: 48px;
  height: 48px;
  background-color: color-mix(in srgb, var(--bg-shell) 85%, var(--color-white));
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
  flex-shrink: 0;
}

.server-icon-img {
  object-fit: cover;
  background-color: transparent;
}

.pixelated {
  image-rendering: pixelated;
}
</style>
