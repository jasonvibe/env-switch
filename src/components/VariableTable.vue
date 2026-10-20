<template>
  <div class="table-container">
    <!-- Filter & Search Toolbar -->
    <div class="toolbar">
      <div class="search-box">
        <Search :size="13" class="search-icon" />
        <input
          v-model="state.searchQuery"
          type="text"
          placeholder="搜索变量名、描述..."
          class="search-input"
        />
        <button
          v-if="state.searchQuery"
          class="clear-btn"
          @click="state.searchQuery = ''"
        >
          <X :size="12" />
        </button>
      </div>

      <div class="filter-pills">
        <button
          class="pill-btn"
          :class="{ active: state.filterType === 'all' }"
          @click="state.filterType = 'all'"
        >
          全部 ({{ totalCount }})
        </button>
        <button
          class="pill-btn"
          :class="{ active: state.filterType === 'secrets' }"
          @click="state.filterType = 'secrets'"
        >
          <KeyRound :size="11" />
          <span>密文 ({{ secretCount }})</span>
        </button>
        <button
          class="pill-btn"
          :class="{ active: state.filterType === 'enabled' }"
          @click="state.filterType = 'enabled'"
        >
          启用中
        </button>
        <button
          class="pill-btn"
          :class="{ active: state.filterType === 'disabled' }"
          @click="state.filterType = 'disabled'"
        >
          已停用
        </button>
      </div>
    </div>

    <!-- Empty State -->
    <div v-if="filteredVariables.length === 0" class="empty-view">
      <div class="empty-card">
        <SlidersHorizontal :size="32" class="text-dim mb-2" />
        <div class="empty-title">当前环境暂无匹配的环境变量</div>
        <div class="empty-desc">
          点击右上角“添加变量”或“导入 .env”快速配置变量
        </div>
      </div>
    </div>

    <!-- Data Table -->
    <div v-else class="table-scroll">
      <table class="env-table">
        <thead>
          <tr>
            <th style="width: 44px; text-align: center;">状态</th>
            <th style="width: 200px;">变量名 (KEY)</th>
            <th>变量值 (VALUE)</th>
            <th style="width: 110px;">存储模式</th>
            <th style="width: 160px;">说明 / 备注</th>
            <th style="width: 80px; text-align: right;">操作</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="item in filteredVariables"
            :key="item.id"
            :class="{ disabled: !item.enabled }"
          >
            <!-- Toggle Switch -->
            <td style="text-align: center;">
              <label class="switch">
                <input
                  type="checkbox"
                  :checked="item.enabled"
                  @change="toggleVariable(item.id)"
                />
                <span class="slider"></span>
              </label>
            </td>

            <!-- Key -->
            <td>
              <div class="key-cell">
                <span class="key-text mono">{{ item.key }}</span>
                <button
                  class="btn-copy-mini"
                  title="复制变量名"
                  @click="copyText(item.key, 'key_' + item.id)"
                >
                  <Check v-if="copiedId === 'key_' + item.id" :size="11" class="text-success" />
                  <Copy v-else :size="11" />
                </button>
              </div>
            </td>

            <!-- Value -->
            <td>
              <div class="value-cell">
                <!-- Secret Mode -->
                <template v-if="item.is_secret">
                  <div class="secret-box">
                    <span
                      v-if="revealedSecrets[item.id] !== undefined"
                      class="mono val-text"
                      :class="{ 'val-empty': !revealedSecrets[item.id] }"
                    >
                      {{ revealedSecrets[item.id] || '(钥匙串中无密文，请点击右侧编辑设置)' }}
                    </span>
                    <span v-else class="masked-dots">••••••••••••••••</span>

                    <button
                      class="btn-reveal"
                      :title="revealedSecrets[item.id] !== undefined ? '隐藏明文' : '从 Keychain 查看明文'"
                      @click="toggleReveal(item)"
                    >
                      <EyeOff v-if="revealedSecrets[item.id] !== undefined" :size="12" />
                      <Eye v-else :size="12" />
                    </button>

                    <button
                      class="btn-copy-mini"
                      title="复制真实值"
                      @click="copySecretVal(item)"
                    >
                      <Check v-if="copiedId === 'val_' + item.id" :size="11" class="text-success" />
                      <Copy v-else :size="11" />
                    </button>
                  </div>
                </template>

                <!-- Plain Mode -->
                <template v-else>
                  <div class="plain-box">
                    <span class="mono val-text" :title="item.value">{{ item.value || '(空)' }}</span>
                    <button
                      class="btn-copy-mini"
                      title="复制变量值"
                      @click="copyText(item.value, 'val_' + item.id)"
                    >
                      <Check v-if="copiedId === 'val_' + item.id" :size="11" class="text-success" />
                      <Copy v-else :size="11" />
                    </button>
                  </div>
                </template>
              </div>
            </td>

            <!-- Storage Mode / Type -->
            <td>
              <span v-if="item.is_secret" class="type-badge badge-secret" title="加密保存在 macOS 原生钥匙串">
                <Lock :size="10" />
                <span>Keychain</span>
              </span>
              <span v-else class="type-badge badge-plain">
                明文
              </span>
            </td>

            <!-- Description -->
            <td>
              <span class="desc-text" :title="item.description">{{ item.description || '-' }}</span>
            </td>

            <!-- Actions -->
            <td style="text-align: right;">
              <div class="actions-cell">
                <button
                  class="action-btn"
                  title="编辑"
                  @click="$emit('edit-var', item)"
                >
                  <Pencil :size="12" />
                </button>
                <button
                  class="action-btn action-delete"
                  title="删除"
                  @click="handleDelete(item)"
                >
                  <Trash2 :size="12" />
                </button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Delete Confirm Modal -->
    <ConfirmModal
      v-if="deletingItem"
      title="删除环境变量"
      :message="`确定要删除环境变量 ${deletingItem.key} 吗？此操作无法撤销。`"
      confirm-text="确认删除"
      :is-danger="true"
      @confirm="doDeleteVariable"
      @cancel="deletingItem = null"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import {
  Search,
  X,
  KeyRound,
  SlidersHorizontal,
  Copy,
  Check,
  Eye,
  EyeOff,
  Lock,
  Pencil,
  Trash2,
} from "lucide-vue-next";
import { useEnvStore } from "../store";
import { api } from "../api";
import type { EnvVariable } from "../types";
import ConfirmModal from "./ConfirmModal.vue";

