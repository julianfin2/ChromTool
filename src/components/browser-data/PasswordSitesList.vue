<script setup lang="ts">
import type { PasswordSiteSortKey, PasswordSiteSummary, SortDirection } from "../../types/browser";

const props = defineProps<{
  passwordSites: PasswordSiteSummary[];
  sortKey: PasswordSiteSortKey;
  sortDirection: SortDirection;
  loaded: boolean;
  loading: boolean;
  error: string;
}>();

const emit = defineEmits<{
  "update:sortKey": [value: PasswordSiteSortKey];
  showProfiles: [url: string];
  load: [];
}>();

function sortIndicator(sortKey: PasswordSiteSortKey) {
  if (props.sortKey !== sortKey) return "↕";
  return props.sortDirection === "asc" ? "↑" : "↓";
}
</script>

<template>
  <section class="table-section">
    <div class="password-actions">
      <div class="password-actions-copy">
        <h3>按需读取已保存登录站点</h3>
        <p>为减少误报风险，这部分数据不会在应用启动时自动扫描。</p>
      </div>
      <button class="load-button" type="button" :disabled="loading" @click="emit('load')">
        {{ loading ? "读取中..." : loaded ? "重新读取" : "手动读取" }}
      </button>
    </div>

    <p v-if="error" class="error-text">{{ error }}</p>

    <div v-if="passwordSites.length" class="data-table">
      <div class="data-table-header passwords-grid">
        <button class="header-cell sortable" :class="{ active: sortKey === 'domain' }" type="button" @click="emit('update:sortKey', 'domain')">
          <span>域名</span>
          <span class="sort-indicator" :class="{ active: sortKey === 'domain' }">{{ sortIndicator("domain") }}</span>
        </button>
        <button class="header-cell sortable" :class="{ active: sortKey === 'url' }" type="button" @click="emit('update:sortKey', 'url')">
          <span>URL</span>
          <span class="sort-indicator" :class="{ active: sortKey === 'url' }">{{ sortIndicator("url") }}</span>
        </button>
        <div class="header-cell actions-cell">关联资料</div>
      </div>
      <div class="data-table-body styled-scrollbar">
        <article
          v-for="passwordSite in passwordSites"
          :key="passwordSite.url"
          class="data-table-row passwords-grid"
        >
          <div class="row-cell primary-cell">
            <strong>{{ passwordSite.domain }}</strong>
          </div>
          <div class="row-cell muted-cell" :title="passwordSite.url">{{ passwordSite.url }}</div>
          <div class="row-cell actions-cell">
            <button class="disclosure-button" type="button" @click="emit('showProfiles', passwordSite.url)">
              <span>查看</span>
              <span class="badge neutral">{{ passwordSite.profileIds.length }}</span>
            </button>
          </div>
        </article>
      </div>
    </div>
    <div v-else class="empty-card">
      <p v-if="loaded">这个浏览器没有检测到任何已保存登录站点。</p>
      <p v-else>点击上方按钮后才会读取当前浏览器的已保存登录站点。</p>
    </div>
  </section>
</template>

<style scoped>
.table-section {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 0;
  height: 100%;
  min-height: 0;
}

.password-actions {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 18px 20px;
  border: 1px solid rgba(148, 163, 184, 0.18);
  border-radius: 22px;
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.94), rgba(240, 249, 255, 0.88));
}

.password-actions-copy h3 {
  margin: 0 0 6px;
  font-size: 1rem;
}

.password-actions-copy p {
  margin: 0;
  color: var(--muted);
  font-size: 0.9rem;
}

.load-button {
  flex-shrink: 0;
  min-width: 112px;
  padding: 10px 16px;
  border-radius: 14px;
  background: var(--accent);
  color: #fff;
  font-weight: 700;
  cursor: pointer;
}

.load-button:disabled {
  cursor: progress;
  opacity: 0.7;
}

.error-text {
  margin: 0;
  color: #b91c1c;
  font-size: 0.9rem;
}

.data-table {
  display: flex;
  flex-direction: column;
  height: 100%;
  min-height: 0;
  border: 1px solid rgba(148, 163, 184, 0.18);
  border-radius: 22px;
  background: var(--panel);
  overflow: hidden;
}

.data-table-body {
  min-height: 0;
  overflow: auto;
  scrollbar-gutter: stable;
}

.passwords-grid {
  display: grid;
  grid-template-columns: minmax(180px, 0.9fr) minmax(280px, 1.2fr) 154px;
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
  display: inline-flex;
  align-items: center;
  gap: 5px;
  padding: 0;
  text-align: left;
  background: transparent;
  cursor: pointer;
}

.header-cell.sortable.active {
  color: var(--text);
}

.sort-indicator {
  color: rgba(100, 116, 139, 0.42);
  font-size: 0.78rem;
  line-height: 1;
}

.sort-indicator.active {
  color: var(--accent);
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
  padding: 7px 10px;
  border-radius: 12px;
  background: rgba(241, 245, 249, 0.9);
  color: var(--badge-text);
  cursor: pointer;
}

.actions-cell {
  display: flex;
  justify-content: flex-end;
}

@media (max-width: 900px) {
  .passwords-grid {
    grid-template-columns: minmax(160px, 0.9fr) minmax(200px, 1fr) 148px;
  }
}

@media (max-width: 720px) {
  .password-actions {
    align-items: stretch;
    flex-direction: column;
  }

  .load-button {
    width: 100%;
  }

  .passwords-grid {
    grid-template-columns: minmax(0, 1fr) 132px;
  }

  .passwords-grid > :nth-child(2) {
    display: none;
  }
}
</style>
