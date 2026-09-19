<template>
  <div class="pdf-overlay" @click.self="$emit('close')">
    <div class="pdf-panel">
      <div class="pdf-header">
        <span>{{ nom }}</span>
        <div style="display:flex;gap:8px;">
          <button class="btn-pdf-ext" @click="ouvrirExterne">↗ Ouvrir</button>
          <button class="fiche-close" style="color:#9ca3af;" @click="$emit('close')">×</button>
        </div>
      </div>
      <embed :src="src" type="application/pdf" class="pdf-embed" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { openPath } from '@tauri-apps/plugin-opener'
import { api } from '../api'
import { showToast } from '../store'

const props = defineProps<{ src: string; nom: string; chemin: string }>()
defineEmits<{ close: [] }>()

async function ouvrirExterne() {
  try {
    const path = await api.getMediaPath(props.chemin)
    await openPath(path)
  } catch (e) {
    showToast('❌ ' + String(e), true)
  }
}
</script>
