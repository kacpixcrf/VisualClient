<script setup lang="ts">
</script>

<template>
  <div class="instances-grid">
    <slot />
  </div>
</template>

<style scoped>
.instances-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
  align-content: start;
}
</style>
