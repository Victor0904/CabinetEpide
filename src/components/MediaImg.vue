<template>
  <img v-if="src" :src="src" :alt="alt || ''" @error="onError" />
  <slot v-else name="fallback">
    <span>{{ fallbackText }}</span>
  </slot>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import { loadMediaUrl } from '../mediaCache'

const props = defineProps<{
  chemin?: string | null
  alt?: string
  fallbackText?: string
}>()

const src = ref<string | null>(null)

async function load() {
  src.value = null
  if (!props.chemin) return
  try {
    src.value = await loadMediaUrl(props.chemin)
  } catch {
    src.value = null
  }
}

function onError() {
  src.value = null
}

onMounted(load)
watch(() => props.chemin, load)
</script>
