<script setup lang="ts">
import { ref } from 'vue';
import { t } from '../composables/useI18n';
import { useAccounts } from '../composables/useAccounts';
import { emit } from '@tauri-apps/api/event';
import AddAccountModal from '../components/modals/AddAccountModal.vue';
import DeleteAccountModal from '../components/modals/DeleteAccountModal.vue';
import AccountsHeader from '../features/accounts/components/AccountsHeader.vue';
import AccountCard from '../features/accounts/components/AccountCard.vue';

const { accounts, addAccount, addMicrosoftAccount, selectAccount, deleteAccount } = useAccounts();

const showAddAccountModal = ref(false);
const accountToDelete = ref<string | null>(null);

const confirmDelete = async () => {
  if (accountToDelete.value) {
    await deleteAccount(accountToDelete.value);
    accountToDelete.value = null;
  }
};

const handleCreateAccount = async (payload: { type: string; name?: string; profile?: { name: string } }) => {
  if (payload.type === 'offline' && payload.name && !accounts.value.some(a => a.username === payload.name)) {
    await addAccount(payload.name, 'Offline');
    showAddAccountModal.value = false;
    emit('show_toast', { message: t('accounts.added_offline', { name: payload.name }) });
  } else if (payload.type === 'microsoft' && payload.profile?.name) {
    await addMicrosoftAccount(payload.profile);
    showAddAccountModal.value = false;
    emit('show_toast', { message: t('accounts.added_microsoft', { name: payload.profile.name }) });
  }
};
</script>

<template>
  <div class="accounts-view">
    <AccountsHeader :title="t('accounts.title')" @add-account="showAddAccountModal = true" />

    <div class="accounts-list">
      <AccountCard
        v-for="acc in accounts"
        :key="acc.username"
        :account="acc"
        @select="selectAccount"
        @delete="accountToDelete = $event"
      />
      <div v-if="accounts.length === 0" class="empty-accounts">
        {{ t('accounts.empty') }}
      </div>
    </div>

    <Transition name="modal">
      <AddAccountModal
        v-if="showAddAccountModal"
        @close="showAddAccountModal = false"
        @create="handleCreateAccount"
      />
    </Transition>

    <Transition name="modal">
      <DeleteAccountModal
        v-if="accountToDelete"
        :name="accountToDelete"
        @close="accountToDelete = null"
        @confirm="confirmDelete"
      />
    </Transition>
  </div>
</template>

<style scoped>
.accounts-view {
  padding: 40px;
  display: flex;
  flex-direction: column;
  gap: 32px;
}

.accounts-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
  align-content: start;
}

.empty-accounts {
  grid-column: 1 / -1;
  text-align: center;
  padding: 40px;
  color: var(--text-muted);
  font-style: italic;
}
</style>
