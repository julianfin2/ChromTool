<script setup lang="ts">
import type { ProfileSortKey, ProfileSummary } from "../../types/browser";
import { profileAvatarSrc } from "../../utils/icons";

defineProps<{
  profiles: ProfileSummary[];
  sortKey: ProfileSortKey;
  openProfileError: string;
  browserId: string;
  browserFamilyId: string | null;
  isOpeningProfile: (browserId: string, profileId: string) => boolean;
}>();

const emit = defineEmits<{
  "update:sortKey": [value: ProfileSortKey];
  openProfile: [browserId: string, profileId: string];
}>();
</script>

<template>
  <section class="table-section">
    <div v-if="openProfileError" class="inline-error">
      {{ openProfileError }}
    </div>

    <div v-if="profiles.length" class="data-table">
      <div class="data-table-header profiles-grid">
        <div class="header-cell icon-cell">头像</div>
        <button class="header-cell sortable" :class="{ active: sortKey === 'name' }" type="button" @click="emit('update:sortKey', 'name')">名称</button>
        <button class="header-cell sortable" :class="{ active: sortKey === 'email' }" type="button" @click="emit('update:sortKey', 'email')">邮箱</button>
        <button class="header-cell sortable" :class="{ active: sortKey === 'id' }" type="button" @click="emit('update:sortKey', 'id')">资料 ID</button>
        <div class="header-cell actions-cell">操作</div>
      </div>
      <div class="data-table-body styled-scrollbar">
        <article v-for="profile in profiles" :key="profile.id" class="data-table-row profiles-grid">
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
          </div>
          <div class="row-cell muted-cell" :title="profile.email ?? undefined">
            {{ profile.email || "" }}
          </div>
          <div class="row-cell">
            <span class="badge neutral">{{ profile.id }}</span>
          </div>
          <div class="row-cell actions-cell">
            <button
              class="card-action-button"
              :disabled="isOpeningProfile(browserId, profile.id)"
              type="button"
              @click="emit('openProfile', browserId, profile.id)"
            >
              {{ isOpeningProfile(browserId, profile.id) ? "打开中..." : "打开" }}
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

.profiles-grid {
  display: grid;
  grid-template-columns: 64px minmax(180px, 1.2fr) minmax(180px, 1fr) 132px 110px;
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

.header-cell {
  color: var(--muted);
  font-size: 0.81rem;
  font-weight: 700;
  letter-spacing: 0.02em;
}

.header-cell.sortable {
  padding: 0;
  text-align: left;
  background: transparent;
  cursor: pointer;
}

.header-cell.sortable.active {
  color: var(--text);
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

.row-cell {
  min-width: 0;
}

.primary-cell strong {
  display: block;
  font-size: 0.93rem;
  line-height: 1.3;
}

.muted-cell {
  color: var(--muted);
  font-size: 0.86rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.actions-cell {
  display: flex;
  justify-content: flex-end;
}

.icon-cell {
  padding-left: 4px;
}

@media (max-width: 900px) {
  .profiles-grid {
    grid-template-columns: 56px minmax(140px, 1fr) minmax(140px, 1fr) 110px 96px;
  }
}

@media (max-width: 720px) {
  .profiles-grid {
    grid-template-columns: 56px minmax(0, 1fr) 96px;
  }

  .profiles-grid > :nth-child(3),
  .profiles-grid > :nth-child(4) {
    display: none;
  }
}
</style>
