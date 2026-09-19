<template>
  <div class="fiche-toast" :class="{ visible: toast.visible }" :style="{ background: toast.error ? '#ef4444' : '#10b981' }">
    {{ toast.message }}
  </div>
</template>

<script setup lang="ts">
import { toast } from '../store'
</script>
