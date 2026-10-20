<template>
  <header class="topbar" data-tauri-drag-region>
    <!-- Left: Breadcrumb -->
    <div class="breadcrumb-area">
      <span class="crumb-ws">{{ currentWorkspace?.name || '工作区' }}</span>
      <ChevronRight :size="12" class="crumb-separator" />
      <span class="crumb-proj">{{ currentProject?.name || '未选项目' }}</span>
      <ChevronRight :size="12" class="crumb-separator" />
      <div class="crumb-profile" v-if="currentProfile">
        <span class="color-indicator" :style="{ backgroundColor: currentProfile.color || '#3b82f6' }"></span>
        <span class="profile-name">{{ currentProfile.name }}</span>
      </div>
    </div>

    <!-- Right: Actions -->
    <div class="action-area">
      <!-- Apply to System button -->
      <button
        v-if="currentProfile"
        class="btn"
        :class="isCurrentProfileActive ? 'btn-success-active' : 'btn-primary'"
        :disabled="applying"
        @click="handleApply"
        :title="isCurrentProfileActive ? '该环境已生效于当前 macOS 系统 GUI' : '将此环境所有变量注入 macOS launchctl'"
      >
        <Zap :size="13" :class="{ 'animate-pulse': applying }" />
        <span>{{ isCurrentProfileActive ? '系统已生效' : '一键应用至系统' }}</span>
      </button>

      <div class="divider"></div>

      <!-- Import .env -->
      <button class="btn btn-sm" @click="$emit('open-import')" title="粘贴导入 .env 文本">
        <UploadCloud :size="13" />
        <span>导入 .env</span>
      </button>

      <!-- Export .env -->
      <button class="btn btn-sm" @click="$emit('open-export')" title="导出当前环境配置">
        <DownloadCloud :size="13" />
        <span>导出 .env</span>
      </button>

      <!-- Add Variable -->
      <button class="btn btn-primary btn-sm" @click="$emit('open-add-var')" title="添加单个环境变量">
        <Plus :size="13" />
        <span>添加变量</span>
      </button>
    </div>
  </header>
</template>

<script setup lang="ts">
import { ref } from "vue";
import {
  ChevronRight,
  Zap,
  UploadCloud,
  DownloadCloud,
  Plus,
} from "lucide-vue-next";
import { useEnvStore } from "../store";

defineEmits<{
  (e: "open-import"): void;
  (e: "open-export"): void;
  (e: "open-add-var"): void;
}>();

const {
  currentWorkspace,
  currentProject,
  currentProfile,
  isCurrentProfileActive,
  applyCurrentProfile,
} = useEnvStore();

const applying = ref(false);

async function handleApply() {
  if (applying.value) return;
  applying.value = true;
  try {
    const keys = await applyCurrentProfile();
    // Native subtle feedback
    console.log("Applied keys to system:", keys);
  } catch (err: any) {
    alert("应用环境变量失败: " + err);
  } finally {
    applying.value = false;
  }
}
</script>

<style scoped>
.topbar {
  height: 48px;
  background-color: var(--bg-app);
  border-bottom: 1px solid var(--border-subtle);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
  flex-shrink: 0;
}

.breadcrumb-area {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
}

.crumb-ws {
  color: var(--text-dim);
}

.crumb-separator {
  color: var(--text-dim);
  opacity: 0.6;
}

.crumb-proj {
  color: var(--text-secondary);
  font-weight: 500;
}

.crumb-profile {
  display: flex;
  align-items: center;
  gap: 6px;
  background: var(--bg-card);
  padding: 3px 8px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--border-subtle);
}

.color-indicator {
  width: 6px;
  height: 6px;
  border-radius: 50%;
}

.profile-name {
  color: var(--text-main);
  font-weight: 600;
  font-size: 12px;
}

.action-area {
  display: flex;
  align-items: center;
  gap: 8px;
}

.divider {
  width: 1px;
  height: 16px;
  background: var(--border-subtle);
  margin: 0 4px;
}

.btn-success-active {
  background: var(--accent-success-bg);
  color: var(--accent-success);
  border: 1px solid rgba(48, 209, 88, 0.4);
  font-weight: 600;
}

.btn-success-active:hover {
  background: rgba(48, 209, 88, 0.25);
}

.animate-pulse {
  animation: pulse 1.5s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}
</style>
