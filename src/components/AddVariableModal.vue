<template>
  <div class="modal-overlay" @click.self="$emit('close')">
    <div class="modal-content">
      <div class="modal-header">
        <div class="modal-title">
          <KeyRound :size="16" class="text-accent" />
          <span>{{ editingVar ? '编辑环境变量' : '添加环境变量' }}</span>
        </div>
        <button class="btn-icon-close" @click="$emit('close')">
          <X :size="14" />
        </button>
      </div>

      <div class="modal-body">
        <!-- Key -->
        <div class="form-group">
          <label>变量名 (KEY)</label>
          <input
            v-model="form.key"
            type="text"
            placeholder="例如：OPENAI_API_KEY, DATABASE_URL"
            class="mono uppercase-input"
            :disabled="!!editingVar"
          />
          <span class="hint">统一使用大写与下划线命名规范</span>
        </div>

        <!-- Secret Toggle -->
        <div class="form-group checkbox-group">
          <label class="checkbox-label">
            <input type="checkbox" v-model="form.isSecret" />
            <div class="checkbox-desc">
              <div class="cb-title">
                <Lock :size="12" class="text-purple" />
                <span>作为 macOS 钥匙串密文存储 (Keychain Secret)</span>
              </div>
              <div class="cb-subtitle">
                敏感凭证（如 API 密钥、私钥、数据库密码）将安全托管至系统钥匙串，本地 JSON 绝不明文落盘。
              </div>
            </div>
          </label>
        </div>

        <!-- Value -->
        <div class="form-group">
          <label>{{ form.isSecret ? '密文内容 (Secret Value)' : '变量值 (Value)' }}</label>
          <input
            v-if="!form.isSecret"
            v-model="form.value"
            type="text"
            placeholder="变量取值"
            class="mono"
          />
          <input
            v-else
            v-model="form.secretValue"
            type="password"
            :placeholder="editingVar ? '留空表示保持当前 Keychain 密文不变' : '输入敏感凭据'"
            class="mono"
          />
        </div>

        <!-- Description -->
        <div class="form-group">
          <label>说明 / 用途备注 (可选)</label>
          <input
            v-model="form.description"
            type="text"
            placeholder="例如：开发环境 OpenAI Key，调用 GPT-4o"
          />
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn" @click="$emit('close')">取消</button>
        <button class="btn btn-primary" :disabled="!canSubmit" @click="handleSubmit">
          {{ editingVar ? '保存修改' : '确认添加' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, computed, onMounted } from "vue";
import { KeyRound, X, Lock } from "lucide-vue-next";
import type { EnvVariable } from "../types";
import { useEnvStore } from "../store";

const props = defineProps<{
  editingVar?: EnvVariable | null;
}>();

const emit = defineEmits<{
  (e: "close"): void;
}>();

const { addVariable, updateVariable } = useEnvStore();

const form = reactive({
  key: "",
  value: "",
  secretValue: "",
  isSecret: false,
  description: "",
});

onMounted(() => {
  if (props.editingVar) {
    form.key = props.editingVar.key;
    form.value = props.editingVar.value;
    form.isSecret = props.editingVar.is_secret;
    form.description = props.editingVar.description;
  }
});

const canSubmit = computed(() => {
  if (!form.key.trim()) return false;
  return true;
});

async function handleSubmit() {
  if (!canSubmit.value) return;

  if (props.editingVar) {
    await updateVariable(
      props.editingVar.id,
      {
        value: form.isSecret ? "" : form.value,
        is_secret: form.isSecret,
        description: form.description,
      },
      form.secretValue ? form.secretValue : undefined
    );
  } else {
    const val = form.isSecret ? form.secretValue : form.value;
    await addVariable(form.key, val, form.isSecret, form.description);
  }

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

.text-purple {
  color: var(--accent-purple);
}

.btn-icon-close {
  background: transparent;
  color: var(--text-dim);
  padding: 4px;
  border-radius: 4px;
}

.btn-icon-close:hover {
  color: var(--text-main);
  background: rgba(255, 255, 255, 0.08);
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

.hint {
  font-size: 10px;
  color: var(--text-dim);
}

.uppercase-input {
  text-transform: uppercase;
}

.checkbox-group {
  background: rgba(191, 90, 242, 0.08);
  border: 1px solid rgba(191, 90, 242, 0.2);
  border-radius: var(--radius-sm);
  padding: 8px 10px;
}

.checkbox-label {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  cursor: pointer;
}

.checkbox-desc {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.cb-title {
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: 12px;
  font-weight: 500;
  color: var(--text-main);
}

.cb-subtitle {
  font-size: 10px;
  color: var(--text-dim);
}

.modal-footer {
  margin-top: 20px;
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}
</style>