defineEmits<{
  (e: "edit-var", item: EnvVariable): void;
}>();

const {
  state,
  filteredVariables,
  toggleVariable,
  deleteVariable,
} = useEnvStore();

const revealedSecrets = ref<Record<string, string>>({});
const copiedId = ref<string | null>(null);
const deletingItem = ref<EnvVariable | null>(null);

const allCurrentVars = computed(() => {
  if (!state.data || !state.selectedProfileId) return [];
  return state.data.variables.filter(v => v.profile_id === state.selectedProfileId);
});

const totalCount = computed(() => allCurrentVars.value.length);
const secretCount = computed(() => allCurrentVars.value.filter(v => v.is_secret).length);

async function toggleReveal(item: EnvVariable) {
  if (revealedSecrets.value[item.id] !== undefined) {
    delete revealedSecrets.value[item.id];
  } else {
    try {
      const val = await api.getSecretVal(item.profile_id, item.key);
      revealedSecrets.value[item.id] = val;
    } catch (e: any) {
      alert("获取钥匙串密文失败: " + e);
    }
  }
}

async function copySecretVal(item: EnvVariable) {
  try {
    let val = revealedSecrets.value[item.id];
    if (val === undefined) {
      val = await api.getSecretVal(item.profile_id, item.key);
    }
    await copyText(val || "", "val_" + item.id);
  } catch (e: any) {
    console.error("读取密文失败: " + e);
  }
}

async function copyText(text: string, id: string) {
  try {
    // 1. 优先调用原生 Tauri 命令 (macOS pbcopy，彻底绕过 WebKit 权限与安全上下文限制)
    await api.copyToClipboard(text);
  } catch {
    // 2. 降级到 navigator.clipboard
    try {
      await navigator.clipboard.writeText(text);
    } catch {
      // 3. 降级到 DOM textarea execCommand
      const el = document.createElement("textarea");
      el.value = text;
      el.style.position = "fixed";
      el.style.opacity = "0";
      document.body.appendChild(el);
      el.select();
      document.execCommand("copy");
      document.body.removeChild(el);
    }
  }

  copiedId.value = id;
  setTimeout(() => {
    if (copiedId.value === id) {
      copiedId.value = null;
    }
  }, 1500);
}

function handleDelete(item: EnvVariable) {
  deletingItem.value = item;
}

async function doDeleteVariable() {
  if (!deletingItem.value) return;
  const id = deletingItem.value.id;
  deletingItem.value = null;
  await deleteVariable(id);
}
</script>

<style scoped>
.table-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background-color: var(--bg-app);
}

.toolbar {
  padding: 10px 16px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid var(--border-subtle);
  background: rgba(255, 255, 255, 0.01);
}

.search-box {
  position: relative;
  width: 260px;
  display: flex;
  align-items: center;
}

.search-icon {
  position: absolute;
  left: 9px;
  color: var(--text-dim);
  pointer-events: none;
}

.search-input {
  width: 100%;
  padding-left: 28px;
  padding-right: 24px;
  height: 28px;
  font-size: 12px;
}

