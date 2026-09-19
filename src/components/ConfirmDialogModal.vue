<template>
  <div v-if="confirmState.visible" class="modal-overlay dialog-top">
    <div class="modal-content modal-confirm-suppr">
      <h2 class="confirm-suppr-titre">⚠️ Confirmation</h2>
      <p class="confirm-suppr-msg">{{ confirmState.message }}</p>

      <template v-if="confirmState.requireTypeConfirm">
        <p class="confirm-suppr-hint">Tapez <strong>supprimer</strong> pour confirmer :</p>
        <input
          ref="inputRef"
          v-model="saisie"
          type="text"
          placeholder="supprimer"
          autocomplete="off"
          @keydown.enter="onOk"
        />
      </template>

      <div class="confirm-suppr-actions">
        <button
          class="btn-enregistrer"
          style="background:#dc2626;"
          :disabled="confirmState.requireTypeConfirm && saisie.trim().toLowerCase() !== 'supprimer'"
          @click="onOk"
        >
          🗑️ Confirmer
        </button>
        <button class="btn-enregistrer" style="background:#6b7280;" @click="resolveConfirm(false)">Annuler</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { nextTick, ref, watch } from 'vue'
import { confirmState, resolveConfirm } from '../dialogs'

const saisie = ref('')
const inputRef = ref<HTMLInputElement | null>(null)
watch(
  () => confirmState.visible,
  async (v) => {
    if (v) {
      saisie.value = ''
      if (confirmState.requireTypeConfirm) {
        await nextTick()
        inputRef.value?.focus()
      }
    }
  },
)

function onOk() {
  if (confirmState.requireTypeConfirm && saisie.value.trim().toLowerCase() !== 'supprimer') return
  resolveConfirm(true)
}
</script>
