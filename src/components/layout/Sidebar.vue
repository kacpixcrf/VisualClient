<script setup lang="ts">
import IconHome from '../icons/IconHome.vue';
import IconLibrary from '../icons/IconLibrary.vue';
import IconInstance from '../icons/IconInstance.vue';
import IconUser from '../icons/IconUser.vue';
import IconSettings from '../icons/IconSettings.vue';
import IconPlus from '../icons/IconPlus.vue';
import { t } from '../../composables/useI18n';
import { useInstances } from '../../composables/useInstances';
import { useSidebarTooltip } from '../../features/sidebar/composables/useSidebarTooltip';

const { instances } = useInstances();

defineProps<{
  activeCategory: string
}>();

const emit = defineEmits(['changeCategory', 'openSettings', 'createInstance']);

const { hoveredTooltip, tooltipTop, handleMouseOver, handleMouseOut } = useSidebarTooltip();
</script>


<template>
  <aside class="sidebar" @mouseover="handleMouseOver" @mouseout="handleMouseOut">
    <nav class="nav-menu">
      <div class="nav-item" :class="{ active: activeCategory === 'home' }" :data-tooltip="t('sidebar.home')" @click="emit('changeCategory', 'home')">
        <IconHome />
      </div>
      
      <div 
        class="nav-item" 
        :class="{ 
          active: activeCategory === 'library',
          'pseudo-hover': activeCategory.startsWith('instance_')
        }" 
        :data-tooltip="t('sidebar.library')" 
        @click="emit('changeCategory', 'library')"
      >
        <IconLibrary />
      </div>

      <div class="nav-item" :class="{ active: activeCategory === 'accounts' }" :data-tooltip="t('accounts.title')" @click="emit('changeCategory', 'accounts')">
        <IconUser />
      </div>
      
      <div class="nav-separator"></div>

      <div v-if="instances.length > 0" class="instances-scroll-list">
        <div 
          v-for="instance in instances" 
          :key="instance.id" 
          class="nav-item instance-nav-item" 
          :class="{ active: activeCategory === 'instance_' + instance.id }" 
          :data-tooltip="instance.name" 
          @click="emit('changeCategory', 'instance_' + instance.id)"
        >
          <div class="instance-avatar">
            <IconInstance />
          </div>
        </div>
      </div>

      <div class="nav-separator" v-if="instances.length > 0"></div>

      <div class="nav-item" :data-tooltip="t('sidebar.create_instance')" @click="emit('createInstance')">
        <IconPlus />
      </div>
    </nav>

    <div class="settings-menu">
      <div class="nav-item settings-btn" :data-tooltip="t('sidebar.settings')" @click="emit('openSettings')">
        <IconSettings />
      </div>
    </div>

    <Transition name="tooltip-fade">
      <div v-if="hoveredTooltip" class="global-tooltip" :style="{ top: tooltipTop + 'px' }">
        {{ hoveredTooltip }}
      </div>
    </Transition>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 60px;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: center;
  padding: 0 0 12px 0;
  background-color: transparent;
  height: 100%;
}

.nav-menu {
  display: flex;
  flex-direction: column;
  gap: 16px;
  width: 100%;
  align-items: center;
  flex: 1;
}

.instances-scroll-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
  max-height: 164px;
  overflow-y: auto;
  overflow-x: hidden;
  width: 100%;
  align-items: center;
}

.instances-scroll-list::-webkit-scrollbar {
  display: none;
}

.settings-menu {
  margin-top: auto;
  flex-shrink: 0;
  padding-top: 12px;
}

.nav-item {
  width: 44px;
  height: 44px;
  flex-shrink: 0;
  display: flex;
  justify-content: center;
  align-items: center;
  border-radius: 50%;
  color: var(--text-muted);
  cursor: pointer;
  transition: background-color 0.2s ease, color 0.2s ease, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
  position: relative;
}

.nav-item * {
  pointer-events: none;
}

.nav-item:hover {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
  color: var(--text-main);
  border-radius: 50%;
}

.nav-item.active {
  background-color: color-mix(in srgb, var(--accent) 15%, transparent);
  color: var(--accent);
  border-radius: 50%;
}



.nav-item:active {
  transform: scale(0.85);
}

.nav-separator {
  width: 32px;
  height: 2px;
  background-color: var(--border-line);
  border-radius: 2px;
  margin: 4px 0;
}

.nav-item.pseudo-hover {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
  color: var(--text-main);
  border-radius: 50%;
}

.instance-nav-item {
  border-radius: 50%;
}

.instance-nav-item:hover, .instance-nav-item.active {
  border-radius: 50%;
}

.instance-avatar {
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  pointer-events: none;
}

.instance-avatar svg {
  width: 26px;
  height: 26px;
}

.global-tooltip {
  position: fixed;
  left: 72px;
  background-color: var(--color-black);
  color: var(--color-white);
  font-weight: 600;
  font-size: 13px;
  padding: 6px 12px;
  border-radius: 8px;
  pointer-events: none;
  z-index: 1000;
  box-shadow: 0 4px 6px color-mix(in srgb, var(--color-black) 30%, transparent);
  white-space: nowrap;
  transform: translateY(-50%);
}

.tooltip-fade-enter-active, .tooltip-fade-leave-active {
  transition: opacity 0.2s, transform 0.2s;
}
.tooltip-fade-enter-from, .tooltip-fade-leave-to {
  opacity: 0;
  transform: translate(-4px, -50%);
}
.tooltip-fade-enter-to, .tooltip-fade-leave-from {
  opacity: 1;
  transform: translate(0, -50%);
}
</style>
