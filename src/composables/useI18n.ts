import { ref } from 'vue';
import en from '../locales/en.json';
import pl from '../locales/pl.json';
import { invoke } from '@tauri-apps/api/core';

const translations: Record<string, any> = { en, pl };

export const currentLanguage = ref('en');
export const currentTheme = ref('dark');
export const currentMainColor = ref('#1ad96a');
export const isLanguageLoaded = ref(false);

export async function initI18n() {
  try {
    const settings: any = await invoke('get_settings');
    if (settings) {
      if (settings.language && translations[settings.language]) {
        currentLanguage.value = settings.language;
      }
      if (settings.theme) {
        currentTheme.value = settings.theme;
        applyThemeClass(settings.theme);
      }
      if (settings.main_color) {
        currentMainColor.value = settings.main_color;
        applyMainColor(settings.main_color);
      }
    }
  } catch (e) {
  } finally {
    isLanguageLoaded.value = true;
  }
}

export async function setLanguage(lang: string) {
  if (translations[lang]) {
    currentLanguage.value = lang;
    await saveSettings();
  }
}

export async function setTheme(theme: string) {
  currentTheme.value = theme;
  applyThemeClass(theme);
  await saveSettings();
}

export async function setMainColor(color: string) {
  currentMainColor.value = color;
  applyMainColor(color);
  await saveSettings();
}

async function saveSettings() {
  try {
    await invoke('save_settings', { 
      settings: { 
        language: currentLanguage.value,
        theme: currentTheme.value,
        main_color: currentMainColor.value
      } 
    });
  } catch (e) {}
}

function applyThemeClass(theme: string) {
  document.documentElement.className = '';
  if (theme !== 'dark') {
    document.documentElement.classList.add(`${theme}-theme`);
  }
}

function applyMainColor(color: string) {
  document.documentElement.style.setProperty('--accent', color);
}

export function t(key: string, params?: Record<string, string | number>): string {
  const keys = key.split('.');
  let value = translations[currentLanguage.value];
  for (const k of keys) {
    if (value === undefined) break;
    value = value[k];
  }
  if (typeof value === 'string' && params) {
    for (const [k, v] of Object.entries(params)) {
      value = value.replace(`{${k}}`, String(v));
    }
  }
  return value || key;
}
