<template>
  <div class="fiche-overlay" @click.self="$emit('close')">
    <div class="fiche-panel" v-if="fiche">
      <div class="fiche-topbar">
        <button class="btn-panier-fiche btn-panier-fiche-1" @click="ajouterAuPanier(fiche, 1)">+ Panier 1</button>
        <button class="btn-panier-fiche btn-panier-fiche-2" @click="ajouterAuPanier(fiche, 2)">+ Panier 2</button>
        <div class="fiche-topbar-spacer"></div>
        <button class="btn-telecharger-fiche" title="Télécharger la fiche" @click="telecharger">⬇️ Télécharger</button>
        <button v-if="!modeEdition" id="btnModifier" class="btn-modifier" @click="basculerEdition">✏️ Modifier</button>
        <template v-else>
          <button class="btn-sauvegarder" @click="sauvegarder">💾 Sauvegarder</button>
          <button class="btn-annuler" @click="annuler">Annuler</button>
          <button class="btn-supprimer-fiche" title="Supprimer la fiche" @click="supprimer">🗑️</button>
        </template>
        <button class="fiche-close" @click="$emit('close')">×</button>
      </div>

      <div class="fiche-corps">
        <div class="fiche-hero">
          <MediaImg :chemin="fiche.photo_1" :alt="fiche.titre" @click="openLightbox(fiche.photo_1)">
            <template #fallback><span></span></template>
          </MediaImg>
        </div>

        <div class="fiche-info-zone">
          <div class="fiche-chemin">{{ chemin }}</div>

          <h2 v-if="!modeEdition" class="fiche-titre">{{ fiche.titre }}</h2>
          <input v-else v-model="titreEdit" class="fiche-titre-edit" placeholder="Titre de la fiche" />

          <button v-if="modeEdition" class="btn-deplacer-fiche" @click="toggleDest">
            📁 Déplacer
          </button>
          <div v-if="modeEdition && destZoneOuverte" style="margin-bottom:12px;">
            <DestinationCascade ref="cascadeRef" @change="destinationEdit = $event" />
          </div>

          <div class="fiche-synonymes-line">
            <span class="fiche-label-inline">Synonymes :</span>
            <span v-if="!modeEdition" class="fiche-val-inline">{{ fiche.synonymes }}</span>
            <input v-else v-model="synonymesEdit" class="fiche-input-inline" placeholder="Aucun synonyme connu" />
          </div>

          <div class="fiche-descriptif-zone">
            <div v-if="!modeEdition" class="fiche-descriptif-view">{{ fiche.descriptif }}</div>
            <textarea
              v-else
              v-model="descriptifEdit"
              class="fiche-descriptif-edit"
              rows="4"
              placeholder="Ajouter un commentaire..."
            ></textarea>
          </div>
        </div>

        <div class="fiche-medias-zone">
          <div class="fiche-medias-grid">
            <div
              v-for="m in fiche.medias"
              :key="m.id_media"
              class="media-vignette"
              :class="{ 'media-pdf': m.type !== 'image' }"
              :title="m.nom_original ?? ''"
              @click="m.type === 'image' ? openLightbox(m.chemin) : ouvrirPdf(m)"
            >
              <template v-if="m.type === 'image'">
                <MediaImg :chemin="m.chemin" :alt="m.nom_original ?? ''">
                  <template #fallback><span style="font-size:1.5rem">🖼️</span></template>
                </MediaImg>
              </template>
              <template v-else>
                <span class="pdf-icon">📄</span>
                <span class="pdf-nom">{{ m.nom_original }}</span>
              </template>
              <button v-if="modeEdition" class="btn-del-media" title="Supprimer" @click.stop="supprimerMedia(m)">
                ×
              </button>
            </div>
          </div>
          <label class="btn-add-media" title="Ajouter image ou PDF">
            + Ajouter
            <input type="file" accept="image/*,.pdf" multiple style="display:none;" @change="onUploadMedias" />
          </label>
        </div>
      </div>
    </div>

    <Lightbox :src="lightboxSrc" @close="lightboxSrc = null" />
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, onMounted, ref } from 'vue'
import { save } from '@tauri-apps/plugin-dialog'
import MediaImg from './MediaImg.vue'
import Lightbox from './Lightbox.vue'
import DestinationCascade from './DestinationCascade.vue'
import { api, fileToBase64 } from '../api'
import { loadMediaUrl, invalidateMedia } from '../mediaCache'
import { ajouterAuPanier, chargerBaseDeDonnees, db, retirerFicheDeTousLesPaniers, showToast } from '../store'
import { confirmDialog } from '../dialogs'
import { cheminFiche } from '../tree'
import { buildFicheHtml } from '../ficheExport'
import type { Destination, FicheDetail, FicheMedia } from '../types'

const props = defineProps<{ id: number; startEdit?: boolean }>()
const emit = defineEmits<{ close: []; 'open-pdf': [src: string, nom: string, chemin: string] }>()

const fiche = ref<FicheDetail | null>(null)
const modeEdition = ref(false)
const titreEdit = ref('')
const synonymesEdit = ref('')
const descriptifEdit = ref('')
const destZoneOuverte = ref(false)
const destinationEdit = ref<Destination | null>(null)
const lightboxSrc = ref<string | null>(null)
const cascadeRef = ref<InstanceType<typeof DestinationCascade> | null>(null)

