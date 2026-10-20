<template>
  <div class="modal-overlay" @click.self="$emit('cancel')">
    <div class="modal-content modal-confirm">
      <div class="confirm-body">
        <div class="confirm-icon-box" :class="{ 'is-danger': isDanger }">
          <AlertTriangle v-if="isDanger" :size="20" class="text-danger" />
          <Info v-else :size="20" class="text-accent" />
        </div>

        <div class="confirm-text">
          <div class="confirm-title">{{ title }}</div>
          <div class="confirm-desc">{{ message }}</div>
        </div>
      </div>

      <div class="confirm-footer">
        <button class="btn btn-sm" @click="$emit('cancel')">
          {{ cancelText || '取消' }}
        </button>
        <button
          class="btn btn-sm"
          :class="isDanger ? 'btn-danger-solid' : 'btn-primary'"
          @click="$emit('confirm')"
        >
          {{ confirmText || '确认' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { AlertTriangle, Info } from "lucide-vue-next";

defineProps<{
  title: string;
  message: string;
  confirmText?: string;
  cancelText?: string;
  isDanger?: boolean;
}>();

defineEmits<{
  (e: "confirm"): void;
  (e: "cancel"): void;
}>();
</script>

<style scoped>
.modal-confirm {
  max-width: 380px;
  padding: 18px 20px;
}

.confirm-body {
  display: flex;
  align-items: flex-start;
  gap: 14px;
}

.confirm-icon-box {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  background: rgba(10, 132, 255, 0.15);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.confirm-icon-box.is-danger {
  background: rgba(255, 69, 58, 0.15);
}

.text-danger {
  color: var(--accent-danger);
}

.text-accent {
  color: var(--accent-primary);
}

.confirm-text {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.confirm-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-main);
}

.confirm-desc {
  font-size: 12px;
  color: var(--text-muted);
  line-height: 1.5;
}

.confirm-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  margin-top: 18px;
}

.btn-danger-solid {
  background: var(--accent-danger);
  color: #ffffff;
  border: none;
  font-weight: 500;
}

.btn-danger-solid:hover {
  background: #e03b30;
}
</style>
