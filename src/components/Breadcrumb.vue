<template>
  <div class="breadcrumb">
    <template v-for="(c, i) in crumbs" :key="i">
      <span v-if="i > 0" class="separator">&gt;</span>
      <span
        :class="c.onClick ? 'bc-link' : 'current'"
        @click="c.onClick && c.onClick()"
      >{{ c.label }}</span>
    </template>
    <button v-if="action" class="btn-ajouter-ici" @click="action.onClick">{{ action.label }}</button>
  </div>
</template>

<script setup lang="ts">
export interface Crumb {
  label: string
  onClick?: () => void
}

defineProps<{
  crumbs: Crumb[]
  action?: { label: string; onClick: () => void }
}>()
</script>
