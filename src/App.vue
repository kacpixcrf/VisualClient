<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import TitleBar from './components/layout/TitleBar.vue';
import Sidebar from './components/layout/Sidebar.vue';
import SettingsModal from './components/modals/SettingsModal.vue';
import CreateInstanceModal from './components/modals/CreateInstanceModal.vue';
import HomeView from './views/HomeView.vue';
import LibraryView from './views/LibraryView.vue';
import InstanceView from './views/InstanceView.vue';
import AccountsView from './views/AccountsView.vue';
import SplashScreen from './components/ui/SplashScreen.vue';
import NotificationContainer from './components/ui/NotificationContainer.vue';
import InstanceSettingsModal from './components/modals/InstanceSettingsModal.vue';
import AppearanceSettings from './components/ui/settings/AppearanceSettings.vue';
import LanguageSettings from './components/ui/settings/LanguageSettings.vue';
import GeneralSettings from './components/ui/instance-settings/GeneralSettings.vue';
import JavaSettings from './components/ui/instance-settings/JavaSettings.vue';
import DeleteInstanceModal from './components/modals/DeleteInstanceModal.vue';
import { initI18n, isLanguageLoaded } from './composables/useI18n';
import { useInstances } from './composables/useInstances';
import { checkForUpdates } from './composables/useUpdater';

const activeCategory = ref('home');
const isSettingsOpen = ref(false);
const isCreateInstanceOpen = ref(false);
const selectedInstanceForSettings = ref<any>(null);

const { fetchInstances } = useInstances();

const isCategoryLoading = ref(false);
let loaderTimeout: number | undefined;

watch(activeCategory, () => {
  isCategoryLoading.value = false;
  clearTimeout(loaderTimeout);
  setTimeout(() => {
    isCategoryLoading.value = true;
    loaderTimeout = window.setTimeout(() => {
      isCategoryLoading.value = false;
    }, 450);
  }, 20);
});

const handleCreateInstance = () => {
  isCreateInstanceOpen.value = true;
};

const handleOpenInstanceSettings = (instance: any) => {
  selectedInstanceForSettings.value = instance;
};

const handleInstanceDeleted = () => {
  selectedInstanceForSettings.value = null;
  activeCategory.value = 'library';
};

onMounted(() => {
  initI18n();
  fetchInstances();
  checkForUpdates(false);
});
</script>

<template>
  <SplashScreen />
  <div class="window-frame" v-if="isLanguageLoaded">
    <TitleBar @openInstance="(id) => activeCategory = 'instance_' + id" />
    <div class="window-body">
      <Sidebar 
        :activeCategory="activeCategory" 
        @changeCategory="(cat) => activeCategory = cat"
        @openSettings="isSettingsOpen = true"
        @createInstance="handleCreateInstance"
      />
      <main class="main-content" style="position: relative;">
        <div class="category-loader" :class="{ loading: isCategoryLoading }"></div>
        <HomeView v-if="activeCategory === 'home'" />
        <LibraryView v-if="activeCategory === 'library'" @createInstance="handleCreateInstance" @openInstance="(name) => activeCategory = 'instance_' + name" @openAccounts="activeCategory = 'accounts'" />
        <AccountsView v-if="activeCategory === 'accounts'" />
        <InstanceView 
          v-if="activeCategory.startsWith('instance_')" 
          :instanceId="activeCategory.replace('instance_', '')"
          @openSettings="handleOpenInstanceSettings"
          @openAccounts="activeCategory = 'accounts'"
        />
      </main>
    </div>
    
    <Transition name="modal">
      <SettingsModal v-if="isSettingsOpen" @close="isSettingsOpen = false">
        <template #default="{ activeTab }">
          <AppearanceSettings v-if="activeTab === 'appearance'" />
          <LanguageSettings v-if="activeTab === 'language'" />
        </template>
      </SettingsModal>
    </Transition>

    <Transition name="modal">
      <CreateInstanceModal v-if="isCreateInstanceOpen" @close="isCreateInstanceOpen = false" @created="fetchInstances" />
    </Transition>

    <Transition name="modal">
      <InstanceSettingsModal
        v-if="selectedInstanceForSettings"
        :instance="selectedInstanceForSettings"
        @close="selectedInstanceForSettings = null"
        @deleted="handleInstanceDeleted"
      >
        <template #default="{ activeTab, instance }">
          <GeneralSettings v-if="activeTab === 'general'" :instance="instance" @requestDelete="selectedInstanceForSettings._deleteRequested = true" />
          <JavaSettings v-if="activeTab === 'java'" :instance="instance" />
        </template>
        <template #deleteModal="{ showDeleteConfirm, onConfirm, onClose }">
          <DeleteInstanceModal
            v-if="showDeleteConfirm"
            :name="selectedInstanceForSettings.name"
            @close="onClose"
            @confirm="onConfirm"
          />
        </template>
      </InstanceSettingsModal>
    </Transition>
    
    <NotificationContainer />
  </div>
</template>

<style scoped>
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.2s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-active :deep(.modal-container),
.modal-leave-active :deep(.modal-container) {
  transition: transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.modal-enter-from :deep(.modal-container),
.modal-leave-to :deep(.modal-container) {
  transform: scale(0.85);
}

.category-loader {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 3px;
  background-color: var(--accent);
  transform: translateY(-100%);
  opacity: 0;
  transition: transform 0.3s ease-in, opacity 0.3s ease;
  z-index: 100;
  box-shadow: 0 0 12px color-mix(in srgb, var(--accent) 80%, transparent);
}

.category-loader.loading {
  transform: translateY(0);
  opacity: 1;
  transition: transform 0.4s cubic-bezier(0.25, 1, 0.5, 1), opacity 0.2s ease;
}
</style>
