import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useInstances } from '../../../composables/useInstances';

const JAVA_DEFAULT_VERSION = 8;

export const useCreateInstance = (onClose: () => void) => {
  const { instances, fetchInstances } = useInstances();

  const name = ref('');
  const selectedLoader = ref('vanilla');
  const selectedVersion = ref('');
  const selectedVersionUrl = ref('');

  const resolveJavaVersion = async (url: string): Promise<number> => {
    if (!url) return JAVA_DEFAULT_VERSION;
    try {
      const res = await fetch(url);
      const json = await res.json();
      return json.javaVersion?.majorVersion ?? JAVA_DEFAULT_VERSION;
    } catch {
      return JAVA_DEFAULT_VERSION;
    }
  };

  const handleCreate = () => {
    const instanceName = name.value || `${selectedLoader.value.charAt(0).toUpperCase() + selectedLoader.value.slice(1)} ${selectedVersion.value}`.trim();
    const version = selectedVersion.value;
    const url = selectedVersionUrl.value;
    const loader = selectedLoader.value;

    onClose();

    (async () => {
      const javaVersion = await resolveJavaVersion(url);
      try {
        await invoke('create_instance', {
          name: instanceName,
          loader,
          version,
          javaVersion,
          folderName: instanceName,
        });
        await fetchInstances();
      } catch {
        // silently fail
      }
    })();
  };

  return {
    instances,
    name,
    selectedLoader,
    selectedVersion,
    selectedVersionUrl,
    handleCreate,
  };
};
