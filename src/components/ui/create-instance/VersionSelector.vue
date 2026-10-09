<script setup lang="ts">
import { ref, watch } from 'vue';
import { t } from '../../../composables/useI18n';
import { useVersionFetch } from '../../../features/create-instance/composables/useVersionFetch';

const props = defineProps<{
  modelValue: string;
  loader: string;
}>();

const emit = defineEmits(['update:modelValue', 'update:url']);

const dropdownListRef = ref<HTMLElement | null>(null);

const {
  filteredVersions,
  isFetchingVersions,
  versionSearch,
  isDropdownOpen,
  dropdownDirection,
  fetchVersions,
  openDropdown,
  selectVersion,
  onSearchInput,
} = useVersionFetch(
  () => props.loader,
  (id, url) => {
    emit('update:modelValue', id);
    emit('update:url', url);
  },
  dropdownListRef
);

watch(() => props.loader, fetchVersions);

</script>


<template>
  <div class="custom-select-container">
    <div style="position: relative;">
      <input
        type="text"
        v-model="versionSearch"
        placeholder="Search version..."
        class="form-input version-input"
        :class="{ open: isDropdownOpen }"
        @focus="openDropdown"
        @click="openDropdown"
        @input="onSearchInput"
        :disabled="isFetchingVersions"
      />
      <svg class="trigger-icon" :class="{ open: isDropdownOpen }" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="6 9 12 15 18 9"></polyline>
      </svg>
    </div>

    <Transition name="dropdown-fade">
      <div
        v-if="isDropdownOpen"
        class="custom-select-dropdown"
        :class="dropdownDirection"
        ref="dropdownListRef"
      >
        <div class="versions-list">
          <div v-if="isFetchingVersions" class="loading-text">{{ t('create_instance.no_versions') }}</div>
          <div v-else-if="filteredVersions.length === 0" class="loading-text">{{ t('create_instance.no_versions_found') }}</div>
          <template v-else>
            <div
              v-for="v in filteredVersions"
              :key="v.id"
              class="version-card"
              :class="{ selected: versionSearch === v.label }"
              @click="selectVersion(v)"
            >
              <span class="version-name">{{ v.label }}</span>
            </div>
          </template>
        </div>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.form-input {
  width: 100%;
  height: 44px;
  background-color: color-mix(in srgb, var(--color-white) 3%, transparent);
  border: none;
  border-radius: 12px;
  padding: 0 16px;
  color: var(--text-main);
  font-family: inherit;
  font-size: 1rem;
  transition: background-color 0.2s ease, color 0.2s ease, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
  outline: none;
}

.form-input::placeholder {
  color: var(--text-muted);
}

.form-input:focus, .form-input.open {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
  box-shadow: 0 0 0 4px var(--accent);
}

.custom-select-container {
  position: relative;
}

.trigger-icon {
  position: absolute;
  right: 16px;
  top: 14px;
  pointer-events: none;
  color: var(--text-muted);
  transition: transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.trigger-icon.open {
  transform: rotate(180deg);
}

.custom-select-dropdown {
  position: absolute;
  left: 0;
  right: 0;
  background-color: var(--bg-shell);
  border: 1px solid var(--border-line);
  border-radius: 12px;
  z-index: 100;
  box-shadow: 0 10px 40px color-mix(in srgb, var(--color-black) 40%, transparent);
  display: flex;
  flex-direction: column;
}

.custom-select-dropdown.down {
  top: calc(100% + 8px);
}

.custom-select-dropdown.up {
  bottom: calc(100% + 8px);
}

.versions-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
  max-height: 240px;
  overflow-y: auto;
  padding: 8px;
}

.versions-list::-webkit-scrollbar { width: 6px; }
.versions-list::-webkit-scrollbar-track { background: transparent; margin: 8px 0; }
.versions-list::-webkit-scrollbar-thumb { background: var(--border-line); border-radius: 6px; }
.versions-list::-webkit-scrollbar-thumb:hover { background: var(--text-muted); }

.version-card {
  display: flex;
  align-items: center;
  padding: 10px 14px;
  background-color: transparent;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.version-card:hover {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
}

.version-card.selected {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
  color: var(--text-main);
}

.version-name {
  font-weight: 600;
  color: var(--text-main);
}

.loading-text {
  text-align: center;
  padding: 24px 16px;
  font-size: 1.05rem;
  color: var(--text-muted);
}

.dropdown-fade-enter-active,
.dropdown-fade-leave-active {
  transition: opacity 0.2s, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.dropdown-fade-enter-from,
.dropdown-fade-leave-to {
  opacity: 0;
}

.dropdown-fade-enter-from.down,
.dropdown-fade-leave-to.down {
  transform: translateY(-10px);
}

.dropdown-fade-enter-from.up,
.dropdown-fade-leave-to.up {
  transform: translateY(10px);
}
</style>
