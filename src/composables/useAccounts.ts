import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export interface Account {
  username: string;
  active: boolean;
  type?: string;
}

const accounts = ref<Account[]>([]);
const isLoaded = ref(false);

export function useAccounts() {
  const fetchAccounts = async () => {
    try {
      accounts.value = await invoke<Account[]>('get_accounts');
      isLoaded.value = true;
    } catch (e) {
      console.error('Failed to fetch accounts', e);
    }
  };

  const addAccount = async (username: string, accountType: string = 'Offline') => {
    try {
      await invoke('add_account', { username, accountType });
      await fetchAccounts();
      await selectAccount(username);
    } catch (e) {
      console.error('Failed to add account', e);
    }
  };

  const addMicrosoftAccount = async (profile: any) => {
    try {
      await invoke('add_microsoft_account', {
        username: profile.name,
        uuid: profile.id,
        mctoken: profile.mc_token || profile.mcToken || "",
        refreshtoken: profile.refresh_token || profile.refreshToken || ""
      });
      await fetchAccounts();
      await selectAccount(profile.name);
    } catch (e) {
      console.error('Failed to add Microsoft account', e);
    }
  };

  const selectAccount = async (username: string) => {
    try {
      await invoke('select_account', { username });
      await fetchAccounts();
    } catch (e) {
      console.error('Failed to select account', e);
    }
  };

  const deleteAccount = async (username: string) => {
    try {
      await invoke('delete_account', { username });
      accounts.value = accounts.value.filter(a => a.username !== username);
    } catch (e) {
      console.error('Failed to delete account', e);
    }
  };

  if (!isLoaded.value) {
    fetchAccounts();
  }

  return { accounts, fetchAccounts, addAccount, addMicrosoftAccount, selectAccount, deleteAccount };
}
