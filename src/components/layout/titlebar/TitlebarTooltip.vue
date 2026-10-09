<script setup lang="ts">
defineProps<{
  text: string;
  leftPosition: number;
}>();
</script>

<template>
  <div class="titlebar-tooltip" :style="{ left: leftPosition + 'px' }">
    {{ text }}
  </div>
</template>

<style scoped>
.titlebar-tooltip {
  position: fixed;
  top: 56px;
  background-color: var(--color-black);
  color: var(--color-white);
  font-weight: 600;
  font-size: 13px;
  padding: 6px 12px;
  border-radius: 8px;
  pointer-events: none;
  z-index: 1000;
  box-shadow: 0 4px 6px color-mix(in srgb, var(--color-black) 30%, transparent);
  white-space: nowrap;
  transform: translateX(-50%);
}
</style>
