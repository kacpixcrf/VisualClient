<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { t } from '../../composables/useI18n';
import IconChevronRight from '../icons/IconChevronRight.vue';
import AccountTypeStep from './add-account/AccountTypeStep.vue';
import OfflineStep from './add-account/OfflineStep.vue';
import MicrosoftStep from './add-account/MicrosoftStep.vue';

const emit = defineEmits(['close', 'create']);

const activeStep = ref<'choose' | 'offline' | 'microsoft'>('choose');
const loginError = ref('');

const handleClose = async () => {
  if (activeStep.value === 'microsoft') {
    await invoke('cancel_microsoft_login');
  }
  emit('close');
};

const goBackToChoose = async () => {
  if (activeStep.value === 'microsoft') {
    await invoke('cancel_microsoft_login');
  }
  activeStep.value = 'choose';
};

const handleOfflineCreate = (name: string) => {
  emit('create', { type: 'offline', name });
};

const handleMicrosoftLogin = async () => {
  activeStep.value = 'microsoft';
  loginError.value = '';
  try {
    const profile = await invoke('start_microsoft_login');
    emit('create', { type: 'microsoft', profile });
  } catch (e) {
    if (String(e) !== 'Login window closed') {
      loginError.value = String(e);
    } else {
      activeStep.value = 'choose';
    }
  }
};
</script>

<template>
  <div class="modal-backdrop" @click.self="handleClose">
    <div class="modal-container add-account-modal">
      <header class="modal-header">
        <div class="breadcrumb">
          <span class="breadcrumb-item static">{{ t('accounts.add_account') }}</span>
          <span class="breadcrumb-separator"><IconChevronRight /></span>
          <span
            class="breadcrumb-item"
            :class="{ active: activeStep === 'choose', clickable: activeStep !== 'choose' }"
            @click="activeStep !== 'choose' ? goBackToChoose() : null"
          >
            {{ t('accounts.choose_type') }}
          </span>
          <template v-if="activeStep === 'offline' || activeStep === 'microsoft'">
            <span class="breadcrumb-separator"><IconChevronRight /></span>
            <span class="breadcrumb-item active">{{ activeStep === 'offline' ? t('accounts.offline') : t('accounts.microsoft') }}</span>
          </template>
        </div>
        <div class="close-control" @click="handleClose">
          <svg width="16" height="16" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M3 3L9 9M9 3L3 9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
      </header>

      <div class="modal-content-wrapper">
        <div class="sliding-container" :class="'step-' + activeStep">
          <AccountTypeStep @selectOffline="activeStep = 'offline'" @selectMicrosoft="handleMicrosoftLogin" />
          <div class="slide-pane">
            <OfflineStep v-if="activeStep === 'offline'" @create="handleOfflineCreate" />
            <MicrosoftStep v-else-if="activeStep === 'microsoft'" :loginError="loginError" @retry="activeStep = 'choose'" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-backdrop {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: color-mix(in srgb, var(--color-black) 40%, transparent);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 999;
}

.add-account-modal {
  width: 640px;
  background-color: var(--bg-shell);
  border-radius: 16px;
  border: 1px solid var(--border-line);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  box-shadow: 0 10px 40px color-mix(in srgb, var(--color-black) 50%, transparent);
}

.modal-header {
  padding: 0 24px;
  min-height: 80px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid var(--border-line);
}

.breadcrumb {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 1.1rem;
}

.breadcrumb-item {
  color: var(--text-muted);
  transition: color 0.2s;
}

.breadcrumb-item.active {
  color: var(--color-white);
  font-weight: 600;
}

.breadcrumb-item.clickable {
  cursor: pointer;
}

.breadcrumb-item.clickable:hover {
  color: var(--text-main);
}

.breadcrumb-separator {
  display: flex;
  align-items: center;
  color: var(--text-muted);
}

.breadcrumb-separator svg {
  width: 18px;
  height: 18px;
}

.close-control {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
  cursor: pointer;
  transition: background-color 0.2s, color 0.2s, transform 0.2s;
}

.close-control:hover {
  background-color: var(--danger);
  color: var(--color-white);
}

.close-control:active {
  transform: scale(0.85);
}

.modal-content-wrapper {
  position: relative;
  overflow: hidden;
  height: 280px;
}

.sliding-container {
  display: flex;
  width: 200%;
  height: 100%;
  transition: transform 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.sliding-container.step-choose {
  transform: translateX(0);
}

.sliding-container.step-offline,
.sliding-container.step-microsoft {
  transform: translateX(-50%);
}

.slide-pane {
  width: 50%;
  height: 100%;
  display: flex;
  flex-direction: column;
}
</style>
