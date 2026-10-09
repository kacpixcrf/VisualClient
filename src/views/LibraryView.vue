<script setup lang="ts">
import { ref } from 'vue';
import { t } from '../composables/useI18n';
import { useInstances, type Instance } from '../composables/useInstances';
import { useAccounts } from '../composables/useAccounts';
import LibraryEmpty from '../features/library/components/LibraryEmpty.vue';
import InstanceGrid from '../features/library/components/InstanceGrid.vue';
import InstanceCard from '../features/library/components/InstanceCard.vue';
import DeleteInstanceModal from '../components/modals/DeleteInstanceModal.vue';
import LoginRequiredModal from '../components/modals/LoginRequiredModal.vue';

const emit = defineEmits(['createInstance', 'openInstance', 'openAccounts']);
const { instances, deleteInstance, runningInstances, startingInstances, killInstance, launchInstance } = useInstances();
const { accounts } = useAccounts();

const instanceToDelete = ref<{ id: string; name: string } | null>(null);
const showLoginModal = ref(false);

const confirmDelete = async () => {
  if (instanceToDelete.value) {
    await deleteInstance(instanceToDelete.value.id);
    instanceToDelete.value = null;
  }
};

const handleQuickPlay = async (instance: Instance) => {
  if ([...runningInstances.value, ...startingInstances.value].includes(instance.id)) {
    await killInstance(instance.id);
    return;
  }
  const activeUsername = accounts.value.find(a => a.active)?.username;
  if (!activeUsername) {
    showLoginModal.value = true;
  } else {
    try {
      await launchInstance({ id: instance.id, username: activeUsername, launchingText: t('instance.launching') });
    } catch (e) {
      console.error(e);
    }
  }
};

const handleAddAccountClick = () => {
  showLoginModal.value = false;
  emit('openAccounts');
};
</script>

<template>
  <div class="library-view">
    <LibraryEmpty v-if="instances.length === 0" @create="emit('createInstance')" />
    <InstanceGrid v-else>
      <InstanceCard
        v-for="instance in instances"
        :key="instance.id"
        :instance="instance"
        :isRunning="[...runningInstances, ...startingInstances].includes(instance.id)"
        @open="emit('openInstance', $event)"
        @quickPlay="handleQuickPlay"
        @delete="instanceToDelete = { id: $event.id, name: $event.name }"
      />
    </InstanceGrid>

    <Transition name="modal">
      <DeleteInstanceModal
        v-if="instanceToDelete"
        :name="instanceToDelete.name"
        @close="instanceToDelete = null"
        @confirm="confirmDelete"
      />
    </Transition>

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
.library-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 32px;
  overflow-y: auto;
}
</style>
