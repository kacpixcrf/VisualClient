import { ref, onMounted, onUnmounted } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';

export function useWindowControls() {
  const appWindow = getCurrentWindow();
  const isMaximized = ref(false);

  const minimize = () => appWindow.minimize();
  const toggleMaximize = () => appWindow.toggleMaximize();
  const close = () => appWindow.close();
  const startDrag = (e: MouseEvent) => {
    if (e.target !== e.currentTarget) return;
    appWindow.startDragging();
  };

  let unlisten: (() => void) | undefined;

  onMounted(async () => {
    isMaximized.value = await appWindow.isMaximized();
    unlisten = await appWindow.onResized(async () => {
      isMaximized.value = await appWindow.isMaximized();
    });
  });

  onUnmounted(() => {
    unlisten?.();
  });

  return { isMaximized, minimize, toggleMaximize, close, startDrag };
}
