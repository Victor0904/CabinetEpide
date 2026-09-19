<template>
  <div class="report-root">
    <h1>Consultation épidémiologique — {{ dateStr }}</h1>
    <p class="count">{{ fiches.length }} fiche(s) sélectionnée(s)</p>

    <template v-for="(f, i) in fiches" :key="f.id_fiche_produit">
      <hr v-if="i > 0" />
      <div class="fiche-rapport">
        <MediaImg :chemin="f.photo_1" :alt="f.titre" class="rapport-img">
          <template #fallback><div class="rapport-img no-img"></div></template>
        </MediaImg>
        <div class="rapport-info">
          <h2>{{ f.titre }}</h2>
          <p v-if="f.synonymes" class="syno">↳ {{ f.synonymes }}</p>
          <p v-if="f.clinique"><strong>Tableau clinique :</strong><br />{{ f.clinique }}</p>
          <p v-if="f.traitement"><strong>Traitement :</strong><br />{{ f.traitement }}</p>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { nextTick, onMounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import MediaImg from '../components/MediaImg.vue'
import { api } from '../api'
import type { FicheDetail } from '../types'

const route = useRoute()
const ids = String(route.query.ids ?? '')
  .split(',')
  .filter(Boolean)
  .map(Number)

const fiches = ref<FicheDetail[]>([])
const dateStr = new Date().toLocaleDateString('fr-FR', { day: '2-digit', month: 'long', year: 'numeric' })

onMounted(async () => {
  const results = await Promise.all(ids.map((id) => api.getFicheDetail(id).catch(() => null)))
  fiches.value = results.filter((f): f is FicheDetail => f !== null)
  await nextTick()
  window.print()
})
</script>

<style scoped>
.report-root {
  flex: 1;
  overflow-y: auto;
  font-family: 'Segoe UI', sans-serif;
  margin: 40px;
  color: #111;
  max-width: 800px;
}
h1 { color: #1e3a8a; border-bottom: 2px solid #1e3a8a; padding-bottom: 8px; }
.count { color: #6b7280; }
.fiche-rapport { display: flex; gap: 20px; padding: 20px 0; page-break-inside: avoid; }
.rapport-img { width: 160px; height: 160px; object-fit: cover; border-radius: 8px; flex-shrink: 0; background: #f3f4f6; }
.no-img { display: block; }
.rapport-info h2 { margin: 0 0 6px; color: #1e3a8a; }
.rapport-info p { margin: 8px 0; font-size: 0.9rem; line-height: 1.6; white-space: pre-wrap; }
.syno { color: #6b7280; font-style: italic; }
hr { border: none; border-top: 1px solid #e5e7eb; margin: 0; }
@media print {
  .report-root { margin: 20px; }
}
</style>
