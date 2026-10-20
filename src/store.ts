import { reactive, computed } from "vue";
import { api } from "./api";
import type { StoreData, Project, Profile, EnvVariable } from "./types";

export const storeState = reactive<{
  data: StoreData | null;
  loading: boolean;
  selectedWorkspaceId: string;
  selectedProjectId: string;
  selectedProfileId: string;
  searchQuery: string;
  filterType: "all" | "secrets" | "enabled" | "disabled";
}>({
  data: null,
  loading: true,
  selectedWorkspaceId: "",
  selectedProjectId: "",
  selectedProfileId: "",
  searchQuery: "",
  filterType: "all",
});

export function useEnvStore() {
  const currentWorkspace = computed(() => {
    return storeState.data?.workspaces.find(w => w.id === storeState.selectedWorkspaceId);
  });

  const projectsForCurrentWorkspace = computed(() => {
    if (!storeState.data) return [];
    return storeState.data.projects.filter(p => p.workspace_id === storeState.selectedWorkspaceId);
  });

  const currentProject = computed(() => {
    return storeState.data?.projects.find(p => p.id === storeState.selectedProjectId);
  });

  const profilesForCurrentProject = computed(() => {
    if (!storeState.data) return [];
    return storeState.data.profiles.filter(p => p.project_id === storeState.selectedProjectId);
  });

  const currentProfile = computed(() => {
    return storeState.data?.profiles.find(p => p.id === storeState.selectedProfileId);
  });

  const filteredVariables = computed(() => {
    if (!storeState.data || !storeState.selectedProfileId) return [];
    let list = storeState.data.variables.filter(v => v.profile_id === storeState.selectedProfileId);

    if (storeState.filterType === "secrets") {
      list = list.filter(v => v.is_secret);
    } else if (storeState.filterType === "enabled") {
      list = list.filter(v => v.enabled);
    } else if (storeState.filterType === "disabled") {
      list = list.filter(v => !v.enabled);
    }

    if (storeState.searchQuery.trim()) {
      const q = storeState.searchQuery.toLowerCase();
      list = list.filter(v =>
        v.key.toLowerCase().includes(q) ||
        v.value.toLowerCase().includes(q) ||
        v.description.toLowerCase().includes(q)
      );
    }

    return list;
  });

  const isCurrentProfileActive = computed(() => {
    return storeState.data?.active_profile_id === storeState.selectedProfileId;
  });

  async function init() {
    storeState.loading = true;
    try {
      const data = await api.getStore();
      storeState.data = data;

      if (data.workspaces.length > 0) {
        storeState.selectedWorkspaceId = data.workspaces[0].id;
        const projects = data.projects.filter(p => p.workspace_id === data.workspaces[0].id);
        if (projects.length > 0) {
          storeState.selectedProjectId = projects[0].id;
          const profiles = data.profiles.filter(p => p.project_id === projects[0].id);
          if (profiles.length > 0) {
            // Default to active profile if it matches this project, else first profile
            const active = profiles.find(p => p.id === data.active_profile_id);
            storeState.selectedProfileId = active ? active.id : profiles[0].id;
          }
        }
      }
    } catch (err) {
      console.error("Failed to load store data:", err);
    } finally {
      storeState.loading = false;
    }
  }

  async function persist() {
    if (!storeState.data) return;
    await api.saveStoreData(storeState.data);
  }

  async function selectWorkspace(id: string) {
    storeState.selectedWorkspaceId = id;
    const projects = projectsForCurrentWorkspace.value;
    if (projects.length > 0) {
      selectProject(projects[0].id);
    } else {
      storeState.selectedProjectId = "";
      storeState.selectedProfileId = "";
    }
  }

  async function selectProject(id: string) {
    storeState.selectedProjectId = id;
    const profiles = profilesForCurrentProject.value;
    if (profiles.length > 0) {
      storeState.selectedProfileId = profiles[0].id;
    } else {
      storeState.selectedProfileId = "";
    }
  }

  function selectProfile(id: string) {
    storeState.selectedProfileId = id;
  }

  async function applyCurrentProfile(): Promise<string[]> {
    if (!storeState.selectedProfileId || !storeState.data) return [];
    try {
      const appliedKeys = await api.applyProfile(storeState.selectedProfileId);
      storeState.data.active_profile_id = storeState.selectedProfileId;
      storeState.data.applied_keys = appliedKeys;
      for (const p of storeState.data.profiles) {
        p.is_active = p.id === storeState.selectedProfileId;
      }
      return appliedKeys;
    } catch (err: any) {
      console.error("Apply profile error:", err);
      throw err;
    }
  }

  async function clearSystemEnv() {
    if (!storeState.data) return;
    try {
      await api.clearSystemEnv();
      storeState.data.active_profile_id = undefined;
      storeState.data.applied_keys = [];
      for (const p of storeState.data.profiles) {
        p.is_active = false;
      }
    } catch (err: any) {
      console.error("Clear system env error:", err);
      throw err;
    }
  }

  async function addVariable(key: string, value: string, isSecret: boolean, description = "") {
    if (!storeState.data || !storeState.selectedProfileId) return;

    const newVar: EnvVariable = {
      id: crypto.randomUUID(),
      profile_id: storeState.selectedProfileId,
      key: key.trim().toUpperCase(),
      value: isSecret ? "" : value,
      is_secret: isSecret,
      enabled: true,
      description: description.trim(),
      updated_at: new Date().toISOString(),
    };

    if (isSecret && value) {
      await api.setSecretVal(storeState.selectedProfileId, newVar.key, value);
    }

    storeState.data.variables.push(newVar);
    await persist();
  }

  async function updateVariable(varId: string, updates: Partial<EnvVariable>, newSecretVal?: string) {
    if (!storeState.data) return;
    const index = storeState.data.variables.findIndex(v => v.id === varId);
    if (index === -1) return;

    const old = storeState.data.variables[index];
    const updated = { ...old, ...updates, updated_at: new Date().toISOString() };

    if (updated.is_secret && newSecretVal !== undefined) {
      await api.setSecretVal(updated.profile_id, updated.key, newSecretVal);
      updated.value = "";
    }

    storeState.data.variables[index] = updated;
    await persist();
  }

  async function deleteVariable(varId: string) {
    if (!storeState.data) return;
    storeState.data.variables = storeState.data.variables.filter(v => v.id !== varId);
    await persist();
  }

  async function toggleVariable(varId: string) {
    if (!storeState.data) return;
    const item = storeState.data.variables.find(v => v.id === varId);
    if (item) {
      item.enabled = !item.enabled;
      item.updated_at = new Date().toISOString();
      await persist();
    }
  }

  async function addProject(workspaceId: string, name: string, description: string) {
    if (!storeState.data) return;
    const newProj: Project = {
      id: crypto.randomUUID(),
      workspace_id: workspaceId,
      name,
      description,
      created_at: new Date().toISOString(),
    };
    storeState.data.projects.push(newProj);

    // auto create a default Dev profile
    const defaultProfile: Profile = {
      id: crypto.randomUUID(),
      project_id: newProj.id,
      name: "Development",
      color: "#10b981",
      is_active: false,
      created_at: new Date().toISOString(),
    };
    storeState.data.profiles.push(defaultProfile);

    await persist();
    storeState.selectedProjectId = newProj.id;
    storeState.selectedProfileId = defaultProfile.id;
  }

  async function addProfile(projectId: string, name: string, color: string = "#3b82f6") {
    if (!storeState.data) return;
    const newProf: Profile = {
      id: crypto.randomUUID(),
      project_id: projectId,
      name,
      color,
      is_active: false,
      created_at: new Date().toISOString(),
    };
    storeState.data.profiles.push(newProf);
    await persist();
    storeState.selectedProfileId = newProf.id;
  }

  async function importParsedPairs(pairs: [string, string][], secretKeys: Set<string>) {
    if (!storeState.data || !storeState.selectedProfileId) return;

    for (const [key, val] of pairs) {
      const isSecret = secretKeys.has(key);
      const existing = storeState.data.variables.find(
        v => v.profile_id === storeState.selectedProfileId && v.key === key
      );

      if (existing) {
        existing.is_secret = isSecret;
        existing.enabled = true;
        existing.updated_at = new Date().toISOString();
        if (isSecret) {
          await api.setSecretVal(storeState.selectedProfileId, key, val);
          existing.value = "";
        } else {
          existing.value = val;
        }
      } else {
        const newVar: EnvVariable = {
          id: crypto.randomUUID(),
          profile_id: storeState.selectedProfileId,
          key,
          value: isSecret ? "" : val,
          is_secret: isSecret,
          enabled: true,
          description: "导入的环境变量",
          updated_at: new Date().toISOString(),
        };
        if (isSecret) {
          await api.setSecretVal(storeState.selectedProfileId, key, val);
        }
        storeState.data.variables.push(newVar);
      }
    }
    await persist();
  }

  return {
    state: storeState,
    currentWorkspace,
    projectsForCurrentWorkspace,
    currentProject,
    profilesForCurrentProject,
    currentProfile,
    filteredVariables,
    isCurrentProfileActive,
    init,
    selectWorkspace,
    selectProject,
    selectProfile,
    applyCurrentProfile,
    clearSystemEnv,
    addVariable,
    updateVariable,
    deleteVariable,
    toggleVariable,
    addProject,
    addProfile,
    importParsedPairs,
  };
}
