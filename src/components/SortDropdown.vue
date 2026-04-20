<script setup lang="ts">
import { computed, onBeforeUnmount, ref } from "vue";

type Option = {
  label: string;
  value: string;
  iconSrc?: string | null;
};

const props = defineProps<{
  label?: string;
  modelValue: string;
  options: Option[];
}>();

const emit = defineEmits<{
  "update:modelValue": [value: string];
}>();

const open = ref(false);
const root = ref<HTMLElement | null>(null);

const selectedLabel = computed(
  () =>
    props.options.find((option) => option.value === props.modelValue)?.label ??
    props.options[0]?.label ??
    "",
);

function toggle() {
  open.value = !open.value;
}

function select(value: string) {
  emit("update:modelValue", value);
  open.value = false;
}

function handleDocumentPointer(event: PointerEvent) {
  if (!root.value) return;
  const target = event.target;
  if (target instanceof Node && !root.value.contains(target)) {
    open.value = false;
  }
}

if (typeof window !== "undefined") {
  window.addEventListener("pointerdown", handleDocumentPointer);
}

onBeforeUnmount(() => {
  if (typeof window !== "undefined") {
    window.removeEventListener("pointerdown", handleDocumentPointer);
  }
});
</script>

<template>
  <div ref="root" class="sort-dropdown">
    <span v-if="label" class="sort-dropdown-label">{{ label }}</span>
    <button
      class="sort-dropdown-trigger"
      :class="{ open }"
      type="button"
      @click="toggle"
    >
      <span class="sort-dropdown-trigger-label">
        <img
          v-if="options.find((option) => option.value === modelValue)?.iconSrc"
          class="sort-dropdown-option-icon"
          :src="options.find((option) => option.value === modelValue)?.iconSrc ?? undefined"
          :alt="selectedLabel"
        />
        <span>{{ selectedLabel }}</span>
      </span>
      <span class="sort-dropdown-caret" aria-hidden="true"></span>
    </button>
    <div v-if="open" class="sort-dropdown-menu">
      <button
        v-for="option in options"
        :key="option.value"
        class="sort-dropdown-option"
        :class="{ active: option.value === modelValue }"
        type="button"
        @click="select(option.value)"
      >
        <span class="sort-dropdown-option-content">
          <img
            v-if="option.iconSrc"
            class="sort-dropdown-option-icon"
            :src="option.iconSrc"
            :alt="option.label"
          />
          <span>{{ option.label }}</span>
        </span>
      </button>
    </div>
  </div>
</template>

<style scoped>
.sort-dropdown {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  position: relative;
}

.sort-dropdown-label {
  color: var(--muted);
  font-size: 0.84rem;
}

.sort-dropdown-trigger {
  display: inline-flex;
  align-items: center;
  justify-content: space-between;
  gap: 14px;
  min-width: 152px;
  padding: 7px 10px;
  border: 1px solid rgba(148, 163, 184, 0.26);
  border-radius: 10px;
  background: linear-gradient(180deg, rgba(255, 255, 255, 0.96), rgba(241, 245, 249, 0.92));
  color: var(--text);
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.7);
  cursor: pointer;
  transition:
    border-color 160ms ease,
    box-shadow 160ms ease,
    background 160ms ease;
}

.sort-dropdown-trigger-label,
.sort-dropdown-option-content {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  min-width: 0;
}

.sort-dropdown-option-icon {
  width: 18px;
  height: 18px;
  flex-shrink: 0;
  object-fit: contain;
}

.sort-dropdown-trigger:hover {
  border-color: rgba(100, 116, 139, 0.36);
}

.sort-dropdown-trigger.open {
  border-color: rgba(47, 111, 237, 0.42);
  box-shadow:
    0 0 0 3px rgba(47, 111, 237, 0.12),
    inset 0 1px 0 rgba(255, 255, 255, 0.8);
}

.sort-dropdown-caret {
  width: 10px;
  height: 10px;
  flex-shrink: 0;
  border-right: 1.8px solid #64748b;
  border-bottom: 1.8px solid #64748b;
  transform: rotate(45deg) translateY(-1px);
  transition: transform 160ms ease;
}

.sort-dropdown-trigger.open .sort-dropdown-caret {
  transform: rotate(-135deg) translate(-2px, -2px);
}

.sort-dropdown-menu {
  position: absolute;
  top: calc(100% + 8px);
  right: 0;
  z-index: 30;
  min-width: 100%;
  padding: 6px;
  border: 1px solid rgba(148, 163, 184, 0.22);
  border-radius: 14px;
  background: rgba(255, 255, 255, 0.98);
  box-shadow:
    0 20px 40px rgba(15, 23, 42, 0.14),
    inset 0 1px 0 rgba(255, 255, 255, 0.75);
  backdrop-filter: blur(16px);
  -webkit-backdrop-filter: blur(16px);
}

.sort-dropdown-option {
  display: block;
  width: 100%;
  padding: 10px 12px;
  border-radius: 10px;
  text-align: left;
  color: var(--text);
  background: transparent;
  cursor: pointer;
  transition:
    background 140ms ease,
    color 140ms ease;
}

.sort-dropdown-option:hover {
  background: rgba(241, 245, 249, 0.92);
}

.sort-dropdown-option.active {
  color: #fff;
  background: linear-gradient(135deg, #213f75, #325ca8);
}
</style>
