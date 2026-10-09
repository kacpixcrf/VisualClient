<script setup lang="ts">
const props = defineProps<{
  modelValue: string;
}>();

const emit = defineEmits(['update:modelValue']);

const selectLoader = (loader: string) => {
  emit('update:modelValue', props.modelValue === loader ? '' : loader);
};
</script>

<template>
  <div class="loaders-container">
    <div 
      class="loader-card" 
      :class="{ selected: modelValue === 'vanilla' }" 
      @click="selectLoader('vanilla')"
    >
      <svg v-if="modelValue === 'vanilla'" class="check-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="20 6 9 17 4 12"></polyline>
      </svg>
      <span>Vanilla</span>
    </div>
    <div 
      class="loader-card" 
      :class="{ selected: modelValue === 'fabric' }" 
      @click="selectLoader('fabric')"
    >
      <svg v-if="modelValue === 'fabric'" class="check-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="20 6 9 17 4 12"></polyline>
      </svg>
      <span>Fabric</span>
    </div>
    <div 
      class="loader-card" 
      :class="{ selected: modelValue === 'forge' }" 
      @click="selectLoader('forge')"
    >
      <svg v-if="modelValue === 'forge'" class="check-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="20 6 9 17 4 12"></polyline>
      </svg>
      <span>Forge</span>
    </div>
  </div>
</template>

<style scoped>
.loaders-container {
  display: flex;
  gap: 12px;
}

.loader-card {
  height: 36px;
  padding: 0 16px;
  background-color: color-mix(in srgb, var(--color-white) 3%, transparent);
  border: 4px solid var(--border-line);
  border-radius: 12px;
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-weight: 600;
  color: var(--text-muted);
  transition: background-color 0.2s ease, color 0.2s ease, transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.loader-card:active {
  transform: scale(0.85);
}

.loader-card:hover {
  background-color: color-mix(in srgb, var(--color-white) 6%, transparent);
}

.loader-card.selected {
  border-color: var(--accent);
  color: var(--text-main);
}

.check-icon {
  flex-shrink: 0;
  color: var(--accent);
}
</style>
