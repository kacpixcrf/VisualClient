import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Server, EditServerForm } from '../types';
import { MCSRVSTAT_STATUS_URL, DEFAULT_ICON_PATH, DEFAULT_SERVER_NAME } from '../../../constants';

export function useServers(instanceId: () => string | undefined) {
  const servers = ref<Server[]>([]);

  const fetchMotd = async (index: number) => {
    const server = servers.value[index];
    if (!server) return;
    try {
      servers.value[index].loadingMotd = true;
      const res = await fetch(`${MCSRVSTAT_STATUS_URL}/${encodeURIComponent(server.ip)}`);
      const data = await res.json();

      if (servers.value[index]) {
        if (data.online) {
          servers.value[index].motdHtml = data.motd.html.join('<br>');
          servers.value[index].online = true;

          if (data.icon) {
            const cleanBase64 = data.icon.startsWith('data:image/') ? data.icon.split(',')[1] : data.icon;
            if (servers.value[index].icon_base64 !== cleanBase64) {
              servers.value[index].icon_base64 = cleanBase64;
              invoke('update_server_icon', {
                id: instanceId(),
                ipToMatch: server.ip,
                iconBase64: data.icon
              }).catch(() => {});
            }
          }
        } else {
          servers.value[index].motdHtml = '';
          servers.value[index].online = false;
        }
      }
    } catch {
      if (servers.value[index]) {
        servers.value[index].motdHtml = '';
        servers.value[index].online = false;
      }
    } finally {
      if (servers.value[index]) {
        servers.value[index].loadingMotd = false;
      }
    }
  };

  const loadData = async () => {
    const id = instanceId();
    if (!id) return;
    try {
      const fetchedServers: Server[] = await invoke('get_instance_servers', { id });

      const newServers = fetchedServers.map(s => {
        const existing = servers.value.find(ex => ex.ip === s.ip);
        if (existing) return existing;
        return {
          ...s,
          motdHtml: '',
          loadingMotd: true,
          online: false,
          icon_base64: s.icon_base64 || undefined
        };
      });

      const ipsToFetch = newServers.filter(s => s.loadingMotd).map(s => s.ip);
      servers.value = newServers;

      servers.value.forEach((server, index) => {
        if (ipsToFetch.includes(server.ip)) {
          fetchMotd(index);
        }
      });
    } catch {}
  };

  const refreshServers = () => {
    servers.value.forEach((_, idx) => fetchMotd(idx));
  };

  const saveServer = async (form: EditServerForm, isAdding: boolean): Promise<boolean> => {
    const id = instanceId();
    if (!id || !form.ip || form.ip.trim().length < 1) return false;
    try {
      const finalName = form.name || DEFAULT_SERVER_NAME;
      if (isAdding) {
        await invoke('add_instance_server', {
          id,
          name: finalName,
          ip: form.ip,
          acceptTextures: form.acceptTextures
        });
        servers.value.push({
          name: finalName,
          ip: form.ip,
          accept_textures: form.acceptTextures ?? undefined,
          loadingMotd: true,
          online: false,
          motdHtml: ''
        });
        fetchMotd(servers.value.length - 1);
      } else {
        await invoke('update_instance_server', {
          id,
          originalIp: form.originalIp,
          newName: finalName,
          newIp: form.ip,
          acceptTextures: form.acceptTextures
        });

        const idx = servers.value.findIndex(s => s.ip === form.originalIp);
        if (idx !== -1) {
          servers.value[idx].name = finalName;
          servers.value[idx].ip = form.ip;
          servers.value[idx].accept_textures = form.acceptTextures ?? undefined;
          servers.value[idx].loadingMotd = true;
          fetchMotd(idx);
        }
      }
      return true;
    } catch {
      return false;
    }
  };

  const removeServer = async (ip: string): Promise<boolean> => {
    const id = instanceId();
    if (!id) return false;
    try {
      await invoke('remove_instance_server', { id, ipToRemove: ip });
      servers.value = servers.value.filter(s => s.ip !== ip);
      return true;
    } catch {
      return false;
    }
  };

  const handleIconError = (e: Event) => {
    const target = e.target as HTMLImageElement;
    target.src = DEFAULT_ICON_PATH;
  };

  return { servers, loadData, fetchMotd, refreshServers, saveServer, removeServer, handleIconError };
}
