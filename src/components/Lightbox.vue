<template>
  <div v-if="src" class="lightbox-overlay" @click="$emit('close')">
    <img :src="src" alt="" />
  </div>
</template>

<script setup lang="ts">
defineProps<{ src: string | null }>()
defineEmits<{ close: [] }>()
</script>
