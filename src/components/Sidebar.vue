<template>
  <aside class="sidebar">
    <!-- Header / Branding -->
    <div class="sidebar-header" data-tauri-drag-region>
      <div class="logo-box">
        <div class="app-icon">
          <Terminal :size="16" class="text-accent" />
        </div>
        <div class="app-title">
          <span class="name">EnvSwitch</span>
          <span class="badge">v0.1.0</span>
        </div>
      </div>
    </div>

    <!-- Workspace Selector -->
    <div class="section-box">
      <div class="section-label">工作区 (Workspace)</div>
      <div class="workspace-pill-group">
        <button
          v-for="ws in state.data?.workspaces || []"
          :key="ws.id"
          class="ws-pill"
          :class="{ active: ws.id === state.selectedWorkspaceId }"
          @click="selectWorkspace(ws.id)"
        >
          <FolderGit2 :size="13" />
          <span>{{ ws.name }}</span>
        </button>
      </div>
    </div>

    <!-- Projects & Profiles Navigation -->
    <div class="nav-tree">
      <div class="section-header-row">
        <span class="section-label">项目与环境</span>
        <button class="btn btn-icon btn-sm" title="添加新项目" @click="$emit('open-add-project')">
          <Plus :size="14" />
        </button>
      </div>

      <div v-if="projectsForCurrentWorkspace.length === 0" class="empty-state">
        暂无项目，点击右上角加号创建
      </div>

      <div
        v-for="proj in projectsForCurrentWorkspace"
        :key="proj.id"
        class="project-item"
        :class="{ active: proj.id === state.selectedProjectId }"
      >
        <div class="project-header" @click="selectProject(proj.id)">
          <div class="project-title">
            <Boxes :size="14" class="icon-project" />
            <span class="proj-name">{{ proj.name }}</span>
          </div>
          <button
            class="btn-icon-subtle"
            title="为该项目添加环境 Profile"
            @click.stop="$emit('open-add-profile', proj.id)"
          >
            <Plus :size="12" />
          </button>
        </div>

        <!-- Profiles under selected project -->
        <div v-if="proj.id === state.selectedProjectId" class="profile-list">
          <div
            v-for="prof in profilesForCurrentProject"
            :key="prof.id"
            class="profile-item"
            :class="{ selected: prof.id === state.selectedProfileId }"
            @click="selectProfile(prof.id)"
          >
            <div class="profile-left">
              <span class="color-dot" :style="{ backgroundColor: prof.color || '#3b82f6' }"></span>
              <span class="prof-name">{{ prof.name }}</span>
            </div>

            <!-- Injected / Active Indicator -->
            <div v-if="prof.id === state.data?.active_profile_id" class="active-badge" title="系统 GUI 正在生效此配置">
              <span class="pulse-dot"></span>
              <span>生效中</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Bottom Actions / System Status -->
    <div class="sidebar-footer">
      <div class="system-status-card" v-if="state.data?.active_profile_id">
        <div class="status-top">
          <CheckCircle2 :size="13" class="text-success" />
          <span class="status-text">系统 GUI 已注入变量</span>
        </div>
        <div class="status-desc">
          已向 launchctl 注入 {{ state.data?.applied_keys?.length || 0 }} 个环境变量
        </div>
      </div>

      <button
        class="btn btn-danger btn-sm w-full"
        v-if="state.data?.active_profile_id"
        @click="showClearConfirm = true"
        title="从 launchctl 清除所有由 EnvSwitch 注入的环境变量"
      >
        <Trash2 :size="12" />
        <span>清除系统注入变量</span>
      </button>
    </div>

    <!-- Confirm Modal -->
    <ConfirmModal
      v-if="showClearConfirm"
      title="清除系统注入变量"
      message="确定要从 macOS 系统 (launchctl) 中清除当前注入的所有环境变量吗？已打开的终端和应用将恢复默认环境。"
      confirm-text="确认清除"
      :is-danger="true"
      @confirm="doClearSystemEnv"
      @cancel="showClearConfirm = false"
    />
  </aside>
</template>

<script setup lang="ts">
import { ref } from "vue";
import {
  Terminal,
  FolderGit2,
  Boxes,
  Plus,
  CheckCircle2,
  Trash2,
} from "lucide-vue-next";
import { useEnvStore } from "../store";
import ConfirmModal from "./ConfirmModal.vue";

defineEmits<{
  (e: "open-add-project"): void;
  (e: "open-add-profile", projectId: string): void;
}>();

const {
  state,
  projectsForCurrentWorkspace,
  profilesForCurrentProject,
  selectWorkspace,
  selectProject,
  selectProfile,
  clearSystemEnv,
} = useEnvStore();

const showClearConfirm = ref(false);

