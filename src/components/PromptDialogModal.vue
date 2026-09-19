<template>
  <div v-if="promptState.visible" class="modal-overlay dialog-top">
    <div class="modal-content modal-confirm-suppr">
      <h2 class="confirm-suppr-titre">{{ promptState.message }}</h2>
      <input
        ref="inputRef"
        v-model="promptState.value"
        type="text"
        autocomplete="off"
        @keydown.enter="resolvePrompt(promptState.value)"
        @keydown.esc="resolvePrompt(null)"
      />
      <div class="confirm-suppr-actions">
        <button class="btn-enregistrer" @click="resolvePrompt(promptState.value)">Valider</button>
        <button class="btn-enregistrer" style="background:#6b7280;" @click="resolvePrompt(null)">Annuler</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { nextTick, ref, watch } from 'vue'
import { promptState, resolvePrompt } from '../dialogs'

const inputRef = ref<HTMLInputElement | null>(null)
watch(
  () => promptState.visible,
  async (v) => {
    if (v) {
      await nextTick()
      inputRef.value?.focus()
      inputRef.value?.select()
    }
  },
)
</script>
