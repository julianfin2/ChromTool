<script setup lang="ts">
import { computed, ref } from "vue";
import type {
  AssociatedProfileSortKey,
  AssociatedProfileSummary,
  BookmarkAssociatedProfileSummary,
  ExtensionAssociatedProfileSummary,
} from "../../types/browser";
import { profileAvatarSrc } from "../../utils/icons";
import { sortAssociatedProfiles } from "../../utils/sort";

type ModalProfile =
  | AssociatedProfileSummary
  | BookmarkAssociatedProfileSummary
  | ExtensionAssociatedProfileSummary;

const props = defineProps<{
  title: string;
  profiles: ModalProfile[];
  browserId: string;
  browserFamilyId: string | null;
  isBookmark: boolean;
  isExtension?: boolean;
  selectedProfileIds?: string[];
  deleteBusy?: boolean;
  isOpeningProfile: (browserId: string, profileId: string) => boolean;
}>();

const emit = defineEmits<{
  close: [];
  openProfile: [browserId: string, profileId: string];
  toggleProfileSelection: [profileId: string];
  toggleAllProfileSelection: [];
  deleteProfile: [profileId: string];
  deleteSelectedProfiles: [];
}>();

const sortKey = ref<AssociatedProfileSortKey>("id");
const sortedProfiles = computed(() => sortAssociatedProfiles(props.profiles, sortKey.value));
const selectedProfileIds = computed(() => props.selectedProfileIds ?? []);
const allSelected = computed(
  () =>
    sortedProfiles.value.length > 0 &&
    sortedProfiles.value.every((profile) => selectedProfileIds.value.includes(profile.id)),
);
const isSelectableMode = computed(() => props.isExtension || props.isBookmark);

function hasBookmarkPath(profile: ModalProfile): profile is BookmarkAssociatedProfileSummary {
  return "bookmarkPath" in profile;
}

function hasInstallSource(profile: ModalProfile): profile is ExtensionAssociatedProfileSummary {
  return "installSource" in profile;
}

function isSelected(profileId: string) {
  return selectedProfileIds.value.includes(profileId);
}
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

      <div class="modal-table">
        <div v-if="isSelectableMode" class="modal-toolbar">
          <label class="toolbar-checkbox" :class="{ disabled: !sortedProfiles.length }">
            <input
              type="checkbox"
              class="native-checkbox"
              :checked="allSelected"
              :disabled="!sortedProfiles.length || deleteBusy"
              @change="emit('toggleAllProfileSelection')"
            />
            <span class="custom-checkbox" :class="{ checked: allSelected }" aria-hidden="true">
              <svg viewBox="0 0 16 16">
                <path d="M3.5 8.2L6.4 11.1L12.5 4.9" />
              </svg>
            </span>
            <span>全选</span>
          </label>
          <button
            class="danger-button"
            type="button"
            :disabled="!selectedProfileIds.length || deleteBusy"
            @click="emit('deleteSelectedProfiles')"
          >
            {{ deleteBusy ? "删除中..." : `删除所选（${selectedProfileIds.length}）` }}
          </button>
        </div>

        <div class="modal-table-header modal-grid" :class="{ bookmark: isBookmark, extension: isExtension }">
          <div v-if="isSelectableMode" class="header-cell checkbox-cell">选择</div>
          <div class="header-cell icon-cell">头像</div>
          <button class="header-cell sortable" :class="{ active: sortKey === 'name' }" type="button" @click="sortKey = 'name'">名称</button>
          <button
            v-if="!isExtension && !isBookmark"
            class="header-cell sortable"
            :class="{ active: sortKey === 'id' }"
            type="button"
            @click="sortKey = 'id'"
          >
            资料 ID
          </button>
          <div v-if="isExtension" class="header-cell">来源</div>
          <div v-if="isBookmark" class="header-cell">书签路径</div>
          <div class="header-cell actions-cell">操作</div>
        </div>
        <div class="modal-table-body styled-scrollbar">
          <article
            v-for="profile in sortedProfiles"
            :key="profile.id"
            class="modal-table-row modal-grid"
            :class="{ bookmark: isBookmark, extension: isExtension }"
          >
            <div v-if="isSelectableMode" class="row-cell checkbox-cell">
              <label class="table-checkbox" :class="{ disabled: deleteBusy }">
                <input
                  type="checkbox"
                  class="native-checkbox"
                  :checked="isSelected(profile.id)"
                  :disabled="deleteBusy"
                  @change="emit('toggleProfileSelection', profile.id)"
                />
                <span class="custom-checkbox" :class="{ checked: isSelected(profile.id) }" aria-hidden="true">
                  <svg viewBox="0 0 16 16">
                    <path d="M3.5 8.2L6.4 11.1L12.5 4.9" />
                  </svg>
                </span>
              </label>
            </div>
            <div class="modal-profile-avatar">
              <img
                v-if="profileAvatarSrc(profile, browserFamilyId)"
                :src="profileAvatarSrc(profile, browserFamilyId) ?? undefined"
                :alt="`${profile.name} avatar`"
              />
              <span v-else>{{ profile.avatarLabel }}</span>
            </div>
            <div class="row-cell primary-cell">
              <strong>{{ profile.name }}</strong>
              <span v-if="isExtension || isBookmark" class="subtle-line">{{ profile.id }}</span>
            </div>
            <div v-if="!isExtension && !isBookmark" class="row-cell">
              <span class="badge neutral">{{ profile.id }}</span>
            </div>
            <div v-if="isExtension && hasInstallSource(profile)" class="row-cell muted-cell">
              {{ profile.installSource === "store" ? "商店安装" : "外部安装" }}
            </div>
            <div
              v-if="isBookmark && hasBookmarkPath(profile)"
              class="row-cell muted-cell"
              :title="profile.bookmarkPath"
            >
              {{ profile.bookmarkPath }}
            </div>
            <div class="row-cell actions-cell">
              <button
                class="card-action-button"
                type="button"
                :disabled="isOpeningProfile(browserId, profile.id)"
                @click="emit('openProfile', browserId, profile.id)"
              >
                {{ isOpeningProfile(browserId, profile.id) ? "打开中..." : "打开" }}
              </button>
              <button
                v-if="isSelectableMode"
                class="danger-button inline-danger-button"
                type="button"
                :disabled="deleteBusy"
                @click="emit('deleteProfile', profile.id)"
              >
                删除
              </button>
            </div>
          </article>
        </div>
      </div>
    </section>
  </div>
