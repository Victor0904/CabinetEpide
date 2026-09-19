<template>
  <router-view />
  <ToastNotif />
  <ConfirmDialogModal />
  <PromptDialogModal />
</template>

<script setup lang="ts">
import ToastNotif from './components/ToastNotif.vue'
import ConfirmDialogModal from './components/ConfirmDialogModal.vue'
import PromptDialogModal from './components/PromptDialogModal.vue'
</script>