const chemin = computed(() => (fiche.value ? cheminFiche(db, fiche.value).join(' › ') : ''))

const currentDestination = computed<Destination | null>(() => {
  const f = fiche.value
  if (!f) return null
  if (f.id_s_s_s_classeur != null) return { niveau: 4, id: f.id_s_s_s_classeur }
  if (f.id_s_s_classeur != null) return { niveau: 3, id: f.id_s_s_classeur }
  if (f.id_sous_classeur != null) return { niveau: 2, id: f.id_sous_classeur }
  if (f.id_classeur != null) return { niveau: 1, id: f.id_classeur }
  return null
})

async function load() {
  try {
    fiche.value = await api.getFicheDetail(props.id)
    if (props.startEdit) basculerEdition()
  } catch (e) {
    showToast('❌ Erreur : ' + String(e), true)
    emit('close')
  }
}
onMounted(load)

function basculerEdition() {
  if (!fiche.value) return
  titreEdit.value = fiche.value.titre
  synonymesEdit.value = fiche.value.synonymes ?? ''
  descriptifEdit.value = fiche.value.descriptif ?? ''
  destinationEdit.value = null
  destZoneOuverte.value = false
  modeEdition.value = true
}
function annuler() {
  modeEdition.value = false
}

function toggleDest() {
  destZoneOuverte.value = !destZoneOuverte.value
  if (destZoneOuverte.value) {
    nextTick(() => cascadeRef.value?.setPreset(currentDestination.value))
  }
}

async function sauvegarder() {
  if (!fiche.value) return
  const dest = destinationEdit.value ?? currentDestination.value
  try {
    await api.updateFiche({
      id_fiche_produit: fiche.value.id_fiche_produit,
      titre: titreEdit.value,
      descriptif: descriptifEdit.value || null,
      synonymes: synonymesEdit.value || null,
      id_classeur: dest?.niveau === 1 ? dest.id : null,
      id_sous_classeur: dest?.niveau === 2 ? dest.id : null,
      id_s_s_classeur: dest?.niveau === 3 ? dest.id : null,
      id_s_s_s_classeur: dest?.niveau === 4 ? dest.id : null,
    })
    showToast('✅ Fiche sauvegardée !')
    modeEdition.value = false
    await chargerBaseDeDonnees()
    await load()
  } catch (e) {
    showToast('❌ Erreur : ' + String(e), true)
  }
}

async function supprimer() {
  if (!fiche.value) return
  const ok = await confirmDialog(
    `Supprimer définitivement "${fiche.value.titre}" ? Tous les médias associés seront effacés.`,
    true,
  )
  if (!ok) return
  try {
    await api.deleteFiche(fiche.value.id_fiche_produit)
    retirerFicheDeTousLesPaniers(fiche.value.id_fiche_produit)
    showToast('🗑️ Fiche supprimée.')
    await chargerBaseDeDonnees()
    emit('close')
  } catch (e) {
    showToast('❌ Erreur : ' + String(e), true)
  }
}

function openLightbox(chemin?: string | null) {
  if (!chemin) return
  loadMediaUrl(chemin).then((src) => (lightboxSrc.value = src))
}

function ouvrirPdf(m: FicheMedia) {
  loadMediaUrl(m.chemin).then((src) => emit('open-pdf', src, m.nom_original ?? 'Document', m.chemin))
}

async function onUploadMedias(e: Event) {
  const input = e.target as HTMLInputElement
  if (!fiche.value || !input.files?.length) return
  for (const file of Array.from(input.files)) {
    try {
      const base64 = await fileToBase64(file)
      const media = await api.uploadMedia({
        id_fiche_produit: fiche.value.id_fiche_produit,
        file_name: file.name,
        file_base64: base64,
      })
      fiche.value.medias.push(media)
    } catch (err) {
      showToast('❌ ' + String(err), true)
    }
  }
  input.value = ''
}

async function supprimerMedia(m: FicheMedia) {
  if (!fiche.value) return
  const ok = await confirmDialog('Supprimer ce fichier ?')
  if (!ok) return
  try {
    await api.deleteMedia(m.id_media)
    invalidateMedia(m.chemin)
    fiche.value.medias = fiche.value.medias.filter((x) => x.id_media !== m.id_media)
  } catch (e) {
    showToast('❌ Erreur suppression', true)
  }
}

async function telecharger() {
  if (!fiche.value) return
  showToast('⏳ Préparation du téléchargement...')
  try {
    const html = await buildFicheHtml(fiche.value, chemin.value)
    const fileName = fiche.value.titre.replace(/[^\w\sÀ-ɏ-]/g, '_') + '.html'
    const path = await save({ defaultPath: fileName, filters: [{ name: 'Page HTML', extensions: ['html'] }] })
    if (!path) return
    await api.exportTextFile(path, html)
    showToast('✅ Fiche téléchargée.')
  } catch (e) {
    showToast('❌ Erreur : ' + String(e), true)
  }
}
</script>
