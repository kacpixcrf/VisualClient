<script setup lang="ts">
import type { ModrinthProject } from '../composables/useModrinth';

defineProps<{ item: ModrinthProject }>();

const getImage = (item: ModrinthProject) =>
  item.gallery && item.gallery.length > 0 ? item.gallery[0] : item.icon_url;
</script>

<template>
  <div class="card">
    <div class="card-top">
      <img :src="getImage(item)" alt="" class="card-bg" />
    </div>
    <div class="card-bottom">
      <div class="card-header">
        <img :src="item.icon_url" alt="" class="card-icon" />
        <h3 class="card-title">{{ item.title }}</h3>
      </div>
      <p class="card-description">{{ item.description }}</p>
    </div>
  </div>
</template>

<style scoped>
.card {
  height: 280px;
  background-color: var(--bg-shell);
  border-radius: 12px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  transition: transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1), background-color 0.2s, box-shadow 0.2s;
  cursor: pointer;
  border: 1px solid var(--border-line);
}

.card:nth-child(n+4) {
  display: none;
}

.card:hover {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
  box-shadow: 0 8px 24px color-mix(in srgb, var(--color-black) 20%, transparent);
}

.card-top {
  height: 140px;
  width: 100%;
  overflow: hidden;
  background-color: color-mix(in srgb, var(--color-white) 3%, transparent);
}

.card-bg {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.3s ease;
}

.card:hover .card-bg {
  transform: scale(1.05);
}

.card-bottom {
  height: 140px;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 12px;
}

.card-icon {
  width: 36px;
  height: 36px;
  border-radius: 10px;
  object-fit: cover;
  background-color: color-mix(in srgb, var(--color-white) 3%, transparent);
}

.card-title {
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-main);
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.card-description {
  font-size: 0.9rem;
  font-weight: 500;
  color: var(--text-muted);
  line-height: 1.4;
  margin: 0;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

@media (min-width: 1280px) {
  .card:nth-child(4) {
    display: flex;
  }
}

@media (min-width: 1550px) {
  .card:nth-child(5) {
    display: flex;
  }
}
</style>
