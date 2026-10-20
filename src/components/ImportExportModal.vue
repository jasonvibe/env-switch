<template>
  <div class="modal-overlay" @click.self="$emit('close')">
    <div class="modal-content modal-lg">
      <div class="modal-header">
        <div class="tab-header">
          <button
            class="tab-btn"
            :class="{ active: activeTab === 'import' }"
            @click="activeTab = 'import'"
          >
            <UploadCloud :size="14" />
            <span>导入 .env 文本</span>
          </button>
          <button
            class="tab-btn"
            :class="{ active: activeTab === 'export' }"
            @click="handleSwitchExport"
          >
            <DownloadCloud :size="14" />
            <span>导出当前环境</span>
          </button>
        </div>
        <button class="btn-icon-close" @click="$emit('close')">
          <X :size="14" />
        </button>
      </div>

      <!-- Tab: Import -->
      <div v-if="activeTab === 'import'" class="tab-content">
        <div class="form-group mb-3">
          <label class="label-row">
            <span>粘贴 .env 或 export 格式文本</span>
            <span class="hint">支持常规 KEY=VALUE 以及 export KEY="VAL" 格式</span>
          </label>
          <textarea
            v-model="importText"
            placeholder="# 示例：&#10;PORT=8080&#10;export OPENAI_API_KEY=sk-xxxx&#10;DATABASE_URL=postgres://..."
            rows="6"
            class="mono import-textarea"
            @input="handleParse"
          ></textarea>
        </div>

        <!-- Preview parsed table -->
        <div v-if="parsedList.length > 0" class="parsed-preview">
          <div class="preview-header">
            <span>识别到 {{ parsedList.length }} 个环境变量：</span>
            <span class="hint">勾选锁标将以 Keychain 密文存储</span>
          </div>

          <div class="parsed-table-scroll">
            <table class="preview-table">
              <thead>
                <tr>
                  <th style="width: 36px; text-align: center;">密文</th>
                  <th>变量名</th>
                  <th>预览值</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="item in parsedList" :key="item.key">
                  <td style="text-align: center;">
                    <input
                      type="checkbox"
                      :checked="secretSet.has(item.key)"
                      @change="toggleSecret(item.key)"
                      title="是否存入 macOS 钥匙串"
                    />
                  </td>
                  <td class="mono font-bold">{{ item.key }}</td>
                  <td class="mono text-muted truncate">
                    {{ secretSet.has(item.key) ? '•••••••• (Keychain)' : item.value }}
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <div class="modal-footer">
          <button class="btn" @click="$emit('close')">取消</button>
          <button
            class="btn btn-primary"
            :disabled="parsedList.length === 0"
            @click="handleDoImport"
          >
            导入 {{ parsedList.length }} 个变量到当前环境
          </button>
        </div>
      </div>

      <!-- Tab: Export -->
      <div v-else class="tab-content">
        <div class="form-group mb-3">
          <label class="label-row">
            <span>当前环境完整 .env 导出文本</span>
            <span class="hint">已自动解密 Keychain 密钥并合并</span>
          </label>
          <textarea
            :value="exportText"
            readonly
            rows="10"
            class="mono export-textarea"
          ></textarea>
        </div>

        <div class="modal-footer">
          <button class="btn" @click="$emit('close')">关闭</button>
          <button class="btn" @click="handleDownload">
            <Download :size="13" />
            <span>下载 .env 文件</span>
          </button>
          <button class="btn btn-primary" @click="handleCopyExport">
            <Check v-if="copiedExport" :size="13" class="text-success" />
            <Copy v-else :size="13" />
            <span>{{ copiedExport ? '已复制到剪贴板' : '复制全部内容' }}</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from "vue";
import {
  UploadCloud,
  DownloadCloud,
  Download,
  Copy,
  Check,
  X,
} from "lucide-vue-next";
import { api } from "../api";
import { useEnvStore } from "../store";

