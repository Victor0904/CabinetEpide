<template>
  <div class="grid-blocs">
    <p v-if="items.length === 0" style="grid-column:1/-1;text-align:center;color:#6b7280;padding:40px;">
      Aucun dossier{{ level === 1 ? '. Créez-en un via "Gérer Dossiers".' : ' ici.' }}
    </p>
    <div
      v-for="it in items"
      :key="it.id"
      class="bloc-dossier"
      :style="borderStyle"
      @click="$emit('open', it.id)"
    >
      <div v-if="modeAdmin" class="bloc-actions" @click.stop>
        <button class="btn-hover-action" @click="$emit('rename', it.id, it.titre)">✏️ Renommer</button>
        <button class="btn-hover-action danger" @click="$emit('delete', it.id, it.titre)">🗑️</button>
      </div>
      <div class="bloc-icon" v-html="icon"></div>
      <strong>{{ it.titre }}</strong>
      <small>{{ it.count }} fiche{{ it.count !== 1 ? 's' : '' }}</small>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { modeAdmin } from '../store'

const props = defineProps<{
  items: { id: number; titre: string; count: number }[]
  level: 1 | 2 | 3 | 4
}>()

defineEmits<{
  open: [id: number]
  rename: [id: number, titre: string]
  delete: [id: number, titre: string]
}>()

const ICONS: Record<number, string> = {
  1: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="34" height="34" style="color:#1e3a8a"><path d="M19.5 21a3 3 0 003-3v-4.5a3 3 0 00-3-3h-15a3 3 0 00-3 3V18a3 3 0 003 3h15zM1.5 10.146V6a3 3 0 013-3h5.379a2.25 2.25 0 011.59.659l2.122 2.121c.14.141.331.22.53.22H19.5a3 3 0 013 3v1.146A4.483 4.483 0 0019.5 9h-15a4.483 4.483 0 00-3 1.146z"/></svg>`,
  2: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="34" height="34" style="color:#0369a1"><path d="M19.906 9c.382 0 .749.057 1.094.162V9a3 3 0 00-3-3h-3.879a.75.75 0 01-.53-.22L11.47 3.66A2.25 2.25 0 009.879 3H6a3 3 0 00-3 3v3.162A3.756 3.756 0 014.094 9h15.812zM4.094 10.5a2.25 2.25 0 00-2.227 2.568l.857 6A2.25 2.25 0 004.951 21H19.05a2.25 2.25 0 002.227-1.932l.857-6a2.25 2.25 0 00-2.227-2.568H4.094z"/></svg>`,
  3: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="30" height="30" style="color:#3b82f6"><path fill-rule="evenodd" d="M5.625 1.5c-1.036 0-1.875.84-1.875 1.875v17.25c0 1.035.84 1.875 1.875 1.875h12.75c1.035 0 1.875-.84 1.875-1.875V12.75A3.75 3.75 0 0016.5 9h-1.875a1.875 1.875 0 01-1.875-1.875V5.25A3.75 3.75 0 009 1.5H5.625zM7.5 15a.75.75 0 01.75-.75h7.5a.75.75 0 010 1.5h-7.5A.75.75 0 017.5 15zm.75-6.75a.75.75 0 000 1.5H12a.75.75 0 000-1.5H8.25z" clip-rule="evenodd"/><path d="M12.971 1.816A5.23 5.23 0 0114.25 5.25v1.875c0 .207.168.375.375.375H16.5a5.23 5.23 0 013.434 1.279 9.768 9.768 0 00-6.963-6.963z"/></svg>`,
  4: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="28" height="28" style="color:#7c3aed"><path fill-rule="evenodd" d="M5.625 1.5c-1.036 0-1.875.84-1.875 1.875v17.25c0 1.035.84 1.875 1.875 1.875h12.75c1.035 0 1.875-.84 1.875-1.875V12.75A3.75 3.75 0 0016.5 9h-1.875a1.875 1.875 0 01-1.875-1.875V5.25A3.75 3.75 0 009 1.5H5.625zM7.5 15a.75.75 0 01.75-.75h7.5a.75.75 0 010 1.5h-7.5A.75.75 0 017.5 15zm.75-6.75a.75.75 0 000 1.5H12a.75.75 0 000-1.5H8.25z" clip-rule="evenodd"/><path d="M12.971 1.816A5.23 5.23 0 0114.25 5.25v1.875c0 .207.168.375.375.375H16.5a5.23 5.23 0 013.434 1.279 9.768 9.768 0 00-6.963-6.963z"/></svg>`,
}

const icon = computed(() => ICONS[props.level])
const borderStyle = computed(() => {
  if (props.level === 3) return { borderColor: '#93c5fd' }
  if (props.level === 4) return { borderColor: '#c4b5fd' }
  return {}
})
</script>
