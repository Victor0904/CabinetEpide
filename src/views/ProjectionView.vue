<template>
  <div class="proj-root">
    <div class="ctrl-bar">
      <span class="proj-info">Panier {{ numero }} · {{ fiches.length }} fiche(s)</span>
      <button class="ctrl-btn" @click="toggleMode">{{ mode === 'mosaique' ? '▶ Présentation' : '⊞ Mosaïque' }}</button>
      <span v-if="mode === 'presentation'" class="nav-ctrl">
        <button class="ctrl-btn" @click="showSlide(idx - 1)">◀ Préc.</button>
        <span class="counter">{{ fiches.length ? idx + 1 : 0 }} / {{ fiches.length }}</span>
        <button class="ctrl-btn" @click="showSlide(idx + 1)">Suiv. ▶</button>
      </span>
    </div>

    <div v-if="mode === 'mosaique'" class="grid" :style="gridStyle">
      <div
        v-for="f in fiches"
        :key="f.id_fiche_produit"
        class="fiche-card"
        @click="pleinEcran(f.id_fiche_produit)"
      >
        <MediaImg :chemin="f.photo_1" :alt="f.titre">
          <template #fallback><div class="no-img">📄</div></template>
        </MediaImg>
      </div>
    </div>
    <div v-else class="pres-view">
      <div v-if="current" class="slide" @click="pleinEcran(current.id_fiche_produit)">
        <MediaImg :chemin="current.photo_1" :alt="current.titre">
          <template #fallback><div class="no-img">📄</div></template>
        </MediaImg>
      </div>
    </div>

    <Lightbox :src="lightboxSrc" @close="lightboxSrc = null" />
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import MediaImg from '../components/MediaImg.vue'
import Lightbox from '../components/Lightbox.vue'
import { api } from '../api'
import { loadMediaUrl } from '../mediaCache'
import type { FicheDetail } from '../types'

const route = useRoute()
const numero = Number(route.params.numero)
const ids = String(route.query.ids ?? '')
  .split(',')
  .filter(Boolean)
  .map(Number)

const fiches = ref<FicheDetail[]>([])
onMounted(async () => {
  const results = await Promise.all(ids.map((id) => api.getFicheDetail(id).catch(() => null)))
  fiches.value = results.filter((f): f is FicheDetail => f !== null)
})

const mode = ref<'mosaique' | 'presentation'>('mosaique')
const idx = ref(0)
const current = computed(() => fiches.value[idx.value])

function toggleMode() {
  mode.value = mode.value === 'mosaique' ? 'presentation' : 'mosaique'
  if (mode.value === 'presentation') idx.value = 0
}
function showSlide(n: number) {
  const len = fiches.value.length
  if (!len) return
  idx.value = ((n % len) + len) % len
}

const cols = computed(() => (fiches.value.length <= 1 ? 1 : fiches.value.length <= 4 ? 2 : 3))
const rows = computed(() => Math.ceil(fiches.value.length / cols.value))
const gridStyle = computed(() => ({
  gridTemplateColumns: `repeat(${cols.value}, 1fr)`,
  gridTemplateRows: `repeat(${rows.value}, 1fr)`,
}))

const lightboxSrc = ref<string | null>(null)
function pleinEcran(id: number) {
  const f = fiches.value.find((x) => x.id_fiche_produit === id)
  if (f?.photo_1) loadMediaUrl(f.photo_1).then((src) => (lightboxSrc.value = src))
}

function onKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    lightboxSrc.value = null
    return
  }
  if (mode.value !== 'presentation') return
  if (e.key === 'ArrowRight' || e.key === 'ArrowDown') showSlide(idx.value + 1)
  if (e.key === 'ArrowLeft' || e.key === 'ArrowUp') showSlide(idx.value - 1)
}
onMounted(() => window.addEventListener('keydown', onKeydown))
onUnmounted(() => window.removeEventListener('keydown', onKeydown))
</script>

<style scoped>
.proj-root {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: #000;
  color: #e6edf3;
  overflow: hidden;
}
.ctrl-bar {
  height: 44px;
  background: #161b22;
  border-bottom: 1px solid #30363d;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 20px;
  flex-shrink: 0;
}
.proj-info { color: #8b949e; font-size: 0.85rem; }
.ctrl-btn {
  background: #21262d;
  color: #e6edf3;
  border: 1px solid #30363d;
  border-radius: 6px;
  padding: 5px 14px;
  cursor: pointer;
  font-size: 0.85rem;
  font-weight: 600;
  transition: background 0.2s;
}
.ctrl-btn:hover { background: #30363d; }
.nav-ctrl { display: flex; align-items: center; gap: 10px; }
.counter { color: #e6edf3; min-width: 60px; text-align: center; font-size: 0.9rem; }
.grid { flex: 1; display: grid; gap: 3px; padding: 3px; background: #111; overflow: hidden; }
.fiche-card {
  background: #0a0a0a;
  overflow: hidden;
  cursor: zoom-in;
  display: flex;
  align-items: center;
  justify-content: center;
}
.fiche-card :deep(img) { width: 100%; height: 100%; object-fit: contain; transition: opacity 0.15s; }
.fiche-card:hover :deep(img) { opacity: 0.85; }
.no-img { font-size: 4rem; color: #333; }
.pres-view { flex: 1; background: #000; display: flex; align-items: center; justify-content: center; overflow: hidden; }
.slide { width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; cursor: zoom-in; }
.slide :deep(img) { max-width: 100%; max-height: 100%; object-fit: contain; }
</style>
