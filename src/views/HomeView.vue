<script setup lang="ts">
import { t } from '../composables/useI18n';
import { useModrinth } from '../features/home/composables/useModrinth';
import ModrinthSection from '../features/home/components/ModrinthSection.vue';
import ModrinthCard from '../features/home/components/ModrinthCard.vue';

const { shaders, mods, modpacks, isLoadingShaders, isLoadingMods, isLoadingModpacks, isOffline } = useModrinth();
</script>

<template>
  <div class="home-view">
    <h1 class="welcome-title">{{ t('home.welcome') }}</h1>

    <div v-if="isOffline" class="offline-state">
      <svg class="offline-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <line x1="1" y1="1" x2="23" y2="23"></line>
        <path d="M16.72 11.06A10.94 10.94 0 0 1 19 12.55"></path>
        <path d="M5 12.55a10.94 10.94 0 0 1 5.17-2.39"></path>
        <path d="M10.71 5.05A16 16 0 0 1 22.58 9"></path>
        <path d="M1.42 9a15.91 15.91 0 0 1 4.7-2.88"></path>
        <path d="M8.53 16.11a6 6 0 0 1 6.95 0"></path>
        <line x1="12" y1="20" x2="12.01" y2="20"></line>
      </svg>
      <h2>{{ t('instance.cant_connect') }}</h2>
      <p>{{ t('home.offline_desc') }}</p>
    </div>

    <div v-else class="discover-content">
      <ModrinthSection :title="t('home.discover_shaders')" :items="shaders" :loading="isLoadingShaders">
        <ModrinthCard v-for="item in shaders" :key="item.slug" :item="item" />
      </ModrinthSection>

      <ModrinthSection class="mt-section" :title="t('home.discover_mods')" :items="mods" :loading="isLoadingMods">
        <ModrinthCard v-for="item in mods" :key="item.slug" :item="item" />
      </ModrinthSection>

      <ModrinthSection class="mt-section" :title="t('home.discover_modpacks')" :items="modpacks" :loading="isLoadingModpacks">
        <ModrinthCard v-for="item in modpacks" :key="item.slug" :item="item" />
      </ModrinthSection>
    </div>
  </div>
</template>

<style scoped>
.home-view {
  display: flex;
  flex-direction: column;
  padding: 0;
  padding-bottom: 32px;
}

.offline-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  margin-top: 64px;
  color: var(--text-muted);
}

.offline-icon {
  width: 64px;
  height: 64px;
  stroke: color-mix(in srgb, var(--color-white) 6%, transparent);
}

.offline-state h2 {
  color: var(--text-main);
  margin: 0;
}

.welcome-title {
  font-size: 1.8rem;
  font-weight: 600;
  color: var(--text-main);
  margin-top: 0;
  margin-bottom: 32px;
}

.discover-content {
  display: flex;
  flex-direction: column;
  gap: 0;
}

.mt-section {
  margin-top: 32px;
}
</style>
