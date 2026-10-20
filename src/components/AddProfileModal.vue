<template>
  <div class="modal-overlay" @click.self="$emit('close')">
    <div class="modal-content">
      <div class="modal-header">
        <div class="modal-title">
          <Layers :size="16" class="text-accent" />
          <span>新建环境配置 (New Profile)</span>
        </div>
        <button class="btn-icon-close" @click="$emit('close')">
          <X :size="14" />
        </button>
      </div>

      <div class="modal-body">
        <div class="form-group">
          <label>环境名称</label>
          <input
            v-model="name"
            type="text"
            placeholder="例如：Development, Staging, Production, Local-Test"
            autofocus
          />
        </div>

        <div class="form-group">
          <label>标识色彩</label>
          <div class="color-picker-row">
            <button
              v-for="color in presetColors"
              :key="color"
              class="color-btn"
              :class="{ selected: selectedColor === color }"
              :style="{ backgroundColor: color }"
              @click="selectedColor = color"
            >
              <Check v-if="selectedColor === color" :size="12" class="color-check" />
            </button>
          </div>
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn" @click="$emit('close')">取消</button>
        <button class="btn btn-primary" :disabled="!name.trim()" @click="handleSubmit">
          创建环境
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { Layers, X, Check } from "lucide-vue-next";
import { useEnvStore } from "../store";

const props = defineProps<{
  projectId: string;
}>();

const emit = defineEmits<{
  (e: "close"): void;
}>();

const { addProfile } = useEnvStore();

const name = ref("");
const presetColors = [
  "#10b981", // Emerald (Dev)
  "#3b82f6", // Blue (Staging)
  "#f59e0b", // Amber (UAT)
  "#ef4444", // Red (Prod)
  "#8b5cf6", // Purple
  "#ec4899", // Pink
  "#06b6d4", // Cyan
  "#6b7280", // Gray
];
const selectedColor = ref(presetColors[0]);

async function handleSubmit() {
  if (!name.value.trim() || !props.projectId) return;
  await addProfile(props.projectId, name.value.trim(), selectedColor.value);
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
  gap: 14px;
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

.color-picker-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 4px;
}

.color-btn {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  border: 2px solid transparent;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: transform 0.1s ease;
}

.color-btn:hover {
  transform: scale(1.15);
}

.color-btn.selected {
  border-color: #ffffff;
  box-shadow: 0 0 6px rgba(255, 255, 255, 0.4);
}

.color-check {
  color: #ffffff;
}

.modal-footer {
  margin-top: 20px;
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}
</style>
