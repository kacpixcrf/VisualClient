<script setup lang="ts">
import { ref, computed, onMounted, watch, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useInstances, type Instance } from '../composables/useInstances';
import { useAccounts } from '../composables/useAccounts';
import { useServers } from '../features/instance/composables/useServers';
import { usePlay } from '../features/instance/composables/usePlay';
import InstanceHeader from '../features/instance/components/InstanceHeader.vue';
import InstanceTabs from '../features/instance/components/InstanceTabs.vue';
import ServerList from '../features/instance/components/ServerList.vue';
import WorldList from '../features/instance/components/WorldList.vue';
import ServerCard from '../features/instance/components/ServerCard.vue';
import WorldCard from '../features/instance/components/WorldCard.vue';
import LoginRequiredModal from '../components/modals/LoginRequiredModal.vue';
import EditServerModal from '../components/modals/EditServerModal.vue';
import DeleteServerModal from '../components/modals/DeleteServerModal.vue';
import type { Server, EditServerForm } from '../features/instance/types';
import {
  TAB_INDEX_SERVERS,
  TAB_INDEX_WORLDS,
  INSTANCE_POLL_INTERVAL_MS,
  QUICK_PLAY_MIN_MAJOR,
  QUICK_PLAY_MIN_MINOR
} from '../constants';

const props = defineProps<{ instanceId: string }>();
const emit = defineEmits(['openSettings', 'openAccounts']);

const { instances, runningInstances, startingInstances, killInstance, launchInstance } = useInstances();
const { accounts } = useAccounts();

const instance = computed(() => instances.value.find((i: Instance) => i.id === props.instanceId));
const isStarting = computed(() => instance.value ? startingInstances.value.includes(instance.value.id) : false);
const isRunning = computed(() => instance.value ? runningInstances.value.includes(instance.value.id) : false);

const activeTab = ref(TAB_INDEX_SERVERS);
const worlds = ref<{ folder_name: string; name: string; last_played: number; icon_base64?: string }[]>([]);

const { servers, loadData: loadServers, refreshServers, saveServer, removeServer } = useServers(
  () => instance.value?.id
);

const loadData = async () => {
  await loadServers();
  if (instance.value) {
    try {
      worlds.value = await invoke('get_instance_worlds', { id: instance.value.id });
    } catch {}
  }
};

let pollInterval: ReturnType<typeof setInterval> | null = null;

onMounted(() => {
  loadData();
  pollInterval = setInterval(loadData, INSTANCE_POLL_INTERVAL_MS);
});

onUnmounted(() => {
  if (pollInterval) clearInterval(pollInterval);
});

watch(() => props.instanceId, () => {
  servers.value = [];
  worlds.value = [];
  loadData();
});

const supportsWorldQuickPlay = computed(() => {
  if (!instance.value?.version) return false;
  const parts = instance.value.version.split('.');
  if (parts.length >= 2) {
    const major = parseInt(parts[0], 10);
    const minor = parseInt(parts[1], 10);
    return major > QUICK_PLAY_MIN_MAJOR || (major === QUICK_PLAY_MIN_MAJOR && minor >= QUICK_PLAY_MIN_MINOR);
  }
  return false;
});

const showLoginModal = ref(false);

const { handlePlay, handlePlayServer, handlePlayWorld } = usePlay({
  isRunning,
  isStarting,
  killInstance,
  launchInstance,
  activeAccount: () => accounts.value.find(a => a.active)?.username,
  showLoginModal
});

const handleAddAccountClick = () => {
  showLoginModal.value = false;
  emit('openAccounts');
};

const openFolder = async () => {
  if (instance.value) {
    try {
      await invoke('open_instance_folder', { id: instance.value.id });
    } catch {}
  }
};

const isEditModalOpen = ref(false);
const isAddingServer = ref(false);
const editServerForm = ref<EditServerForm>({ originalIp: '', name: '', ip: '', acceptTextures: null });

const openAddServer = () => {
  isAddingServer.value = true;
  editServerForm.value = { originalIp: '', name: '', ip: '', acceptTextures: null };
  isEditModalOpen.value = true;
};

const openEditServer = (server: Server) => {
  isAddingServer.value = false;
  editServerForm.value = {
    originalIp: server.ip,
    name: server.name,
    ip: server.ip,
    acceptTextures: server.accept_textures !== undefined ? server.accept_textures : null
  };
  isEditModalOpen.value = true;
};

const handleSaveServer = async () => {
  const success = await saveServer(editServerForm.value, isAddingServer.value);
  if (success) isEditModalOpen.value = false;
};

const isDeleteModalOpen = ref(false);
const serverToDelete = ref<string | null>(null);

const openRemoveServer = (ip: string) => {
  serverToDelete.value = ip;
  isDeleteModalOpen.value = true;
};

const confirmRemoveServer = async () => {
  if (!serverToDelete.value) return;
  const success = await removeServer(serverToDelete.value);
  if (success) {
    isDeleteModalOpen.value = false;
    serverToDelete.value = null;
  }
};

const handleCopyAddress = async (ip: string) => {
  try {
    await navigator.clipboard.writeText(ip);
  } catch {}
};
</script>

<template>
  <div v-if="instance" class="instance-view">
    <InstanceHeader
      :instance="instance"
      :is-running="isRunning"
      :is-starting="isStarting"
      @play="handlePlay(instance!.id)"
      @open-settings="emit('openSettings', $event)"
      @open-folder="openFolder"
    />

    <InstanceTabs
      v-model:active-tab="activeTab"
      @add-server="openAddServer"
      @refresh="refreshServers"
    />

    <div class="tab-content">
      <ServerList v-if="activeTab === TAB_INDEX_SERVERS" :hasServers="servers.length > 0">
        <ServerCard
          v-for="server in servers"
          :key="server.ip"
          :server="server"
          :is-running="isRunning"
          :is-starting="isStarting"
          @play="handlePlayServer(instance!.id, $event)"
          @edit="openEditServer"
          @remove="openRemoveServer"
          @copy-address="handleCopyAddress"
        />
      </ServerList>

      <WorldList v-if="activeTab === TAB_INDEX_WORLDS" :hasWorlds="worlds.length > 0">
        <WorldCard
          v-for="world in worlds"
          :key="world.folder_name"
          :world="world"
          :is-running="isRunning"
          :is-starting="isStarting"
          :supports-world-quick-play="supportsWorldQuickPlay"
          @play="handlePlayWorld(instance!.id, $event)"
        />
      </WorldList>
    </div>

    <EditServerModal
      :is-open="isEditModalOpen"
      :is-adding-server="isAddingServer"
      v-model:edit-server-form="editServerForm"
      @close="isEditModalOpen = false"
      @save="handleSaveServer"
    />

    <DeleteServerModal
      :is-open="isDeleteModalOpen"
      @close="isDeleteModalOpen = false; serverToDelete = null"
      @confirm="confirmRemoveServer"
    />

    <Transition name="modal">
      <LoginRequiredModal
        v-if="showLoginModal"
        @close="showLoginModal = false"
        @login="handleAddAccountClick"
      />
    </Transition>
  </div>
</template>

<style scoped>
.instance-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 0;
}

.tab-content {
  margin-top: 24px;
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
}
</style>
