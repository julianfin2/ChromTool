<script setup lang="ts">
import type { CleanupHistoryResult, ProfileSummary } from "../../types/browser";

defineProps<{
  mode: "confirm" | "result";
  title: string;
  profiles: ProfileSummary[];
  results: CleanupHistoryResult[];
  busy?: boolean;
  generalError?: string;
}>();

const emit = defineEmits<{
  close: [];
  confirm: [];
}>();
</script>

<template>
  <div class="modal-backdrop" @click.self="emit('close')">
    <section class="modal-card">
      <div class="modal-header">
        <h3>{{ title }}</h3>
        <button class="secondary-button modal-close-button" type="button" @click="emit('close')">
          关闭
        </button>
      </div>

      <template v-if="mode === 'confirm'">
        <p class="modal-copy">
          将删除所选资料中的 <code>History</code>、<code>Top Sites</code>、<code>Visited Links</code>、<code>Shortcuts</code>，
          并清空 <code>Sessions</code> 目录中的所有文件。
        </p>

        <div class="profile-list styled-scrollbar">
          <article v-for="profile in profiles" :key="profile.id" class="profile-item">
            <strong>{{ profile.name }}</strong>
            <span>{{ profile.id }}</span>
          </article>
        </div>

        <div class="modal-actions">
          <button class="secondary-button" type="button" @click="emit('close')">取消</button>
          <button class="danger-button" type="button" :disabled="busy" @click="emit('confirm')">
            {{ busy ? "清理中..." : "确认清理" }}
          </button>
        </div>
      </template>

      <template v-else>
        <p v-if="generalError" class="result-banner error">{{ generalError }}</p>

        <div class="result-list styled-scrollbar">
          <article
            v-for="result in results"
            :key="result.profileId"
            class="result-card"
            :class="{ error: result.error }"
          >
            <strong>{{ result.profileId }}</strong>
            <p v-if="result.error">{{ result.error }}</p>
            <p v-else-if="result.deletedFiles.length">
              已删除：{{ result.deletedFiles.join("、") }}
            </p>
            <p v-else>
              没有删除任何文件。
            </p>
            <p v-if="result.skippedFiles.length" class="muted-line">
              已跳过（不存在）：{{ result.skippedFiles.join("、") }}
            </p>
          </article>
        </div>

        <div class="modal-actions">
          <button class="primary-button" type="button" @click="emit('close')">关闭</button>
        </div>
      </template>
    </section>
  </div>
</template>

<style scoped>
.modal-backdrop {
  position: fixed;
  inset: 0;
  z-index: 50;
  display: grid;
  place-items: center;
  padding: 24px;
  background: rgba(15, 23, 42, 0.26);
  backdrop-filter: blur(6px);
  -webkit-backdrop-filter: blur(6px);
}

.modal-card {
  width: min(640px, 100%);
  max-height: min(76vh, 820px);
  display: flex;
  flex-direction: column;
  gap: 14px;
  padding: 16px;
  border: 1px solid var(--panel-border);
  border-radius: 22px;
  background: rgba(255, 255, 255, 0.96);
  box-shadow: 0 28px 70px rgba(15, 23, 42, 0.18);
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.modal-header h3 {
  margin: 0;
  font-weight: 600;
  letter-spacing: -0.03em;
}

.modal-copy {
  margin: 0;
  color: var(--muted);
  line-height: 1.55;
}

.profile-list,
.result-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
  min-height: 0;
  overflow: auto;
}

.profile-item,
.result-card {
  display: grid;
  gap: 4px;
  padding: 12px 14px;
  border: 1px solid rgba(148, 163, 184, 0.18);
  border-radius: 16px;
  background: rgba(248, 250, 252, 0.84);
}

.profile-item span,
.muted-line {
  color: var(--muted);
  font-size: 0.85rem;
}

.result-card p {
  margin: 0;
  color: var(--text);
  line-height: 1.5;
}

.result-card.error {
  border-color: rgba(239, 68, 68, 0.18);
  background: rgba(254, 242, 242, 0.96);
}

.result-banner {
  margin: 0;
  padding: 12px 14px;
  border-radius: 14px;
  font-size: 0.9rem;
}

.result-banner.error {
  background: rgba(254, 242, 242, 0.96);
  color: #b42318;
  border: 1px solid rgba(239, 68, 68, 0.18);
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

code {
  padding: 1px 5px;
  border-radius: 7px;
  background: rgba(226, 232, 240, 0.72);
  color: var(--text);
}

@media (max-width: 720px) {
  .modal-backdrop {
    padding: 12px;
  }

  .modal-actions {
    flex-direction: column-reverse;
  }
}
</style>
