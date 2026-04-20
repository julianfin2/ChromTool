<script setup lang="ts">
import { computed } from "vue";
import type { CleanupFileStatus, ProfileSummary } from "../../types/browser";
import { profileAvatarSrc } from "../../utils/icons";

const props = defineProps<{
  browserFamilyId: string | null;
  profiles: ProfileSummary[];
  selectedProfileIds: string[];
  cleanupBusy: boolean;
}>();

const emit = defineEmits<{
  toggleProfile: [profileId: string];
  toggleAllProfiles: [];
  cleanupSelected: [];
  cleanupProfile: [profileId: string];
}>();

const selectableProfiles = computed(() =>
  props.profiles.filter((profile) =>
    hasAnyHistoryFile([
      profile.historyCleanup.history,
      profile.historyCleanup.topSites,
      profile.historyCleanup.visitedLinks,
      profile.historyCleanup.shortcuts,
      profile.historyCleanup.sessions,
    ]),
  ),
);

const allSelected = computed(
  () =>
    selectableProfiles.value.length > 0 &&
    selectableProfiles.value.every((profile) =>
      props.selectedProfileIds.includes(profile.id),
    ),
);

function isSelected(profileId: string) {
  return props.selectedProfileIds.includes(profileId);
}

function isSelectable(profile: ProfileSummary) {
  return hasAnyHistoryFile([
    profile.historyCleanup.history,
    profile.historyCleanup.topSites,
    profile.historyCleanup.visitedLinks,
    profile.historyCleanup.shortcuts,
    profile.historyCleanup.sessions,
  ]);
}

function hasAnyHistoryFile(statuses: CleanupFileStatus[]) {
  return statuses.some((status) => status === "found");
}

function cleanupItems(profile: ProfileSummary) {
  const items = [
    { key: "history", label: "历史记录", status: profile.historyCleanup.history },
    { key: "top-sites", label: "热门站点", status: profile.historyCleanup.topSites },
    { key: "visited-links", label: "访问链接", status: profile.historyCleanup.visitedLinks },
    { key: "shortcuts", label: "快捷方式", status: profile.historyCleanup.shortcuts },
    { key: "sessions", label: "会话", status: profile.historyCleanup.sessions },
  ];

  return items.filter((item) => item.status === "found");
}
</script>

<template>
  <section class="table-section">
    <div v-if="profiles.length" class="data-table">
      <div class="history-toolbar">
        <label class="toolbar-checkbox" :class="{ disabled: !selectableProfiles.length }">
          <input
            type="checkbox"
            class="native-checkbox"
            :checked="allSelected"
            :disabled="!selectableProfiles.length || cleanupBusy"
            @change="emit('toggleAllProfiles')"
          />
          <span class="custom-checkbox" :class="{ checked: allSelected }" aria-hidden="true">
            <svg viewBox="0 0 16 16">
              <path d="M3.5 8.2L6.4 11.1L12.5 4.9" />
            </svg>
          </span>
          <span>全选</span>
        </label>
        <button
          class="danger-button toolbar-danger-button"
          type="button"
          :disabled="!selectedProfileIds.length || cleanupBusy"
          @click="emit('cleanupSelected')"
        >
          {{ cleanupBusy ? "清理中..." : `清理所选（${selectedProfileIds.length}）` }}
        </button>
      </div>

      <div class="data-table-header history-grid">
        <div class="header-cell checkbox-cell">选择</div>
        <div class="header-cell icon-cell">头像</div>
        <div class="header-cell">资料</div>
        <div class="header-cell">可清理项</div>
        <div class="header-cell actions-cell">操作</div>
      </div>
      <div class="data-table-body styled-scrollbar">
        <article v-for="profile in profiles" :key="profile.id" class="data-table-row history-grid">
          <div class="row-cell checkbox-cell">
            <label class="table-checkbox" :class="{ disabled: !isSelectable(profile) || cleanupBusy }">
              <input
                type="checkbox"
                class="native-checkbox"
                :checked="isSelected(profile.id)"
                :disabled="!isSelectable(profile) || cleanupBusy"
                @change="emit('toggleProfile', profile.id)"
              />
              <span class="custom-checkbox" :class="{ checked: isSelected(profile.id) }" aria-hidden="true">
                <svg viewBox="0 0 16 16">
                  <path d="M3.5 8.2L6.4 11.1L12.5 4.9" />
                </svg>
              </span>
            </label>
          </div>
          <div class="profile-avatar table-avatar">
            <img
              v-if="profileAvatarSrc(profile, browserFamilyId)"
              :src="profileAvatarSrc(profile, browserFamilyId) ?? undefined"
              :alt="`${profile.name} avatar`"
            />
            <span v-else>{{ profile.avatarLabel }}</span>
          </div>
          <div class="row-cell primary-cell">
            <strong>{{ profile.name }}</strong>
            <span class="subtle-line">{{ profile.id }}</span>
          </div>
          <div class="row-cell cleanup-summary-cell">
            <div v-if="cleanupItems(profile).length" class="cleanup-tag-list">
              <span
                v-for="item in cleanupItems(profile)"
                :key="item.key"
                class="cleanup-tag"
              >
                {{ item.label }}
              </span>
            </div>
            <span v-else class="cleanup-empty">没有可清理项</span>
          </div>
          <div class="row-cell actions-cell">
            <button
              class="danger-button action-button"
              type="button"
              :disabled="!isSelectable(profile) || cleanupBusy"
              @click="emit('cleanupProfile', profile.id)"
            >
              清理
            </button>
          </div>
        </article>
      </div>
    </div>
    <div v-else class="empty-card">
      <p>这个浏览器没有找到任何用户资料目录。</p>
    </div>
  </section>
