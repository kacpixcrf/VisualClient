import { ref } from 'vue';

export const useSidebarTooltip = () => {
  const hoveredTooltip = ref('');
  const tooltipTop = ref(0);

  const handleMouseOver = (e: MouseEvent) => {
    const target = (e.target as HTMLElement).closest('[data-tooltip]');
    if (target) {
      hoveredTooltip.value = target.getAttribute('data-tooltip') || '';
      const rect = target.getBoundingClientRect();
      tooltipTop.value = rect.top + rect.height / 2;
    }
  };

  const handleMouseOut = (e: MouseEvent) => {
    const target = (e.target as HTMLElement).closest('[data-tooltip]');
    const related = e.relatedTarget as Node | null;
    if (target && related && target.contains(related)) return;
    hoveredTooltip.value = '';
  };

  return { hoveredTooltip, tooltipTop, handleMouseOver, handleMouseOut };
};
