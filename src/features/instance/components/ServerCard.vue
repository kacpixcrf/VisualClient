<script setup lang="ts">
import { t } from '../../../composables/useI18n';
import { useServerMenu } from '../composables/useServerMenu';
import IconPlay from '../../../components/icons/IconPlay.vue';
import IconTrash from '../../../components/icons/IconTrash.vue';
import type { Server } from '../types';
import { MCSRVSTAT_ICON_URL } from '../../../constants';

const props = defineProps<{
  server: Server;
  isRunning: boolean;
  isStarting: boolean;
}>();

const emit = defineEmits<{
  play: [ip: string];
  edit: [server: Server];
  remove: [ip: string];
  copyAddress: [ip: string];
}>();

const { menuOpen, toggleMenu, closeMenu } = useServerMenu();

const handleIconError = (e: Event) => {
  const target = e.target as HTMLImageElement;
  target.src = '/noicon.png';
};
</script>


<template>
  <div class="server-card" @click.away="closeMenu">
    <img
      v-if="server.icon_base64"
      :src="`data:image/png;base64,${server.icon_base64}`"
      class="item-icon server-icon-img pixelated"
      alt="Server Icon"
    />
    <img
      v-else
      :src="`${MCSRVSTAT_ICON_URL}/${server.ip}`"
      @error="handleIconError"
      class="item-icon server-icon-img pixelated"
      alt="Server Icon"
    />
    <div class="server-card-content">
      <div class="server-details">
        <h4 class="item-name">{{ server.name }}</h4>
        <div class="server-ip">{{ server.ip }}</div>
      </div>
      <div class="server-motd-container">
        <div v-if="!server.motdHtml && server.loadingMotd" class="motd-loading-state">
          <svg class="motd-spinner" viewBox="0 0 50 50">
            <circle class="path" cx="25" cy="25" r="20" fill="none" stroke-width="5"></circle>
          </svg>
          <span>{{ t('instance.loading') }}</span>
        </div>
        <div v-else-if="!server.online && !server.loadingMotd" class="server-motd is-error">
          {{ t('instance.cant_connect') }}
        </div>
        <div
          v-else
          class="server-motd"
          :class="{ 'is-loading': server.loadingMotd }"
          v-html="server.motdHtml"
        ></div>
      </div>
      <div class="server-actions">
        <button
          class="btn-play-server"
          :disabled="isRunning || isStarting"
          @click="emit('play', server.ip)"
        >
          <IconPlay class="play-icon-server" />
        </button>

        <div class="server-menu-wrapper" @click.stop>
          <button class="btn-server-menu" @click="toggleMenu">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="12" cy="12" r="1"></circle>
              <circle cx="12" cy="5" r="1"></circle>
              <circle cx="12" cy="19" r="1"></circle>
            </svg>
          </button>

          <Transition name="dropdown">
            <div v-if="menuOpen" class="server-dropdown-menu">
              <button class="dropdown-item" @click="emit('copyAddress', server.ip); closeMenu()">
                <svg class="dropdown-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                  <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                </svg>
                {{ t('instance.copy_address') }}
              </button>
              <button class="dropdown-item" @click="emit('edit', server); closeMenu()">
                <svg class="dropdown-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M12 20h9"></path>
                  <path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"></path>
                </svg>
                {{ t('instance.edit_server') }}
              </button>
              <div class="dropdown-divider"></div>
              <button class="dropdown-item item-danger" @click="emit('remove', server.ip); closeMenu()">
                <IconTrash class="dropdown-icon" />
                {{ t('instance.remove_server') }}
              </button>
            </div>
          </Transition>
        </div>
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

.item-name {
  margin: 0;
  font-size: 1.05rem;
  font-weight: 600;
  color: var(--text-main);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.server-ip {
  font-size: 0.85rem;
  color: var(--text-muted);
  background-color: color-mix(in srgb, var(--bg-shell) 85%, var(--color-white));
  padding: 4px 10px;
  border-radius: 8px;
  font-family: monospace;
  white-space: nowrap;
}

.server-motd-container {
  flex: 1;
  display: flex;
  justify-content: flex-start;
}

.server-motd {
  font-size: 0.85rem;
  color: var(--text-muted);
  white-space: pre-wrap;
  line-height: 1.4;
  height: calc(1.4em * 2);
  overflow: hidden;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  transition: opacity 0.2s ease;
}

.server-motd.is-error {
  display: flex;
  align-items: center;
  -webkit-line-clamp: unset;
  color: var(--danger) !important;
}

.server-motd.is-loading {
  opacity: 0.5;
}

.motd-loading-state {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  color: var(--text-muted);
  font-size: 0.9rem;
  font-weight: 500;
  height: calc(1.4em * 2);
}

.motd-spinner {
  animation: motd-rotate 2s linear infinite;
  width: 18px;
  height: 18px;
}

.motd-spinner .path {
  stroke: var(--accent);
  stroke-linecap: round;
  animation: motd-dash 1.5s ease-in-out infinite;
}

@keyframes motd-rotate {
  100% { transform: rotate(360deg); }
}

@keyframes motd-dash {
  0% { stroke-dasharray: 1, 150; stroke-dashoffset: 0; }
  50% { stroke-dasharray: 90, 150; stroke-dashoffset: -35; }
  100% { stroke-dasharray: 90, 150; stroke-dashoffset: -124; }
}

.server-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.server-menu-wrapper {
  position: relative;
}

.btn-server-menu {
  background: transparent;
  color: var(--text-muted);
  border: none;
  width: 36px;
  height: 36px;
  border-radius: 50%;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.2s ease, color 0.2s ease;
  will-change: transform;
  backface-visibility: hidden;
}

.btn-server-menu:hover {
  background-color: color-mix(in srgb, var(--text-main) 10%, transparent);
  color: var(--text-main);
}

.btn-server-menu:active {
  transform: scale(0.85);
}

.server-dropdown-menu {
  position: absolute;
  top: calc(100% + 12px);
  right: 0;
  background-color: var(--bg-shell);
  border: 1px solid var(--border-line);
  border-radius: 12px;
  padding: 6px;
  min-width: 200px;
  box-shadow: 0 10px 30px color-mix(in srgb, var(--color-black) 50%, transparent);
  z-index: 100;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.server-dropdown-menu::before {
  content: '';
  position: absolute;
  top: -6px;
  right: 14px;
  width: 12px;
  height: 12px;
  background-color: var(--bg-shell);
  border-top: 1px solid var(--border-line);
  border-left: 1px solid var(--border-line);
  transform: rotate(45deg);
  z-index: -1;
  border-bottom-right-radius: 2px;
}

.dropdown-enter-active,
.dropdown-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease-out;
}
.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translate(10px, -10px);
}

.dropdown-item {
  background: transparent;
  border: none;
  color: var(--text-muted);
  padding: 10px 12px;
  border-radius: 10px;
  font-size: 0.9rem;
  font-weight: 600;
  text-align: left;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 10px;
  transition: background-color 0.2s, color 0.2s;
}

.dropdown-item:hover {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
  color: var(--text-main);
}

.dropdown-icon {
  width: 16px;
  height: 16px;
  stroke-width: 2.5;
  flex-shrink: 0;
}

.dropdown-divider {
  height: 1px;
  background-color: var(--border-line);
  margin: 4px 12px;
}

.item-danger {
  color: var(--danger);
}

.item-danger:hover {
  background-color: var(--danger);
  color: var(--color-black);
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
