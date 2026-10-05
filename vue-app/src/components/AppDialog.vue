<script setup lang="ts">
import { watch } from 'vue'
import AppIcon from './AppIcon.vue'

const props = defineProps<{
  open: boolean
  title?: string
  description?: string
}>()

const emit = defineEmits<{
  'update:open': [value: boolean]
  close: []
}>()

const close = () => {
  emit('update:open', false)
  emit('close')
}

const onBackdropClick = (e: MouseEvent) => {
  if (e.target === e.currentTarget) {
    close()
  }
}

watch(
  () => props.open,
  (val) => {
    if (val) {
      document.body.style.overflow = 'hidden'
    } else {
      document.body.style.overflow = ''
    }
  },
)
</script>

<template>
  <Teleport to="body">
    <Transition name="dialog">
      <div v-if="open" class="dialog-overlay" @click="onBackdropClick">
        <div class="dialog" role="dialog" aria-modal="true">
          <div class="dialog-header">
            <div>
              <h2 v-if="title" class="dialog-title">{{ title }}</h2>
              <p v-if="description" class="dialog-desc">{{ description }}</p>
            </div>
            <button class="dialog-close" @click="close" aria-label="Close">
              <AppIcon name="xmark" :size="20" />
            </button>
          </div>
          <div class="dialog-body">
            <slot />
          </div>
          <div v-if="$slots.footer" class="dialog-footer">
            <slot name="footer" />
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 16px;
}

.dialog {
  background: var(--canvas, #ffffff);
  border-radius: 8px;
  width: 100%;
  max-width: 480px;
  max-height: 85vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.15);
}

.dialog-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  padding: 24px;
}

.dialog-title {
  font-size: 20px;
  font-weight: 700;
  color: var(--ink, #333333);
  margin: 0;
}

.dialog-desc {
  font-size: 14px;
  color: var(--muted, #777777);
  margin: 4px 0 0;
}

.dialog-close {
  background: transparent;
  border: none;
  font-size: 20px;
  color: var(--muted, #777777);
  cursor: pointer;
  padding: 4px;
  line-height: 1;
  transition: color 0.2s;
}

.dialog-close:hover {
  color: var(--ink, #333333);
}

.dialog-body {
  padding: 0 24px 24px;
  overflow-y: auto;
}

.dialog-footer {
  padding: 0 24px 24px;
  display: flex;
  gap: 12px;
  justify-content: flex-end;
}

/* Transitions */
.dialog-enter-active,
.dialog-leave-active {
  transition: opacity 0.2s ease;
}

.dialog-enter-active .dialog,
.dialog-leave-active .dialog {
  transition:
    transform 0.2s ease,
    opacity 0.2s ease;
}

.dialog-enter-from,
.dialog-leave-to {
  opacity: 0;
}

.dialog-enter-from .dialog,
.dialog-leave-to .dialog {
  transform: scale(0.95);
  opacity: 0;
}
</style>
