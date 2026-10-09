<script setup lang="ts">
import { currentLanguage, setLanguage, t } from '../../../composables/useI18n';
</script>

<template>
  <div class="settings-page">
    <h2>{{ t('settings.language') }}</h2>
    
    <div class="language-list">
      

      <div 
        class="language-tile" 
        :class="{ active: currentLanguage === 'pl' }"
        @click="setLanguage('pl')"
      >
        <div class="tile-left">
          <div class="radio-circle">
            <div class="radio-inner" v-if="currentLanguage === 'pl'"></div>
          </div>
          <span class="native-name">{{ t('language.pl_native') }}</span>
        </div>
        <div class="tile-right">
          <span class="translated-name">{{ t('language.pl_translated') }}</span>
        </div>
      </div>


      <div 
        class="language-tile" 
        :class="{ active: currentLanguage === 'en' }"
        @click="setLanguage('en')"
      >
        <div class="tile-left">
          <div class="radio-circle">
            <div class="radio-inner" v-if="currentLanguage === 'en'"></div>
          </div>
          <span class="native-name">{{ t('language.en_native') }}</span>
        </div>
        <div class="tile-right">
          <span class="translated-name">{{ t('language.en_translated') }}</span>
        </div>
      </div>

    </div>
  </div>
</template>

<style scoped>
.language-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-top: 24px;
}

.language-tile {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background-color: color-mix(in srgb, var(--color-white) 3%, transparent);
  padding: 16px 20px;
  border-radius: 12px;
  border: none;
  cursor: pointer;
  transition: background-color 0.2s ease, color 0.2s ease, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.language-tile:hover {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
}

.language-tile:active {
  transform: scale(0.85);
}

.tile-left {
  display: flex;
  align-items: center;
  gap: 16px;
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

.language-tile.active .radio-circle {
  border-color: var(--text-main);
}

.radio-inner {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background-color: var(--text-main);
}

.native-name {
  font-size: 1.1rem;
  font-weight: 500;
  color: var(--text-main);
}

.tile-right {
  color: var(--text-muted);
  font-size: 0.95rem;
}
</style>
