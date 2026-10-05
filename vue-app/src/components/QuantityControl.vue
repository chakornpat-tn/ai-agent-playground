<script setup lang="ts">
const model = defineModel<number>({ default: 1 })

const decrease = () => {
  model.value = Math.max(1, model.value - 1)
}
const increase = () => {
  model.value += 1
}
</script>

<template>
  <div class="quantity-control" aria-label="Quantity selector">
    <button type="button" aria-label="Decrease quantity" @click="decrease">-</button>
    <output>{{ model }}</output>
    <button type="button" aria-label="Increase quantity" @click="increase">+</button>
  </div>
</template>

<style scoped>
.quantity-control {
  display: inline-flex;
  overflow: hidden;
  border: 1px solid var(--ds-border, #eaeaea);
  border-radius: 4px;
  background: #fff;
}
.quantity-control button {
  width: 31px;
  height: 30px;
  border: 0;
  background: var(--ds-surface-soft, #f9f9f9);
  color: var(--ds-ink, #333);
  cursor: pointer;
  font-size: 16px;
  line-height: 1;
}
.quantity-control button:hover {
  background: #f2f2f2;
}
.quantity-control output {
  display: grid;
  width: 30px;
  place-items: center;
  border-right: 1px solid var(--ds-border, #eaeaea);
  border-left: 1px solid var(--ds-border, #eaeaea);
  color: var(--ds-ink, #333);
  font:
    12px 'DM Mono',
    monospace;
}
</style>