const emit = defineEmits<{
  (e: "close"): void;
}>();

const { state, importParsedPairs } = useEnvStore();

const activeTab = ref<"import" | "export">("import");
const importText = ref("");
const parsedList = ref<{ key: string; value: string }[]>([]);
const secretSet = reactive(new Set<string>());

const exportText = ref("");
const copiedExport = ref(false);

const SECRET_KEYWORDS = ["KEY", "TOKEN", "SECRET", "PASS", "AUTH", "PRIVATE", "CREDENTIAL"];

async function handleParse() {
  if (!importText.value.trim()) {
    parsedList.value = [];
    secretSet.clear();
    return;
  }

  try {
    const pairs = await api.parseEnvText(importText.value);
    parsedList.value = pairs.map(([k, v]) => ({ key: k, value: v }));

    secretSet.clear();
    for (const item of parsedList.value) {
      const upper = item.key.toUpperCase();
      if (SECRET_KEYWORDS.some(kw => upper.includes(kw))) {
        secretSet.add(item.key);
      }
    }
  } catch (err) {
    console.error("Parse error", err);
  }
}

function toggleSecret(key: string) {
  if (secretSet.has(key)) {
    secretSet.delete(key);
  } else {
    secretSet.add(key);
  }
}

async function handleDoImport() {
  if (parsedList.value.length === 0) return;
  const pairs: [string, string][] = parsedList.value.map(p => [p.key, p.value]);
  await importParsedPairs(pairs, secretSet);
  emit("close");
}

async function handleSwitchExport() {
  activeTab.value = "export";
  if (!state.selectedProfileId) return;
  try {
    exportText.value = await api.exportProfileDotenv(state.selectedProfileId);
  } catch (err: any) {
    exportText.value = "# 导出失败: " + err;
  }
}

async function handleCopyExport() {
  await navigator.clipboard.writeText(exportText.value);
  copiedExport.value = true;
  setTimeout(() => {
    copiedExport.value = false;
  }, 1500);
}

function handleDownload() {
  const blob = new Blob([exportText.value], { type: "text/plain;charset=utf-8" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = ".env";
  a.click();
  URL.revokeObjectURL(url);
}
</script>

<style scoped>
.modal-lg {
  max-width: 640px;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 14px;
}

.tab-header {
  display: flex;
  align-items: center;
  gap: 8px;
}

.tab-btn {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--text-muted);
  font-size: 13px;
  font-weight: 500;
}

.tab-btn:hover {
  color: var(--text-main);
  background: rgba(255, 255, 255, 0.05);
}

.tab-btn.active {
  background: var(--bg-card);
  color: var(--accent-primary);
  border: 1px solid var(--border-subtle);
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

.label-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 11px;
  font-weight: 600;
  color: var(--text-secondary);
  margin-bottom: 6px;
}

.hint {
  font-size: 10px;
  color: var(--text-dim);
}

.import-textarea, .export-textarea {
  width: 100%;
  resize: vertical;
  font-size: 11px;
  line-height: 1.6;
}

.parsed-preview {
  margin-top: 10px;
  background: var(--bg-app);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-sm);
  overflow: hidden;
}

.preview-header {
  display: flex;
  justify-content: space-between;
  padding: 6px 10px;
  background: rgba(255, 255, 255, 0.02);
  border-bottom: 1px solid var(--border-subtle);
  font-size: 11px;
  color: var(--text-secondary);
}

.parsed-table-scroll {
  max-height: 150px;
  overflow-y: auto;
}

.preview-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 11px;
}

.preview-table th {
  padding: 4px 8px;
  color: var(--text-dim);
  text-align: left;
  background: rgba(0, 0, 0, 0.2);
}

.preview-table td {
  padding: 5px 8px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.03);
}

.truncate {
  max-width: 250px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.modal-footer {
  margin-top: 16px;
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

.mb-3 {
  margin-bottom: 12px;
}

.text-success {
  color: var(--accent-success);
}
</style>