async function doClearSystemEnv() {
  showClearConfirm.value = false;
  try {
    await clearSystemEnv();
  } catch (err: any) {
    console.error("清除环境变量失败:", err);
  }
}
</script>

<style scoped>
.sidebar {
  width: 240px;
  min-width: 240px;
  background-color: var(--bg-sidebar);
  border-right: 1px solid var(--border-subtle);
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.sidebar-header {
  padding: 16px 14px 12px;
  border-bottom: 1px solid var(--border-subtle);
}

.logo-box {
  display: flex;
  align-items: center;
  gap: 8px;
}

.app-icon {
  width: 28px;
  height: 28px;
  border-radius: var(--radius-sm);
  background: rgba(10, 132, 255, 0.15);
  display: flex;
  align-items: center;
  justify-content: center;
}

.text-accent {
  color: var(--accent-primary);
}

.app-title {
  display: flex;
  align-items: center;
  gap: 6px;
}

.app-title .name {
  font-size: 14px;
  font-weight: 600;
  letter-spacing: -0.2px;
}

.app-title .badge {
  font-size: 10px;
  color: var(--text-dim);
  background: rgba(255, 255, 255, 0.05);
  padding: 1px 4px;
  border-radius: 4px;
}

.section-box {
  padding: 12px 14px;
  border-bottom: 1px solid var(--border-subtle);
}

.section-label {
  font-size: 10px;
  font-weight: 600;
  text-transform: uppercase;
  color: var(--text-dim);
  letter-spacing: 0.5px;
  margin-bottom: 8px;
  display: block;
}

.workspace-pill-group {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.ws-pill {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 8px;
  border-radius: var(--radius-sm);
  font-size: 12px;
  color: var(--text-secondary);
  background: transparent;
  text-align: left;
}

.ws-pill:hover {
  background: rgba(255, 255, 255, 0.04);
}

.ws-pill.active {
  background: rgba(10, 132, 255, 0.15);
  color: var(--accent-primary);
  font-weight: 500;
}

.nav-tree {
  flex: 1;
  overflow-y: auto;
  padding: 12px 10px;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.section-header-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 4px;
  margin-bottom: 4px;
}

.empty-state {
  font-size: 11px;
  color: var(--text-dim);
  text-align: center;
  padding: 24px 8px;
}

.project-item {
  border-radius: var(--radius-sm);
  overflow: hidden;
}

.project-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 8px;
  border-radius: var(--radius-sm);
  cursor: pointer;
  color: var(--text-secondary);
}

.project-header:hover {
  background: rgba(255, 255, 255, 0.04);
}

.project-item.active > .project-header {
  font-weight: 500;
  color: var(--text-main);
}

.project-title {
  display: flex;
  align-items: center;
  gap: 6px;
}

.icon-project {
  color: var(--text-muted);
}

.proj-name {
  font-size: 12px;
}

.btn-icon-subtle {
  background: transparent;
  color: var(--text-dim);
  padding: 2px 4px;
  border-radius: 4px;
}

.btn-icon-subtle:hover {
  color: var(--text-main);
  background: rgba(255, 255, 255, 0.1);
}

.profile-list {
  padding-left: 12px;
  margin-top: 2px;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.profile-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 5px 8px;
  border-radius: var(--radius-sm);
  cursor: pointer;
  color: var(--text-muted);
  font-size: 12px;
}

.profile-item:hover {
  background: rgba(255, 255, 255, 0.04);
  color: var(--text-secondary);
}

.profile-item.selected {
  background: var(--bg-card);
  color: var(--text-main);
  font-weight: 500;
  border: 1px solid var(--border-subtle);
}

.profile-left {
  display: flex;
  align-items: center;
  gap: 6px;
}

.color-dot {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  flex-shrink: 0;
}

.active-badge {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 10px;
  color: var(--accent-success);
  background: var(--accent-success-bg);
  padding: 1px 6px;
  border-radius: 10px;
  font-weight: 500;
}

.pulse-dot {
  width: 5px;
  height: 5px;
  border-radius: 50%;
  background-color: var(--accent-success);
  box-shadow: 0 0 6px var(--accent-success);
}

.sidebar-footer {
  padding: 12px;
  border-top: 1px solid var(--border-subtle);
  background: var(--bg-sidebar);
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.system-status-card {
  background: rgba(48, 209, 88, 0.08);
  border: 1px solid rgba(48, 209, 88, 0.2);
  border-radius: var(--radius-sm);
  padding: 8px 10px;
}

.status-top {
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: 11px;
  font-weight: 600;
  color: var(--accent-success);
}

.status-desc {
  font-size: 10px;
  color: var(--text-dim);
  margin-top: 2px;
}

.text-success {
  color: var(--accent-success);
}

.w-full {
  width: 100%;
}
</style>
