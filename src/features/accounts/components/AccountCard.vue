<script setup lang="ts">
const MC_HEADS_URL = 'https://mc-heads.net/head';
const FALLBACK_AVATAR = '/steve.png';

interface Account {
  username: string;
  active: boolean;
  type?: string;
}

defineProps<{ account: Account }>();

defineEmits<{
  select: [username: string];
  delete: [username: string];
}>();

const handleAvatarError = (e: Event) => {
  (e.target as HTMLImageElement).src = FALLBACK_AVATAR;
};
</script>

<template>
  <div
    class="account-card"
    :class="{ active: account.active }"
    @click="$emit('select', account.username)"
  >
    <div class="account-info-left">
      <div class="account-avatar">
        <img
          :src="`${MC_HEADS_URL}/${account.username}`"
          :alt="account.username"
          @error="handleAvatarError"
          class="avatar-img pixelated"
        />
      </div>
      <div class="account-name-group">
        <span class="account-type">{{ account.type || 'Offline' }}</span>
        <h3 class="account-name">{{ account.username }}</h3>
      </div>
    </div>

    <div class="actions-right">
      <div class="delete-action" @click.stop="$emit('delete', account.username)">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="3 6 5 6 21 6"></polyline>
          <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
        </svg>
      </div>
      <div class="radio-circle">
        <div class="radio-inner" v-if="account.active"></div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.account-card {
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
  position: relative;
  will-change: transform;
  transform: translateZ(0);
}

.account-card:hover {
  background-color: color-mix(in srgb, var(--bg-shell) 85%, var(--color-black));
}

.account-card:active:not(:has(.delete-action:active)) {
  transform: scale(0.85);
}

.account-info-left {
  display: flex;
  align-items: center;
  gap: 14px;
  flex: 1;
  min-width: 0;
}

.account-name-group {
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 2px;
  flex: 1;
  min-width: 0;
}

.account-type {
  font-size: 0.75rem;
  color: var(--text-muted);
  text-transform: uppercase;
  font-weight: 600;
  letter-spacing: 0.05em;
}

.account-name {
  margin: 0;
  font-size: 1rem;
  font-weight: 600;
  color: var(--color-white);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.account-avatar {
  width: 44px;
  height: 44px;
  border-radius: 12px;
  background-color: var(--bg-shell);
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  flex-shrink: 0;
}

.avatar-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  image-rendering: pixelated;
}

.actions-right {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
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

.account-card:hover .delete-action {
  opacity: 1;
}

.delete-action:hover {
  background-color: color-mix(in srgb, var(--danger) 15%, transparent);
  color: var(--danger);
}

.delete-action:active {
  transform: scale(0.85);
}

.radio-circle {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  border: 2px solid var(--border-line);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
  background-color: color-mix(in srgb, var(--color-white) 3%, transparent);
}

.account-card.active .radio-circle {
  border-color: var(--text-main);
}

.radio-inner {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background-color: var(--text-main);
}
</style>
