import { ref, computed, watch, onMounted, onUnmounted } from 'vue';
import { useInstances } from '../../../composables/useInstances';

export const useRunningInstance = (onOpenInstance: (id: string) => void) => {
  const { instances, runningInstances, killInstance } = useInstances();

  const selectedInstanceId = ref<string | null>(null);
  const isDropdownOpen = ref(false);

  const activeInstanceId = computed(() => {
    if (runningInstances.value.length === 0) return null;
    if (selectedInstanceId.value && runningInstances.value.includes(selectedInstanceId.value)) {
      return selectedInstanceId.value;
    }
    return runningInstances.value[0];
  });

  const activeInstanceName = computed(() => {
    if (!activeInstanceId.value) return '';
    const inst = instances.value.find(i => i.id === activeInstanceId.value);
    return inst ? inst.name : 'Minecraft';
  });

  const toggleDropdown = () => {
    if (runningInstances.value.length > 1) {
      isDropdownOpen.value = !isDropdownOpen.value;
    }
  };

  watch(() => runningInstances.value.length, (newLen) => {
    if (newLen < 2) isDropdownOpen.value = false;
  });

  const closeDropdownOnClick = (e: MouseEvent) => {
    if (!(e.target as HTMLElement).closest('.active-instance-tile')) {
      isDropdownOpen.value = false;
    }
  };

  const selectInstance = (id: string) => {
    selectedInstanceId.value = id;
    isDropdownOpen.value = false;
  };

  const handleStopClick = async (e: Event, id?: string) => {
    e.stopPropagation();
    const targetId = id || activeInstanceId.value;
    if (targetId) await killInstance(targetId);
  };

  const goToInstance = (e: Event, id: string | null) => {
    e.stopPropagation();
    if (id) onOpenInstance(id);
  };

  onMounted(() => document.addEventListener('click', closeDropdownOnClick));
  onUnmounted(() => document.removeEventListener('click', closeDropdownOnClick));

  return {
    instances,
    runningInstances,
    activeInstanceId,
    activeInstanceName,
    isDropdownOpen,
    toggleDropdown,
    selectInstance,
    handleStopClick,
    goToInstance,
  };
};
