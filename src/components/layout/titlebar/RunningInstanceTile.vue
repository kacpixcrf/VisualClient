<script setup lang="ts">
import { t } from '../../../composables/useI18n';
import { useRunningInstance } from '../../../features/titlebar/composables/useRunningInstance';
import IconStop from '../../icons/IconStop.vue';

const emit = defineEmits(['openInstance']);

const {
  instances,
  runningInstances,
  activeInstanceId,
  activeInstanceName,
  isDropdownOpen,
  toggleDropdown,
  selectInstance,
  handleStopClick,
  goToInstance,
} = useRunningInstance((id) => emit('openInstance', id));
</script>

<template>
  <div>
    <div v-if="runningInstances.length > 0" class="active-instance-tile" @click="toggleDropdown">
      <div class="active-circle"></div>
      <span
        class="active-name"
        :data-tooltip="t('instance.view_instance')"
        @click="goToInstance($event, activeInstanceId)"
      >{{ activeInstanceName }}</span>
      <IconStop class="stop-btn" @click="handleStopClick($event)" />
      <svg
        v-if="runningInstances.length > 1"
        class="dropdown-arrow"
        :class="{ 'is-open': isDropdownOpen }"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <polyline points="6 9 12 15 18 9"></polyline>
      </svg>

      <Transition name="dropdown">
        <div v-if="isDropdownOpen && runningInstances.length > 1" class="instance-dropdown" @click.stop>
          <div
            v-for="id in runningInstances"
            :key="id"
            class="dropdown-item"
            @click="selectInstance(id)"
          >
            <div class="item-name-wrapper">
              <span class="item-name">{{ instances.find(i => i.id === id)?.name || 'Minecraft' }}</span>
              <svg
                v-if="id === activeInstanceId"
                class="active-star"
                viewBox="0 0 24 24"
                fill="#ffd700"
                stroke="#ffd700"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"></polygon>
              </svg>
            </div>
            <IconStop class="stop-btn-small" @click="handleStopClick($event, id)" />
          </div>
        </div>
      </Transition>
    </div>

    <div v-else class="active-instance-tile empty-state">
      <div class="active-circle inactive"></div>
      <span class="active-name inactive-text">{{ t('instance.no_instances') }}</span>
    </div>
  </div>
</template>

<style scoped>
.active-instance-tile {
  position: relative;
  display: flex;
  align-items: center;
  background-color: color-mix(in srgb, var(--color-white) 3%, transparent);
  border-radius: 12px;
  padding: 4px 8px 4px 8px;
  gap: 8px;
  margin-right: 8px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.active-instance-tile:hover {
  background-color: color-mix(in srgb, var(--bg-shell) 85%, var(--color-white));
}

.active-circle {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background-color: var(--accent);
  box-shadow: 0 0 8px var(--accent);
}

.active-circle.inactive {
  background-color: var(--text-muted);
  box-shadow: none;
}

.active-name {
  font-size: 12px;
  color: var(--text-muted);
  font-weight: 600;
  white-space: nowrap;
  cursor: pointer;
}

.active-name:hover {
  text-decoration: underline;
  color: var(--text-main);
}

.active-name.inactive-text {
  cursor: default;
}

.active-name.inactive-text:hover {
  text-decoration: none;
  color: var(--text-muted);
}

.empty-state {
  cursor: default;
}

.empty-state:hover {
  background-color: color-mix(in srgb, var(--color-white) 3%, transparent);
}

.stop-btn {
  width: 16px;
  height: 16px;
  color: var(--danger);
  transition: filter 0.2s, transform 0.2s;
  cursor: pointer;
}

.stop-btn:hover {
  filter: brightness(1.2);
}

.stop-btn:active {
  transform: scale(0.85);
}

.dropdown-arrow {
  width: 14px;
  height: 14px;
  color: var(--text-muted);
  transition: transform 0.2s;
}

.dropdown-arrow.is-open {
  transform: rotate(180deg);
}

.instance-dropdown {
  position: absolute;
  top: calc(100% + 8px);
  right: 0;
  background-color: var(--bg-shell);
  border: 1px solid var(--border-line);
  border-radius: 12px;
  padding: 4px;
  min-width: 180px;
  box-shadow: 0 8px 24px color-mix(in srgb, var(--color-black) 50%, transparent);
  z-index: 1000;
  display: flex;
  flex-direction: column;
}

.dropdown-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: background-color 0.2s;
  gap: 16px;
}

.dropdown-item:hover {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
}

.item-name-wrapper {
  display: flex;
  align-items: center;
  gap: 8px;
}

.active-star {
  width: 14px;
  height: 14px;
}

.item-name {
  font-size: 13px;
  color: var(--text-main);
  font-weight: 500;
}

.stop-btn-small {
  width: 24px;
  height: 24px;
  padding: 4px;
  border-radius: 50%;
  color: var(--danger);
  cursor: pointer;
  transition: background-color 0.2s ease, color 0.2s ease, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.stop-btn-small:hover {
  background-color: color-mix(in srgb, var(--danger) 15%, transparent);
}

.stop-btn-small:active {
  transform: scale(0.85);
}

.dropdown-enter-active,
.dropdown-leave-active {
  transition: opacity 0.2s, transform 0.2s;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}
</style>
