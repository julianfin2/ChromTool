<script setup lang="ts">
import { computed } from "vue";
import type { ExtensionSortKey, ExtensionSummary } from "../../types/browser";

const props = defineProps<{
  extensions: ExtensionSummary[];
  sortKey: ExtensionSortKey;
  extensionMonogram: (name: string) => string;
  selectedExtensionIds: string[];
  deleteBusy: boolean;
}>();

const emit = defineEmits<{
  "update:sortKey": [value: ExtensionSortKey];
  showProfiles: [extensionId: string];
  toggleExtension: [extensionId: string];
  toggleAllExtensions: [];
  deleteExtension: [extensionId: string];
  deleteSelected: [];
}>();

const allSelected = computed(
  () =>
    props.extensions.length > 0 &&
    props.extensions.every((extension) => props.selectedExtensionIds.includes(extension.id)),
);

function isSelected(extensionId: string) {
  return props.selectedExtensionIds.includes(extensionId);
}
</script>

<template>
  <section class="table-section">
    <div v-if="extensions.length" class="data-table">
      <div class="extensions-toolbar">
        <label class="toolbar-checkbox" :class="{ disabled: !extensions.length }">
          <input
            type="checkbox"
            class="native-checkbox"
            :checked="allSelected"
            :disabled="!extensions.length || deleteBusy"
            @change="emit('toggleAllExtensions')"
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
          :disabled="!selectedExtensionIds.length || deleteBusy"
          @click="emit('deleteSelected')"
        >
          {{ deleteBusy ? "删除中..." : `删除所选（${selectedExtensionIds.length}）` }}
        </button>
      </div>

      <div class="data-table-header extensions-grid">
        <div class="header-cell checkbox-cell">选择</div>
        <div class="header-cell icon-cell">图标</div>
        <button class="header-cell sortable" :class="{ active: sortKey === 'name' }" type="button" @click="emit('update:sortKey', 'name')">名称</button>
        <button class="header-cell sortable" :class="{ active: sortKey === 'id' }" type="button" @click="emit('update:sortKey', 'id')">插件 ID</button>
        <div class="header-cell actions-cell">操作</div>
      </div>
      <div class="data-table-body styled-scrollbar">
        <article v-for="extension in extensions" :key="extension.id" class="data-table-row extensions-grid">
          <div class="row-cell checkbox-cell">
            <label class="table-checkbox" :class="{ disabled: deleteBusy }">
              <input
                type="checkbox"
                class="native-checkbox"
                :checked="isSelected(extension.id)"
                :disabled="deleteBusy"
                @change="emit('toggleExtension', extension.id)"
              />
              <span class="custom-checkbox" :class="{ checked: isSelected(extension.id) }" aria-hidden="true">
                <svg viewBox="0 0 16 16">
                  <path d="M3.5 8.2L6.4 11.1L12.5 4.9" />
                </svg>
              </span>
            </label>
          </div>
          <div class="extension-icon table-icon" :class="{ filled: Boolean(extension.iconDataUrl) }">
            <img v-if="extension.iconDataUrl" :src="extension.iconDataUrl" :alt="`${extension.name} icon`" />
            <span v-else>{{ extensionMonogram(extension.name) }}</span>
          </div>
          <div class="row-cell primary-cell">
            <strong>{{ extension.name }}</strong>
          </div>
          <div class="row-cell muted-cell" :title="extension.id">{{ extension.id }}</div>
          <div class="row-cell actions-cell">
            <button class="disclosure-button" type="button" @click="emit('showProfiles', extension.id)">
              <span>查看</span>
              <span class="badge neutral">{{ extension.profileIds.length }}</span>
            </button>
            <button
              class="danger-button inline-danger-button"
              type="button"
              :disabled="deleteBusy"
              @click="emit('deleteExtension', extension.id)"
            >
              删除
            </button>
          </div>
        </article>
      </div>
    </div>
    <div v-else class="empty-card">
      <p>这个浏览器没有扫描到任何插件。</p>
    </div>
  </section>
</template>

<style scoped>
.table-section {
  padding: 0;
  height: 100%;
  min-height: 0;
}

.extensions-toolbar {
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

.extensions-grid {
  display: grid;
  grid-template-columns: 52px 60px minmax(180px, 1.1fr) minmax(220px, 1fr) 250px;
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

.extension-icon {
  display: grid;
  place-items: center;
  flex-shrink: 0;
  background: linear-gradient(135deg, #e2e8f0, #f8fafc);
  color: #475569;
  font-weight: 700;
  overflow: hidden;
}

.extension-icon.filled {
  background: transparent;
}

.table-icon {
  width: 36px;
  height: 36px;
  border-radius: 12px;
}

.extension-icon img {
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
  font-size: 0.87rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.disclosure-button {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
  width: fit-content;
  min-width: 120px;
  padding: 5px 10px;
  border-radius: 10px;
  background: rgba(241, 245, 249, 0.9);
  color: var(--badge-text);
  font-size: 0.8rem;
  cursor: pointer;
}

.disclosure-button .badge {
  min-width: 24px;
  padding: 4px 8px;
  font-size: 0.76rem;
}

.actions-cell {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

.checkbox-cell {
  display: flex;
  justify-content: center;
}

.inline-danger-button {
  padding: 5px 10px;
  border-radius: 10px;
  font-size: 0.8rem;
}

.toolbar-danger-button {
  padding: 6px 10px;
  border-radius: 10px;
}

.icon-cell {
  padding-left: 4px;
}

@media (max-width: 900px) {
  .extensions-grid {
    grid-template-columns: 52px 56px minmax(160px, 1fr) minmax(160px, 1fr) 220px;
  }
}

@media (max-width: 720px) {
  .extensions-toolbar {
    flex-direction: column;
    align-items: stretch;
  }

  .extensions-grid {
    grid-template-columns: 52px 56px minmax(0, 1fr) 132px;
  }

  .extensions-grid > :nth-child(4),
  .extensions-grid > :nth-child(5) {
    display: none;
  }
}
</style>
