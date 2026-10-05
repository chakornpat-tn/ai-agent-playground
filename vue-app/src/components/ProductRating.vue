<script setup lang="ts">
withDefaults(defineProps<{ rating?: number; reviews?: number }>(), { rating: 5, reviews: 0 })
</script>

<template>
  <div class="product-rating" :aria-label="`${rating} out of 5 stars`">
    <span v-for="star in 5" :key="star" :class="{ empty: star > rating }">★</span>
    <small v-if="reviews">({{ reviews }})</small>
  </div>
</template>

<style scoped>
.product-rating {
  display: inline-flex;
  align-items: center;
  gap: 2px;
  color: var(--ds-star, #f1c40f);
  font-size: 12px;
}
.product-rating span.empty {
  color: var(--ds-border, #eaeaea);
}
.product-rating small {
  margin-left: 5px;
  color: var(--ds-muted, #777);
  font-size: 11px;
}
</style>
