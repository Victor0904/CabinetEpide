<template>
  <div class="grid-produits">
    <div v-if="destination" class="carte carte-ajout" @click="$emit('add-here')">
      <div class="carte-ajout-plus">+</div>
      <span class="carte-ajout-label">Ajouter une<br />fiche ici</span>
    </div>

    <p v-if="fiches.length === 0 && !destination" style="grid-column:1/-1;text-align:center;color:#ef4444;padding:20px;">
      Aucun produit trouvé pour cette recherche.
    </p>

    <div
      v-for="f in fiches"
      :key="f.id_fiche_produit"
      class="carte"
      :data-id="f.id_fiche_produit"
      title="Ouvrir la fiche"
      @click="$emit('open', f.id_fiche_produit)"
    >
      <div class="carte-actions" @click.stop>
        <button class="btn-hover-action" @click="$emit('edit', f.id_fiche_produit)">✏️ Modifier</button>
        <button class="btn-hover-action danger" @click="$emit('delete', f.id_fiche_produit, f.titre)">🗑️</button>
      </div>
      <div v-if="estDansUnPanier(f.id_fiche_produit).length" class="badge-panier">
        <span v-for="n in estDansUnPanier(f.id_fiche_produit)" :key="n" :class="n === 1 ? 'b1' : 'b2'">P{{ n }}</span>
      </div>
      <div class="carte-img">
        <MediaImg :chemin="f.photo_1" :alt="f.titre" fallback-text="Pas d'image" />
      </div>
      <h3>{{ f.titre }}</h3>
      <p>{{ f.descriptif ? f.descriptif.substring(0, 50) + '...' : 'Aucune description' }}</p>
      <div v-if="panierActif" @click.stop>
        <button class="btn-panier-1" @click="ajouterAuPanier(f, 1)">+ Panier 1</button>
        <button class="btn-panier-2" @click="ajouterAuPanier(f, 2)">+ Panier 2</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import MediaImg from './MediaImg.vue'
import { ajouterAuPanier, estDansUnPanier, panierActif } from '../store'
import type { Destination, FicheProduit } from '../types'

defineProps<{
  fiches: FicheProduit[]
  destination: Destination | null
}>()

defineEmits<{
  open: [id: number]
  edit: [id: number]
  delete: [id: number, titre: string]
  'add-here': []
}>()
</script>
