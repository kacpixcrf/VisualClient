import { ref } from 'vue';

export const useServerMenu = () => {
  const menuOpen = ref(false);

  const toggleMenu = () => {
    menuOpen.value = !menuOpen.value;
  };

  const closeMenu = () => {
    menuOpen.value = false;
  };

  return { menuOpen, toggleMenu, closeMenu };
};
