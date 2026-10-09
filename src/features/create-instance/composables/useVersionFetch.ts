import { ref, computed, onMounted, onUnmounted, nextTick, type Ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const MOJANG_MANIFEST_URL = 'https://launchermeta.mojang.com/mc/game/version_manifest.json';
const VERSION_TYPE_RELEASE = 'release';

interface VersionEntry {
  id: string;
  type: string;
  url: string;
}

interface VersionManifest {
  versions: VersionEntry[];
}

interface ForgePromos {
  promos: Record<string, string>;
}

export const useVersionFetch = (
  loader: () => string,
  onSelect: (id: string, url: string) => void,
  dropdownListRef: Ref<HTMLElement | null>
) => {

  const versions = ref<{ id: string; label: string }[]>([]);
  const versionUrls = ref<Record<string, string>>({});
  const isFetchingVersions = ref(true);
  const versionSearch = ref('');
  const isUserTyping = ref(false);
  const isDropdownOpen = ref(false);
  const dropdownDirection = ref<'up' | 'down'>('down');


  const filteredVersions = computed(() => {
    if (!isUserTyping.value) return versions.value;
    const q = versionSearch.value.toLowerCase();
    return versions.value.filter(v => v.label.toLowerCase().includes(q) || v.id.toLowerCase().includes(q));
  });

  const setFirst = () => {
    if (versions.value.length > 0) {
      versionSearch.value = versions.value[0].label;
      onSelect(versions.value[0].id, versionUrls.value[versions.value[0].id] || '');
    }
  };

  const loadRelease = async (): Promise<VersionEntry[]> => {
    const res = await fetch(MOJANG_MANIFEST_URL);
    const data: VersionManifest = await res.json();
    return data.versions.filter(v => v.type === VERSION_TYPE_RELEASE);
  };

  const fetchVersions = async () => {
    isFetchingVersions.value = true;
    versions.value = [];
    try {
      if (loader() === 'forge') {
        const responseText = await invoke<string>('fetch_forge_versions');
        const data: ForgePromos = JSON.parse(responseText);
        const forgeMcVersions = new Set<string>();
        if (data.promos) {
          for (const key of Object.keys(data.promos)) {
            forgeMcVersions.add(key.replace('-latest', '').replace('-recommended', ''));
          }
        }
        const rel = await loadRelease();
        const filtered = rel.filter(v => forgeMcVersions.has(v.id));
        versions.value = filtered.map(v => ({ id: v.id, label: v.id }));
        filtered.forEach(v => { versionUrls.value[v.id] = v.url; });
      } else {
        const rel = await loadRelease();
        versions.value = rel.map(v => ({ id: v.id, label: v.id }));
        rel.forEach(v => { versionUrls.value[v.id] = v.url; });
      }
      setFirst();
    } catch {
      // silently fail
    } finally {
      isFetchingVersions.value = false;
    }
  };

  const closeDropdownOnClick = (e: MouseEvent) => {
    if (!(e.target as HTMLElement).closest('.custom-select-container')) {
      isDropdownOpen.value = false;
    }
  };

  const openDropdown = async (e: Event) => {
    if (isDropdownOpen.value) return;
    isDropdownOpen.value = true;
    isUserTyping.value = false;
    await nextTick();
    if (e.target && (e.target as HTMLInputElement).select) {
      (e.target as HTMLInputElement).select();
    }
    if (dropdownListRef.value && e.target) {
      const rect = (e.target as HTMLElement).getBoundingClientRect();
      const dropdownHeight = dropdownListRef.value.offsetHeight || 250;
      dropdownDirection.value = rect.bottom + dropdownHeight > window.innerHeight - 20 ? 'up' : 'down';
      setTimeout(() => {
        if (dropdownListRef.value) {
          const list = dropdownListRef.value.querySelector('.versions-list');
          const selectedEl = list?.querySelector('.version-card.selected') as HTMLElement;
          if (list && selectedEl) {
            list.scrollTop = selectedEl.offsetTop - list.clientHeight / 2 + selectedEl.clientHeight / 2;
          }
        }
      }, 10);
    }
  };

  const selectVersion = (v: { id: string; label: string }) => {
    versionSearch.value = v.label;
    isDropdownOpen.value = false;
    onSelect(v.id, versionUrls.value[v.id] || '');
  };

  const onSearchInput = () => {
    isUserTyping.value = true;
    onSelect(versionSearch.value, versionUrls.value[versionSearch.value] || '');
  };

  onMounted(async () => {
    document.addEventListener('click', closeDropdownOnClick);
    await fetchVersions();
  });

  onUnmounted(() => {
    document.removeEventListener('click', closeDropdownOnClick);
  });

  return {
    versions,
    filteredVersions,
    isFetchingVersions,
    versionSearch,
    isDropdownOpen,
    dropdownDirection,
    fetchVersions,
    openDropdown,
    selectVersion,
    onSearchInput,
  };
};

