import type { Ref } from 'vue';
import { t } from '../../../composables/useI18n';

interface PlayOptions {
  instanceId: string;
  serverIp?: string | null;
  worldFolder?: string | null;
}

interface PlayDependencies {
  isRunning: Ref<boolean>;
  isStarting: Ref<boolean>;
  killInstance: (id: string) => Promise<void>;
  launchInstance: (options: {
    id: string;
    username: string;
    launchingText: string;
    serverIp: string | null;
    worldFolder: string | null;
  }) => Promise<void>;
  activeAccount: () => string | undefined;
  showLoginModal: Ref<boolean>;
}

export function usePlay(deps: PlayDependencies) {
  const executePlay = async (options: PlayOptions) => {
    if (deps.isRunning.value || deps.isStarting.value) {
      await deps.killInstance(options.instanceId);
      return;
    }
    const username = deps.activeAccount();
    if (!username) {
      deps.showLoginModal.value = true;
      return;
    }
    try {
      await deps.launchInstance({
        id: options.instanceId,
        username,
        launchingText: t('instance.launching'),
        serverIp: options.serverIp ?? null,
        worldFolder: options.worldFolder ?? null
      });
    } catch {}
  };

  const handlePlay = (instanceId: string) => executePlay({ instanceId });

  const handlePlayServer = (instanceId: string, ip: string) =>
    executePlay({ instanceId, serverIp: ip });

  const handlePlayWorld = (instanceId: string, folderName: string) =>
    executePlay({ instanceId, worldFolder: folderName });

  return { handlePlay, handlePlayServer, handlePlayWorld };
}
