<script setup lang="ts">
import IconInstance from '../../../components/icons/IconInstance.vue';
import IconGamepad from '../../../components/icons/IconGamepad.vue';
import IconPlay from '../../../components/icons/IconPlay.vue';
import IconStop from '../../../components/icons/IconStop.vue';
import { capitalizeLoader } from '../../../utils/minecraft';
import type { Instance } from '../../../composables/useInstances';

const props = defineProps<{
  instance: Instance;
  isRunning: boolean;
}>();

const emit = defineEmits<{
  open: [id: string];
  quickPlay: [instance: Instance];
  delete: [instance: Instance];
}>();
</script>

<template>
  <div class="instance-card" @click="emit('open', props.instance.id)">
    <div class="instance-info-left">
      <div class="instance-avatar">
        <IconInstance class="library-icon-svg" />
        <div class="quick-play-overlay" :class="{ 'is-running': props.isRunning }" @click.stop="emit('quickPlay', props.instance)">
          <IconStop v-if="props.isRunning" class="quick-stop-icon" />
          <IconPlay v-else class="quick-play-icon" />
        </div>
      </div>
      <div class="instance-info">
        <h3 class="instance-name">{{ props.instance.name }}</h3>
        <div class="instance-loader">
          <IconGamepad class="gamepad-icon" />
          <span>{{ capitalizeLoader(props.instance.loader) }} {{ props.instance.version }}</span>
        </div>
      </div>
    </div>
    <div class="delete-action" @click.stop="emit('delete', props.instance)">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="3 6 5 6 21 6"></polyline>
        <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
      </svg>
    </div>
  </div>
</template>

<style scoped>
.instance-card {
  background-color: var(--bg-shell);
  border: none;
  border-radius: 16px;
  padding: 14px 16px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  cursor: pointer;
  transition: background-color 0.2s ease, transform 0.2s ease;
}

.instance-card:hover {
  background-color: color-mix(in srgb, var(--bg-shell) 85%, var(--color-black));
}

.instance-card:active:not(:has(.delete-action:active)):not(:has(.quick-play-overlay:active)) {
  transform: scale(0.85);
}

.instance-info-left {
  display: flex;
  align-items: center;
  gap: 14px;
  flex: 1;
  min-width: 0;
}

.instance-avatar {
  width: 44px;
  height: 44px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  position: relative;
}

.library-icon-svg {
  width: 40px;
  height: 40px;
  transition: opacity 0.2s;
}

.quick-play-overlay {
  position: absolute;
  width: 32px;
  height: 32px;
  background-color: var(--success);
  color: var(--color-black);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transform: scale(0.85);
  transition: background-color 0.2s ease, color 0.2s ease, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
  cursor: pointer;
  z-index: 2;
}

.instance-card:hover .quick-play-overlay {
  opacity: 1;
  transform: scale(1);
}

.quick-play-overlay.is-running {
  opacity: 1;
  transform: scale(1);
  background-color: var(--danger);
}

.quick-play-overlay.is-running:hover {
  background-color: color-mix(in srgb, var(--danger) 85%, black);
}

.quick-play-overlay:hover {
  background-color: color-mix(in srgb, var(--success) 85%, var(--color-black));
}

.quick-play-overlay:active {
  transform: scale(0.85) !important;
}

.quick-play-icon {
  width: 16px;
  height: 16px;
  margin-left: 2px;
  pointer-events: none;
}

.quick-stop-icon {
  width: 16px;
  height: 16px;
  color: var(--color-black);
  pointer-events: none;
}

.instance-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
  overflow: hidden;
}

.instance-name {
  margin: 0;
  font-size: 1rem;
  font-weight: 600;
  color: var(--color-white);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.instance-loader {
  font-size: 0.85rem;
  color: var(--text-muted);
  display: flex;
  align-items: center;
  gap: 6px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.gamepad-icon {
  width: 14px;
  height: 14px;
  flex-shrink: 0;
  color: var(--text-muted);
}

.delete-action {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
  transition: all 0.2s;
  flex-shrink: 0;
  opacity: 0;
}

.instance-card:hover .delete-action {
  opacity: 1;
}

.delete-action:hover {
  background-color: color-mix(in srgb, var(--danger) 15%, transparent);
  color: var(--danger);
}

.delete-action:active {
  transform: scale(0.85);
}
</style>
