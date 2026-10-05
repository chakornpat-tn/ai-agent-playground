<script setup lang="ts">
type IconName =
  | 'apple'
  | 'arrow-right'
  | 'bars'
  | 'basket'
  | 'bolt'
  | 'bread'
  | 'cart'
  | 'chevron-down'
  | 'chevron-left'
  | 'chevron-right'
  | 'clock'
  | 'drumstick'
  | 'envelope'
  | 'fish'
  | 'leaf'
  | 'lock'
  | 'magnifying-glass'
  | 'mug'
  | 'store'
  | 'user'
  | 'wine'
  | 'xmark'

defineProps<{ name: IconName | string; size?: number }>()
</script>

<template>
  <svg
    class="app-icon"
    :width="size ?? 16"
    :height="size ?? 16"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="1.9"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
  >
    <path
      v-if="name === 'store'"
      d="M3 10v10h18V10M4 10l1-6h14l1 6M3 10c0 1.3 1.1 2.3 2.5 2.3S8 11.3 8 10c0 1.3 1.1 2.3 2.5 2.3S13 11.3 13 10c0 1.3 1.1 2.3 2.5 2.3S18 11.3 18 10c0 1.3 1.1 2.3 2.5 2.3S23 11.3 23 10M9 20v-5h6v5"
    />
    <path
      v-else-if="name === 'cart'"
      d="M3 4h2l2.2 11.2a2 2 0 0 0 2 1.6h7.9a2 2 0 0 0 1.9-1.4L21 8H6M10 21a1 1 0 1 1-2 0 1 1 0 0 1 2 0Zm9 0a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z"
    />
    <path v-else-if="name === 'user'" d="M20 21a8 8 0 0 0-16 0M16 7a4 4 0 1 1-8 0 4 4 0 0 1 8 0Z" />
    <path v-else-if="name === 'bars'" d="M4 6h16M4 12h16M4 18h16" />
    <path
      v-else-if="name === 'magnifying-glass'"
      d="m21 21-4.3-4.3M10.8 18a7.2 7.2 0 1 1 0-14.4 7.2 7.2 0 0 1 0 14.4Z"
    />
    <path v-else-if="name === 'chevron-down'" d="m6 9 6 6 6-6" />
    <path v-else-if="name === 'chevron-left'" d="m15 18-6-6 6-6" />
    <path v-else-if="name === 'chevron-right'" d="m9 18 6-6-6-6" />
    <path v-else-if="name === 'arrow-right'" d="M4 12h15m-6-6 6 6-6 6" />
    <path v-else-if="name === 'clock'" d="M12 21a9 9 0 1 0 0-18 9 9 0 0 0 0 18Zm0-14v5l3 2" />
    <path v-else-if="name === 'xmark'" d="m6 6 12 12M18 6 6 18" />
    <path v-else-if="name === 'envelope'" d="M3 5h18v14H3zM3 6l9 7 9-7" />
    <path v-else-if="name === 'lock'" d="M5 10h14v10H5zM8 10V7a4 4 0 0 1 8 0v3" />
    <path
      v-else-if="name === 'leaf'"
      d="M20 4C11 4 5 8 5 15c0 3 2 5 5 5 7 0 10-7 10-16ZM4 20c3-5 7-8 12-10"
    />
    <path v-else-if="name === 'bolt'" d="m13 2-9 12h7l-1 8 9-12h-7l1-8Z" />
    <path
      v-else-if="name === 'bread'"
      d="M4 12V8c0-3 3-5 8-5s8 2 8 5v4M4 12c0 3 3 5 8 5s8-2 8-5M4 12v5c0 3 3 5 8 5s8-2 8-5v-5"
    />
    <path
      v-else-if="name === 'fish'"
      d="M3 12c3-5 8-7 14-6l4-3-1 5c1 2 1 4 0 6l1 5-4-3c-6 1-11-1-14-4Zm7-1h.01"
    />
    <path
      v-else-if="name === 'drumstick'"
      d="M14 5a4 4 0 0 1 5 5c-1 1-2 1-3 1l-7 7-3-3 7-7c0-1 0-2 1-3Zm-8 8-3 3m4 0-2 2"
    />
    <path
      v-else-if="name === 'wine'"
      d="M7 3h10l-1 6a4 4 0 0 1-3 3v7h4M7 3l1 6a4 4 0 0 0 3 3v7H7M8 8h8"
    />
    <path
      v-else-if="name === 'mug'"
      d="M4 7h13v8a4 4 0 0 1-4 4H8a4 4 0 0 1-4-4V7Zm13 2h2a2 2 0 0 1 0 4h-2M8 3v2m4-2v2"
    />
    <path
      v-else-if="name === 'apple'"
      d="M12 7c-3-3-8-1-8 5s3 9 8 9 8-3 8-9-5-8-8-5Zm0 0c0-3 2-5 5-5"
    />
    <path v-else-if="name === 'basket'" d="M4 10h16l-2 10H6L4 10Zm3 0 5-7 5 7M8 13v4m4-4v4m4-4v4" />
  </svg>
</template>

<style scoped>
.app-icon {
  display: inline-block;
  flex: 0 0 auto;
  vertical-align: -0.16em;
}
</style>
