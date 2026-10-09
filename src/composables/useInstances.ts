import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export interface Instance {
  id: string;
  name: string;
  loader: string;
  version: string;
  java_path: string;
}

const instances = ref<Instance[]>([]);
const isLoaded = ref(false);

const runningInstances = ref<string[]>([]);
const startingInstances = ref<string[]>([]);

let initializedEvents = false;

export function useInstances() {
  const fetchInstances = async () => {
    try {
      instances.value = await invoke<Instance[]>('get_instances');
      isLoaded.value = true;
    } catch (e) {
      console.error('Failed to fetch instances:', e);
    }
  };

  const renameInstance = async (id: string, newName: string) => {
    try {
      await invoke('rename_instance', { id, newName });
      const inst = instances.value.find(i => i.id === id);
      if (inst) {
        inst.name = newName;
      }
    } catch (e) {
      console.error('Failed to rename instance:', e);
    }
  };

  const deleteInstance = async (id: string) => {
    try {
      await invoke('delete_instance', { id });
      instances.value = instances.value.filter(i => i.id !== id);
    } catch (e) {
      console.error('Failed to delete instance:', e);
    }
  };

  if (!isLoaded.value) {
    fetchInstances();
  }

  if (!initializedEvents) {
    initializedEvents = true;
    invoke<string[]>('get_running_instances')
      .then((res: string[]) => {
        runningInstances.value = res;
      })
      .catch(console.error);
      
    listen<string>('instance_stopped', (event: any) => {
      runningInstances.value = runningInstances.value.filter(id => id !== event.payload);
      startingInstances.value = startingInstances.value.filter(id => id !== event.payload);
    });
  }

  const launchInstance = async (options: any) => {
    const { id } = options;
    if (runningInstances.value.includes(id) || startingInstances.value.includes(id)) {
      return; // Already running or starting
    }
    
    startingInstances.value.push(id);
    try {
      await invoke('launch_instance', options);
      // It spawned successfully
      startingInstances.value = startingInstances.value.filter(i => i !== id);
      if (!runningInstances.value.includes(id)) {
        runningInstances.value.push(id);
      }
    } catch (e) {
      startingInstances.value = startingInstances.value.filter(i => i !== id);
      throw e;
    }
  };

  const killInstance = async (id: string) => {
    try {
      await invoke('kill_instance', { id });
      // The rust side will emit instance_stopped, but we can do it optimistically too:
      runningInstances.value = runningInstances.value.filter(i => i !== id);
      startingInstances.value = startingInstances.value.filter(i => i !== id);
    } catch (e) {
      console.error('Failed to kill instance:', e);
    }
  };

  return {
    instances,
    isLoaded,
    runningInstances,
    startingInstances,
    fetchInstances,
    renameInstance,
    deleteInstance,
    launchInstance,
    killInstance
  };
}
