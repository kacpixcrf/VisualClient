import { ref, onMounted } from 'vue';

const MODRINTH_API = 'https://api.modrinth.com/v2/search';
const MODRINTH_LIMIT = 5;

export interface ModrinthProject {
  slug: string;
  title: string;
  description: string;
  icon_url: string;
  gallery: string[];
}

const buildUrl = (type: string) =>
  `${MODRINTH_API}?limit=${MODRINTH_LIMIT}&facets=[[%22project_type:${type}%22]]`;

export const useModrinth = () => {
  const shaders = ref<ModrinthProject[]>([]);
  const mods = ref<ModrinthProject[]>([]);
  const modpacks = ref<ModrinthProject[]>([]);
  const isLoadingShaders = ref(true);
  const isLoadingMods = ref(true);
  const isLoadingModpacks = ref(true);
  const isOffline = ref(false);

  const fetchCategory = async (
    url: string,
    target: typeof shaders,
    loading: typeof isLoadingShaders
  ) => {
    try {
      const res = await fetch(url);
      const data = await res.json();
      target.value = data.hits || [];
    } catch {
      isOffline.value = true;
    } finally {
      loading.value = false;
    }
  };

  onMounted(() => {
    fetchCategory(buildUrl('shader'), shaders, isLoadingShaders);
    fetchCategory(buildUrl('mod'), mods, isLoadingMods);
    fetchCategory(buildUrl('modpack'), modpacks, isLoadingModpacks);
  });

  return { shaders, mods, modpacks, isLoadingShaders, isLoadingMods, isLoadingModpacks, isOffline };
};