</template>

<style scoped>
.modal-backdrop {
  position: fixed;
  inset: 0;
  z-index: 40;
  display: grid;
  place-items: center;
  padding: 24px;
  background: rgba(15, 23, 42, 0.26);
  backdrop-filter: blur(6px);
  -webkit-backdrop-filter: blur(6px);
}

.modal-card {
  width: min(760px, 100%);
  max-height: min(78vh, 820px);
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
  min-width: 0;
}

.modal-header h3 {
  margin: 0;
  font-weight: 600;
  letter-spacing: -0.03em;
  min-width: 0;
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.modal-close-button {
  flex-shrink: 0;
}

.modal-table {
  display: flex;
  flex-direction: column;
  min-height: 0;
  border: 1px solid rgba(148, 163, 184, 0.18);
  border-radius: 18px;
  background: var(--panel-strong);
  overflow: hidden;
}

.modal-grid {
  display: grid;
  grid-template-columns: 56px minmax(180px, 1fr) 132px 110px;
  gap: 12px;
  align-items: center;
}

.modal-grid.bookmark {
  grid-template-columns: 52px 56px minmax(180px, 0.78fr) minmax(180px, 1fr) 168px;
}

.modal-table-header.modal-grid.extension,
.modal-table-row.modal-grid.extension {
  grid-template-columns: 52px 56px minmax(180px, 0.78fr) minmax(180px, 1fr) 168px;
}

.modal-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 10px 12px 8px;
  border-bottom: 1px solid rgba(148, 163, 184, 0.1);
}

.modal-table-header {
  padding: 8px 24px 8px 12px;
  border-bottom: 1px solid rgba(148, 163, 184, 0.14);
  background: rgba(248, 250, 252, 0.94);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
}

.modal-table-body {
  min-height: 0;
  overflow: auto;
  scrollbar-gutter: stable;
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

.modal-table-row {
  padding: 10px 12px;
  border-bottom: 1px solid rgba(148, 163, 184, 0.12);
}

.modal-table-row:last-child {
  border-bottom: 0;
}

.modal-table-row:hover {
  background: rgba(248, 250, 252, 0.65);
}

.modal-profile-avatar {
  display: grid;
  place-items: center;
  flex-shrink: 0;
  width: 36px;
  height: 36px;
  border-radius: 999px;
  background: linear-gradient(135deg, #dbeafe, #eff6ff);
  color: #1d4ed8;
  font-size: 0.96rem;
  font-weight: 700;
  overflow: hidden;
}

.modal-profile-avatar img {
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

.subtle-line {
  display: block;
  margin-top: 3px;
  color: var(--muted-soft);
  font-size: 0.8rem;
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
  gap: 8px;
}

.icon-cell {
  padding-left: 4px;
}

.checkbox-cell {
  display: flex;
  justify-content: center;
}

.inline-danger-button {
  padding: 6px 10px;
  border-radius: 10px;
  font-size: 0.8rem;
}

@media (max-width: 720px) {
  .modal-backdrop {
    padding: 12px;
  }

  .modal-grid,
  .modal-grid.bookmark,
  .modal-table-header.modal-grid.extension,
  .modal-table-row.modal-grid.extension {
    grid-template-columns: 56px minmax(0, 1fr) 120px;
  }

  .modal-grid > :nth-child(4),
  .modal-grid.bookmark > :nth-child(4),
  .modal-table-header.modal-grid.extension > :nth-child(4),
  .modal-table-row.modal-grid.extension > :nth-child(4) {
    display: none;
  }
}
</style>
