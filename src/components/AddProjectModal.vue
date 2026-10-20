<template>
  <div class="modal-overlay" @click.self="$emit('close')">
    <div class="modal-content">
      <div class="modal-header">
        <div class="modal-title">
          <Boxes :size="16" class="text-accent" />
          <span>新建项目 (New Project)</span>
        </div>
        <button class="btn-icon-close" @click="$emit('close')">
          <X :size="14" />
        </button>
      </div>

      <div class="modal-body">
        <div class="form-group">
          <label>所属工作区</label>
          <input :value="currentWorkspace?.name" disabled class="disabled-input" />
        </div>

        <div class="form-group">
          <label>项目名称</label>
          <input
            v-model="name"
            type="text"
            placeholder="例如：beaver-cloud, tcl-backend, ai-agent"
            autofocus
          />
        </div>

        <div class="form-group">
          <label>项目描述 (可选)</label>
          <input
            v-model="description"
            type="text"
            placeholder="简要说明此项目的用途"
          />
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn" @click="$emit('close')">取消</button>
        <button class="btn btn-primary" :disabled="!name.trim()" @click="handleSubmit">
          创建项目
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { Boxes, X } from "lucide-vue-next";
import { useEnvStore } from "../store";

const emit = defineEmits<{
  (e: "close"): void;
}>();

const { state, currentWorkspace, addProject } = useEnvStore();

const name = ref("");
const description = ref("");

async function handleSubmit() {
  if (!name.value.trim() || !state.selectedWorkspaceId) return;
  await addProject(state.selectedWorkspaceId, name.value.trim(), description.value.trim());
  emit("close");
}
</script>

<style scoped>
.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;
}

.modal-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  font-weight: 600;
}

.text-accent {
  color: var(--accent-primary);
}

.btn-icon-close {
  background: transparent;
  color: var(--text-dim);
  padding: 4px;
  border-radius: 4px;
}

.modal-body {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-group label {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-secondary);
}

.disabled-input {
  opacity: 0.6;
  cursor: not-allowed;
}

.modal-footer {
  margin-top: 20px;
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}
</style>
