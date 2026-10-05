<script setup lang="ts">
withDefaults(
  defineProps<{
    variant?: 'primary' | 'outline' | 'white'
    block?: boolean
    type?: 'button' | 'submit'
    disabled?: boolean
  }>(),
  { variant: 'primary', type: 'button' },
)
</script>

<template>
  <button
    class="ds-button"
    :class="[`is-${variant}`, { 'is-block': block }]"
    :type="type"
    :disabled="disabled"
  >
    <slot />
  </button>
</template>

<style scoped>
.ds-button {
  border: 1px solid transparent;
  border-radius: 4px;
  padding: 8px 16px;
  background: var(--ds-primary, #f8b82a);
  color: var(--ds-on-primary, #000);
  font:
    600 14px/1.4 Inter,
    sans-serif;
  cursor: pointer;
  transition:
    background 0.2s,
    transform 0.2s,
    border-color 0.2s;
}
.ds-button:hover {
  transform: translateY(-1px);
}
.is-primary:hover {
  background: var(--ds-primary-hover, #d4a017);
}
.is-outline {
  border-color: var(--ds-primary, #f8b82a);
  background: transparent;
  color: var(--ds-primary-hover, #d4a017);
}
.is-white {
  border-color: var(--ds-border, #eaeaea);
  background: #fff;
  color: var(--ds-ink, #333);
}
.is-block {
  width: 100%;
}
.ds-button:disabled {
  cursor: not-allowed;
  opacity: 0.55;
  transform: none;
}
.ds-button:disabled:hover {
  transform: none;
}
</style>
