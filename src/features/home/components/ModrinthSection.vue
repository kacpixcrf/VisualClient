<script setup lang="ts">
import type { ModrinthProject } from '../composables/useModrinth';

defineProps<{
  title: string;
  items: ModrinthProject[];
  loading: boolean;
}>();
</script>

<template>
  <div class="discover-section">
    <h2 class="section-title">{{ title }}</h2>

    <div v-if="loading" class="cards-grid">
      <div class="skeleton-card" v-for="i in 5" :key="i"></div>
    </div>

    <div v-else class="cards-grid">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.discover-section {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.section-title {
  color: var(--text-muted);
  font-size: 1.1rem;
  font-weight: 600;
  margin: 0;
}

.skeleton-card {
  height: 280px;
  background-color: color-mix(in srgb, var(--color-white) 3%, transparent);
  border-radius: 12px;
  animation: pulse 1.5s infinite ease-in-out;
}

.skeleton-card:nth-child(n+4) {
  display: none;
}

@keyframes pulse {
  0% { opacity: 0.5; }
  50% { opacity: 1; }
  100% { opacity: 0.5; }
}

.cards-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
}

@media (min-width: 1280px) {
  .cards-grid {
    grid-template-columns: repeat(4, 1fr);
  }
}

@media (min-width: 1550px) {
  .cards-grid {
    grid-template-columns: repeat(5, 1fr);
  }
}
</style>
