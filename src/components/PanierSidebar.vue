<template>
  <aside class="zone-paniers" :class="{ visible: anyFiche }">
    <div v-for="n in [1, 2] as const" :key="n" class="panier">
      <div class="panier-header">
        <h3 :class="n === 1 ? 'panier-1-titre' : 'panier-2-titre'">PANIER {{ n }}</h3>
        <div style="display:flex;gap:6px;">
          <button class="btn-projeter" @click="onProjeter(n)">🎬 Projeter</button>
          <button class="btn-vider" @click="onCloturer(n)">Clôturer</button>
        </div>
      </div>
      <ul class="liste-items">
        <li v-for="f in paniers[n]" :key="f.id_fiche_produit" class="panier-item">
          <div class="panier-item-thumb">
            <MediaImg :chemin="f.photo_1" :alt="f.titre" fallback-text="📄" />
          </div>
          <span class="panier-item-titre">{{ f.titre }}</span>
          <button class="btn-supprimer" title="Retirer" @click="retirerDuPanier(f.id_fiche_produit, n)">✕</button>
        </li>
      </ul>
      <div class="zone-video">
        <div class="mosaique-grid">
          <div v-if="paniers[n].length === 0" class="video-vide">Aucune fiche</div>
          <div v-for="f in paniers[n]" v-else :key="f.id_fiche_produit" class="video-item">
            <MediaImg :chemin="f.photo_1" :alt="f.titre">
              <template #fallback><span>{{ f.titre }}</span></template>
            </MediaImg>
          </div>
        </div>
      </div>
    </div>
    <button class="btn-enregistrer" style="margin-top:auto;" @click="onExporter">📋 Enregistrer la consultation</button>
  </aside>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import MediaImg from './MediaImg.vue'
import { clearPanier, paniers, showToast } from '../store'
import { confirmDialog } from '../dialogs'
import { openConsultation, openProjection } from '../windows'

const anyFiche = computed(() => paniers[1].length > 0 || paniers[2].length > 0)

function retirerDuPanier(id: number, n: 1 | 2) {
  paniers[n] = paniers[n].filter((f) => f.id_fiche_produit !== id)
}

async function onCloturer(n: 1 | 2) {
  if (paniers[n].length === 0) return
  const ok = await confirmDialog(`Clôturer le panier ${n} ? Les fiches sélectionnées seront effacées.`)
  if (ok) clearPanier(n)
}

function onProjeter(n: 1 | 2) {
  if (paniers[n].length === 0) {
    showToast('⚠️ Le panier est vide.', true)
    return
  }
  openProjection(
    n,
    paniers[n].map((f) => f.id_fiche_produit),
  )
}

function onExporter() {
  const toutes = [...paniers[1], ...paniers[2]]
  if (!toutes.length) {
    showToast('⚠️ Les deux paniers sont vides.', true)
    return
  }
  openConsultation(toutes.map((f) => f.id_fiche_produit))
}
</script>