</template>

<style scoped>
.table-section {
  padding: 0;
  height: 100%;
  min-height: 0;
}

.history-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 10px 12px 8px;
  border-bottom: 1px solid rgba(148, 163, 184, 0.1);
}

.toolbar-checkbox {
  display: inline-flex;
  align-items: center;
  gap: 10px;
  color: var(--text);
  font-size: 0.88rem;
  cursor: pointer;
}

.toolbar-checkbox.disabled {
  opacity: 0.55;
  cursor: default;
}

.native-checkbox {
  position: absolute;
  opacity: 0;
  pointer-events: none;
}

.table-checkbox {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
}

.table-checkbox.disabled {
  cursor: default;
  opacity: 0.5;
}

.custom-checkbox {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  border: 1px solid rgba(148, 163, 184, 0.34);
  border-radius: 7px;
  background: linear-gradient(180deg, rgba(255, 255, 255, 0.98), rgba(241, 245, 249, 0.92));
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.78),
    0 4px 10px rgba(15, 23, 42, 0.06);
  transition:
    border-color 160ms ease,
    background 160ms ease,
    box-shadow 160ms ease,
    transform 160ms ease;
}

.custom-checkbox svg {
  width: 12px;
  height: 12px;
}

.custom-checkbox path {
  fill: none;
  stroke: #fff;
  stroke-width: 2.2;
  stroke-linecap: round;
  stroke-linejoin: round;
  opacity: 0;
  transition: opacity 160ms ease;
}

.toolbar-checkbox:hover .custom-checkbox,
.table-checkbox:hover .custom-checkbox {
  border-color: rgba(47, 111, 237, 0.38);
  transform: translateY(-1px);
}

.custom-checkbox.checked {
  border-color: rgba(47, 111, 237, 0.2);
  background: linear-gradient(135deg, #2f6fed, #5aa1f7);
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.22),
    0 8px 18px rgba(47, 111, 237, 0.22);
}

.custom-checkbox.checked path {
  opacity: 1;
}

.data-table {
  display: flex;
  flex-direction: column;
  height: 100%;
  min-height: 0;
  border: 1px solid rgba(148, 163, 184, 0.18);
  border-radius: 22px;
  background: var(--panel);
  box-shadow: var(--shadow);
  overflow: hidden;
}

.data-table-body {
  min-height: 0;
  overflow: auto;
  scrollbar-gutter: stable;
}

.history-grid {
  display: grid;
  grid-template-columns: 52px 56px minmax(200px, 0.95fr) minmax(260px, 1.4fr) 108px;
  gap: 12px;
  align-items: center;
}

.data-table-header {
  position: sticky;
  top: 0;
  z-index: 2;
  padding: 8px 24px 8px 12px;
  border-bottom: 1px solid rgba(148, 163, 184, 0.14);
  background: rgba(248, 250, 252, 0.94);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
}

.data-table-row {
  padding: 10px 12px;
  border-bottom: 1px solid rgba(148, 163, 184, 0.12);
}

.data-table-row:last-child {
  border-bottom: 0;
}

.data-table-row:hover {
  background: rgba(248, 250, 252, 0.65);
}

.header-cell {
  color: var(--muted);
  font-size: 0.81rem;
  font-weight: 700;
  letter-spacing: 0.02em;
}

.checkbox-cell {
  display: flex;
  justify-content: center;
}

.icon-cell {
  padding-left: 4px;
}

.row-cell {
  min-width: 0;
}

.profile-avatar {
  display: grid;
  place-items: center;
  flex-shrink: 0;
  background: linear-gradient(135deg, #dbeafe, #eff6ff);
  color: #1d4ed8;
  font-size: 0.96rem;
  font-weight: 700;
  overflow: hidden;
}

.table-avatar {
  width: 36px;
  height: 36px;
  border-radius: 999px;
}

.profile-avatar img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.primary-cell strong {
  display: block;
  font-size: 0.93rem;
  line-height: 1.3;
}

.cleanup-summary-cell {
  display: flex;
  align-items: center;
  min-height: 40px;
}

.subtle-line {
  display: block;
  margin-top: 3px;
  color: var(--muted-soft);
  font-size: 0.8rem;
}

.cleanup-tag-list {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 8px;
}

.cleanup-tag {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 5px 9px;
  border: 1px solid rgba(148, 163, 184, 0.18);
  border-radius: 999px;
  background: rgba(248, 250, 252, 0.88);
  color: #475569;
  font-size: 0.77rem;
  font-weight: 600;
  letter-spacing: 0.01em;
}

.cleanup-tag::before {
  content: "";
  width: 6px;
  height: 6px;
  border-radius: 999px;
  background: #60a5fa;
  opacity: 0.85;
}

.cleanup-empty {
  color: var(--muted);
  font-size: 0.85rem;
}

.actions-cell {
  display: flex;
  justify-content: flex-end;
}

.action-button {
  padding: 5px 10px;
  border-radius: 10px;
}

.toolbar-danger-button {
  padding: 6px 10px;
  border-radius: 10px;
}

@media (max-width: 900px) {
  .history-grid {
    grid-template-columns: 52px 56px minmax(160px, 0.9fr) minmax(220px, 1.2fr) 100px;
  }
}

@media (max-width: 720px) {
  .history-toolbar {
    flex-direction: column;
    align-items: stretch;
  }

  .history-grid {
    grid-template-columns: 52px minmax(0, 1fr) 108px;
  }

  .history-grid > :nth-child(2) {
    display: none;
  }

  .cleanup-summary-cell {
    padding-top: 2px;
    padding-bottom: 2px;
  }
}
</style>
