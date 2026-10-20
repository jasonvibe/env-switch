<template>
  <div class="app-root">
    <!-- Loading state -->
    <div v-if="state.loading" class="loading-screen">
      <div class="spinner"></div>
      <span class="loading-text">加载 EnvSwitch 配置中...</span>
    </div>

    <!-- Main View -->
    <div v-else class="app-layout">
      <!-- Sidebar -->
      <Sidebar
        @open-add-project="showAddProjectModal = true"
        @open-add-profile="handleOpenAddProfile"
      />

      <!-- Content Area -->
      <main class="main-content">
        <!-- Topbar -->
        <Topbar
          @open-import="showImportExportModal = true"
          @open-export="showImportExportModal = true"
          @open-add-var="handleOpenAddVar"
        />

        <!-- Variable Table -->
        <VariableTable
          @edit-var="handleEditVar"
        />
      </main>
    </div>

    <!-- Modals -->
    <AddVariableModal
      v-if="showAddVarModal"
      :editing-var="editingVariable"
      @close="handleCloseAddVar"
    />

    <ImportExportModal
      v-if="showImportExportModal"
      @close="showImportExportModal = false"
    />

    <AddProjectModal
      v-if="showAddProjectModal"
      @close="showAddProjectModal = false"
    />

    <AddProfileModal
      v-if="showAddProfileModal"
      :project-id="addProfileProjectId"
      @close="showAddProfileModal = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { listen } from "@tauri-apps/api/event";
import { useEnvStore } from "./store";
import type { EnvVariable } from "./types";

import Sidebar from "./components/Sidebar.vue";
import Topbar from "./components/Topbar.vue";
import VariableTable from "./components/VariableTable.vue";
import AddVariableModal from "./components/AddVariableModal.vue";
import ImportExportModal from "./components/ImportExportModal.vue";
import AddProjectModal from "./components/AddProjectModal.vue";
import AddProfileModal from "./components/AddProfileModal.vue";

const { state, init, clearSystemEnv } = useEnvStore();

const showAddVarModal = ref(false);
const editingVariable = ref<EnvVariable | null>(null);

const showImportExportModal = ref(false);
const showAddProjectModal = ref(false);
const showAddProfileModal = ref(false);
const addProfileProjectId = ref("");

function handleOpenAddVar() {
  editingVariable.value = null;
  showAddVarModal.value = true;
}

function handleEditVar(item: EnvVariable) {
  editingVariable.value = item;
  showAddVarModal.value = true;
}

function handleCloseAddVar() {
  showAddVarModal.value = false;
  editingVariable.value = null;
}

function handleOpenAddProfile(projId: string) {
  addProfileProjectId.value = projId;
  showAddProfileModal.value = true;
}

onMounted(async () => {
  await init();

  // Listen to tray events
  try {
    await listen("tray-clear-env", async () => {
      await clearSystemEnv();
    });
  } catch (err) {
    console.warn("Tray event listen not available in browser mode:", err);
  }
});
</script>

<style scoped>
.app-root {
  width: 100vw;
  height: 100vh;
  display: flex;
  overflow: hidden;
  background-color: var(--bg-app);
}

.loading-screen {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
}

.spinner {
  width: 24px;
  height: 24px;
  border: 2px solid rgba(255, 255, 255, 0.1);
  border-top-color: var(--accent-primary);
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.loading-text {
  font-size: 12px;
  color: var(--text-dim);
}

.app-layout {
  display: flex;
  width: 100%;
  height: 100%;
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background-color: var(--bg-app);
}
</style>
