<script setup lang="ts">
import { currentTheme, setTheme, currentMainColor, setMainColor, t } from '../../../composables/useI18n';

const colors = [
  { id: 'green', value: '#1ad96a' },
  { id: 'blurple', value: '#5865F2' },
  { id: 'red', value: '#ed4245' },
  { id: 'gold', value: '#fee75c' },
  { id: 'cyan', value: '#00e5ff' },
  { id: 'pink', value: '#eb459e' }
];
</script>

<template>
  <div class="settings-page">
    <h2 class="section-title">{{ t('appearance.main_color_title') }}</h2>
    <p class="subtitle">{{ t('appearance.main_color_subtitle') }}</p>

    <div class="color-grid">
      <div 
        v-for="color in colors" 
        :key="color.id"
        class="color-circle"
        :style="{ backgroundColor: color.value }"
        :class="{ active: currentMainColor.toLowerCase() === color.value.toLowerCase() }"
        @click="setMainColor(color.value)"
      >
      </div>
    </div>

    <h2 class="section-title mt-8">{{ t('appearance.title') }}</h2>
    <p class="subtitle">{{ t('appearance.subtitle') }}</p>
    
    <div class="theme-grid">

      <div 
        class="theme-tile" 
        :class="{ active: currentTheme === 'dark' }"
        @click="setTheme('dark')"
      >
        <div class="theme-preview dark-preview">
          <div class="mock-card">
            <div class="mock-square"></div>
            <div class="mock-lines">
              <div class="mock-line wide"></div>
              <div class="mock-line narrow"></div>
            </div>
          </div>
        </div>
        <div class="theme-footer">
          <div class="footer-left">
            <div class="radio-circle">
              <div class="radio-inner" v-if="currentTheme === 'dark'"></div>
            </div>
            <span class="theme-name">{{ t('appearance.dark') }}</span>
          </div>
          <svg class="theme-icon" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
          </svg>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.section-title {
  font-weight: 600;
  color: var(--text-main);
  margin-bottom: 8px;
}

.mt-8 {
  margin-top: 32px;
}

.subtitle {
  color: var(--text-muted);
  font-size: 0.95rem;
  margin-top: -4px;
  margin-bottom: 24px;
}

.color-grid {
  display: flex;
  gap: 16px;
  margin-bottom: 24px;
}

.color-circle {
  width: 44px;
  height: 44px;
  border-radius: 50%;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1), box-shadow 0.2s;
}

.color-circle:hover {
  transform: scale(1.1);
}

.color-circle:active {
  transform: scale(0.9);
}

.color-circle.active {
  box-shadow: 0 0 0 3px var(--bg-content), 0 0 0 5px var(--accent);
}

.theme-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 20px;
}

.theme-tile {
  background-color: transparent;
  border-radius: 16px;
  border: none;
  cursor: pointer;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  transition: transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.theme-tile:active {
  transform: scale(0.85);
}

.theme-preview {
  height: 150px;
  padding: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.mock-card {
  width: 100%;
  padding: 16px;
  border-radius: 16px;
  display: flex;
  align-items: center;
  gap: 16px;
  box-shadow: 0 4px 12px color-mix(in srgb, var(--color-black) 10%, transparent);
}

.mock-square {
  width: 40px;
  height: 40px;
  border-radius: 10px;
}

.mock-lines {
  display: flex;
  flex-direction: column;
  gap: 8px;
  flex: 1;
}

.mock-line {
  height: 8px;
  border-radius: 4px;
}

.mock-line.wide {
  width: 80%;
}

.mock-line.narrow {
  width: 40%;
}

.dark-preview {
  background-color: var(--bg-content);
}
.dark-preview .mock-card {
  background-color: var(--bg-shell);
}
.dark-preview .mock-square {
  background-color: var(--border-line);
}
.dark-preview .mock-line.wide {
  background-color: var(--text-main);
}
.dark-preview .mock-line.narrow {
  background-color: var(--text-muted);
}



.theme-footer {
  padding: 12px 16px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  background-color: color-mix(in srgb, var(--color-white) 3%, transparent);
  transition: background-color 0.2s;
}

.theme-tile:hover .theme-footer {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
}

.theme-tile.active .theme-footer {
  background-color: color-mix(in srgb, var(--color-white) 9%, transparent);
}

.footer-left {
  display: flex;
  align-items: center;
  gap: 12px;
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
  background-color: var(--bg-content);
}

.theme-tile.active .radio-circle {
  border-color: var(--text-main);
}

.radio-inner {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background-color: var(--text-main);
}

.theme-name {
  font-weight: 600;
  color: var(--text-main);
  font-size: 0.95rem;
}

.theme-icon {
  color: var(--text-muted);
}
</style>