.clear-btn {
  position: absolute;
  right: 6px;
  background: transparent;
  color: var(--text-dim);
  display: flex;
  align-items: center;
}

.clear-btn:hover {
  color: var(--text-main);
}

.filter-pills {
  display: flex;
  align-items: center;
  gap: 4px;
}

.pill-btn {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 4px 10px;
  border-radius: 12px;
  font-size: 11px;
  color: var(--text-muted);
  background: var(--bg-card);
  border: 1px solid var(--border-subtle);
}

.pill-btn:hover {
  background: var(--bg-card-hover);
  color: var(--text-main);
}

.pill-btn.active {
  background: rgba(10, 132, 255, 0.15);
  border-color: rgba(10, 132, 255, 0.4);
  color: var(--accent-primary);
  font-weight: 500;
}

.table-scroll {
  flex: 1;
  overflow: auto;
}

.env-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 12px;
}

.env-table th {
  position: sticky;
  top: 0;
  background: #16171b;
  color: var(--text-dim);
  font-weight: 600;
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  padding: 8px 12px;
  border-bottom: 1px solid var(--border-subtle);
  text-align: left;
  z-index: 10;
}

.env-table td {
  padding: 8px 12px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.04);
  vertical-align: middle;
}

.env-table tr:hover td {
  background: rgba(255, 255, 255, 0.02);
}

.env-table tr.disabled td {
  opacity: 0.5;
}

/* Switch */
.switch {
  position: relative;
  display: inline-block;
  width: 28px;
  height: 16px;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  inset: 0;
  background-color: rgba(255, 255, 255, 0.15);
  transition: 0.2s;
  border-radius: 16px;
}

.slider:before {
  position: absolute;
  content: "";
  height: 12px;
  width: 12px;
  left: 2px;
  bottom: 2px;
  background-color: white;
  transition: 0.2s;
  border-radius: 50%;
}

input:checked + .slider {
  background-color: var(--accent-success);
}

input:checked + .slider:before {
  transform: translateX(12px);
}

/* Key cell */
.key-cell {
  display: flex;
  align-items: center;
  gap: 6px;
}

.key-text {
  font-weight: 600;
  color: #60a5fa;
  font-size: 12px;
}

.btn-copy-mini {
  background: transparent;
  color: var(--text-dim);
  padding: 2px;
  border-radius: 4px;
  display: inline-flex;
  align-items: center;
}

.btn-copy-mini:hover {
  color: var(--text-main);
  background: rgba(255, 255, 255, 0.08);
}

/* Value cell */
.value-cell {
  display: flex;
  align-items: center;
}

.secret-box, .plain-box {
  display: flex;
  align-items: center;
  gap: 6px;
  max-width: 320px;
}

.masked-dots {
  letter-spacing: 2px;
  color: var(--text-dim);
  font-size: 10px;
}

.val-text {
  color: var(--text-secondary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.val-empty {
  color: var(--text-dim);
  font-style: italic;
  font-size: 11px;
}

.btn-reveal {
  background: transparent;
  color: var(--text-muted);
  padding: 2px 4px;
  border-radius: 4px;
  display: inline-flex;
  align-items: center;
}

.btn-reveal:hover {
  color: var(--text-main);
  background: rgba(255, 255, 255, 0.08);
}

/* Badges */
.type-badge {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: 10px;
  padding: 2px 6px;
  border-radius: 4px;
  font-weight: 500;
}

.badge-secret {
  background: rgba(191, 90, 242, 0.15);
  color: var(--accent-purple);
  border: 1px solid rgba(191, 90, 242, 0.3);
}

.badge-plain {
  background: rgba(255, 255, 255, 0.05);
  color: var(--text-dim);
}

.desc-text {
  color: var(--text-dim);
  font-size: 11px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  display: block;
}

/* Actions */
.actions-cell {
  display: inline-flex;
  align-items: center;
  gap: 2px;
}

.action-btn {
  background: transparent;
  color: var(--text-dim);
  padding: 4px;
  border-radius: 4px;
  display: inline-flex;
  align-items: center;
}

.action-btn:hover {
  background: rgba(255, 255, 255, 0.08);
  color: var(--text-main);
}

.action-delete:hover {
  color: var(--accent-danger);
  background: var(--accent-danger-bg);
}

/* Empty View */
.empty-view {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.empty-card {
  text-align: center;
  padding: 32px;
}

.empty-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-secondary);
  margin-bottom: 4px;
}

.empty-desc {
  font-size: 12px;
  color: var(--text-dim);
}

.mb-2 {
  margin-bottom: 8px;
}

.text-dim {
  color: var(--text-dim);
}

.text-success {
  color: var(--accent-success);
}
</style>
