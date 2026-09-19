<template>
  <AppHeader @open-upload="openUpload(null)" @open-arbo="openArbo(null)" />

  <div class="container">
    <main>
      <Breadcrumb :crumbs="breadcrumb.crumbs" :action="breadcrumb.action" />

      <FolderGrid
        v-if="viewState.type === 'folders'"
        :items="viewState.items"
        :level="viewState.level"
        @open="onOpenFolder"
        @rename="onRenameFolder"
        @delete="onDeleteFolder"
      />
      <FicheGrid
        v-else
        :fiches="viewState.fiches"
        :destination="viewState.destination"
        @open="(id) => openFiche(id, false)"
        @edit="(id) => openFiche(id, true)"
        @delete="onDeleteFiche"
        @add-here="onAddHere"
      />
    </main>

    <PanierSidebar v-if="panierActif" />
  </div>

  <ModalUpload v-if="uploadOpen" :preset="uploadPreset" @close="uploadOpen = false" />
  <ModalArborescence v-if="arboOpen" :preset="arboPreset" @close="arboOpen = false" />
  <ModalFiche
    v-if="ficheOpenId !== null"
    :id="ficheOpenId"
    :start-edit="ficheStartEdit"
    @close="ficheOpenId = null"
    @open-pdf="onOpenPdf"
  />
  <ModalPdf v-if="pdfState" :src="pdfState.src" :nom="pdfState.nom" :chemin="pdfState.chemin" @close="pdfState = null" />
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import AppHeader from '../components/AppHeader.vue'
import Breadcrumb from '../components/Breadcrumb.vue'
import type { Crumb } from '../components/Breadcrumb.vue'
import FolderGrid from '../components/FolderGrid.vue'
import FicheGrid from '../components/FicheGrid.vue'
import PanierSidebar from '../components/PanierSidebar.vue'
import ModalUpload from '../components/ModalUpload.vue'
import ModalArborescence from '../components/ModalArborescence.vue'
import ModalFiche from '../components/ModalFiche.vue'
import ModalPdf from '../components/ModalPdf.vue'
import { api } from '../api'
import { confirmDialog, promptDialog } from '../dialogs'
import {
  chargerBaseDeDonnees,
  db,
  irAccueil,
  irClasseur,
  irFamille,
  irSousFamille,
  irSousSousFamille,
  modeAdmin,
  nav,
  panierActif,
  retirerFicheDeTousLesPaniers,
  searchResults,
  searchTerm,
  showToast,
} from '../store'
import {
  compterFichesClasseur,
  compterFichesFamille,
  compterFichesSousFamille,
  compterFichesSousSousFamille,
  famillesDe,
  fichesPourDestination,
  findClasseur,
  findSousClasseur,
  findSousSousClasseur,
  findSousSousSousClasseur,
  sousFamillesDe,
  sousSousFamillesDe,
} from '../tree'
import type { CategoryType, Destination, FicheProduit } from '../types'

onMounted(chargerBaseDeDonnees)

// ── Vue courante (dossiers ou fiches) ───────────────────────────

type ViewState =
  | { type: 'folders'; level: 1 | 2 | 3 | 4; items: { id: number; titre: string; count: number }[] }
  | { type: 'fiches'; destination: Destination | null; fiches: FicheProduit[] }

const viewState = computed<ViewState>(() => {
  if (searchResults.value !== null) {
    return { type: 'fiches', destination: null, fiches: searchResults.value }
  }
  const n = nav.value
  if (n.kind === 'home') {
    return {
      type: 'folders',
      level: 1,
      items: db.classeurs.map((c) => ({ id: c.id_classeur, titre: c.titre, count: compterFichesClasseur(db, c.id_classeur) })),
    }
  }
  if (n.kind === 'families') {
    return {
      type: 'folders',
      level: 2,
      items: famillesDe(db, n.id).map((f) => ({
        id: f.id_sous_classeur,
        titre: f.titre,
        count: compterFichesFamille(db, f.id_sous_classeur),
      })),
    }
  }
  if (n.kind === 'subfamilies') {
    return {
      type: 'folders',
      level: 3,
      items: sousFamillesDe(db, n.id).map((f) => ({
        id: f.id_s_s_classeur,
        titre: f.titre,
        count: compterFichesSousFamille(db, f.id_s_s_classeur),
      })),
    }
  }
  if (n.kind === 'subsubfamilies') {
    return {
      type: 'folders',
      level: 4,
      items: sousSousFamillesDe(db, n.id).map((f) => ({
        id: f.id_s_s_s_classeur,
        titre: f.titre,
        count: compterFichesSousSousFamille(db, f.id_s_s_s_classeur),
      })),
    }
  }
  if (n.kind === 'fiches') {
    return { type: 'fiches', destination: { niveau: n.niveau, id: n.id }, fiches: fichesPourDestination(db, n.niveau, n.id) }
  }
  return { type: 'fiches', destination: null, fiches: db.fiches }
})

function onOpenFolder(id: number) {
  if (viewState.value.type !== 'folders') return
  const level = viewState.value.level
  if (level === 1) irClasseur(id)
  else if (level === 2) irFamille(id)
  else if (level === 3) irSousFamille(id)
  else irSousSousFamille(id)
}

function typeForLevel(level: 1 | 2 | 3 | 4): CategoryType {
  return level === 1 ? 'classeur' : level === 2 ? 'famille' : level === 3 ? 'sous_famille' : 'sous_sous_famille'
}

async function onRenameFolder(id: number, titreActuel: string) {
  if (viewState.value.type !== 'folders') return
  const nouveau = await promptDialog(`Renommer "${titreActuel}" en :`, titreActuel)
  if (!nouveau || !nouveau.trim() || nouveau.trim() === titreActuel) return
  try {
    await api.renameCategory({ type: typeForLevel(viewState.value.level), id, titre: nouveau.trim() })
    showToast('✅ Renommé avec succès.')
    await chargerBaseDeDonnees()
  } catch (e) {
    showToast('❌ ' + String(e), true)
  }
}

async function onDeleteFolder(id: number, titre: string) {
  if (viewState.value.type !== 'folders') return
  const ok = await confirmDialog(`Supprimer "${titre}" et tout son contenu ? Cette action est irréversible.`, true)
  if (!ok) return
  try {
    await api.deleteCategory({ type: typeForLevel(viewState.value.level), id })
    showToast('🗑️ Supprimé avec succès.')
    await chargerBaseDeDonnees()
  } catch (e) {
    showToast('❌ ' + String(e), true)
  }
}

async function onDeleteFiche(id: number, titre: string) {
  const ok = await confirmDialog(`Supprimer la fiche "${titre}" ? Cette action est irréversible.`, true)
  if (!ok) return
  try {
    await api.deleteFiche(id)
    retirerFicheDeTousLesPaniers(id)
    showToast('🗑️ Fiche supprimée.')
    await chargerBaseDeDonnees()
  } catch (e) {
    showToast('❌ ' + String(e), true)
  }
}

// ── Fil d'Ariane ─────────────────────────────────────────────

interface BreadcrumbState {
  crumbs: Crumb[]
  action?: { label: string; onClick: () => void }
}

const breadcrumb = computed<BreadcrumbState>(() => {
  if (searchResults.value !== null) {
    return { crumbs: [{ label: 'Accueil', onClick: irAccueil }, { label: `Résultats pour "${searchTerm.value.trim()}"` }] }
  }
  const n = nav.value
  const home: Crumb = { label: 'Accueil', onClick: irAccueil }

  if (n.kind === 'home') return { crumbs: [{ label: 'Accueil' }] }
  if (n.kind === 'all_fiches') return { crumbs: [home, { label: 'Toutes les fiches' }] }

  if (n.kind === 'families') {
    const cl = findClasseur(db, n.id)
    return {
      crumbs: [home, { label: cl?.titre ?? '' }],
      action: modeAdmin.value ? { label: '+ Nouveau dossier', onClick: () => openArbo({ type: 'famille', parentId: n.id }) } : undefined,
    }
  }
  if (n.kind === 'subfamilies') {
    const sc = findSousClasseur(db, n.id)
    const cl = sc ? findClasseur(db, sc.id_classeur) : undefined
    const crumbs: Crumb[] = [home]
    if (cl) crumbs.push({ label: cl.titre, onClick: () => irClasseur(cl.id_classeur) })
    crumbs.push({ label: sc?.titre ?? '' })
    return {
      crumbs,
      action: modeAdmin.value ? { label: '+ Nouveau dossier', onClick: () => openArbo({ type: 'sous_famille', parentId: n.id }) } : undefined,
    }
  }
  if (n.kind === 'subsubfamilies') {
    const ssc = findSousSousClasseur(db, n.id)
    const sc = ssc ? findSousClasseur(db, ssc.id_sous_classeur) : undefined
    const cl = sc ? findClasseur(db, sc.id_classeur) : undefined
    const crumbs: Crumb[] = [home]
    if (cl) crumbs.push({ label: cl.titre, onClick: () => irClasseur(cl.id_classeur) })
    if (sc) crumbs.push({ label: sc.titre, onClick: () => irFamille(sc.id_sous_classeur) })
    crumbs.push({ label: ssc?.titre ?? '' })
    return {
      crumbs,
      action: modeAdmin.value
        ? { label: '+ Nouveau dossier', onClick: () => openArbo({ type: 'sous_sous_famille', parentId: n.id }) }
        : undefined,
    }
  }

  // fiches
  const { niveau, id } = n
  const crumbs: Crumb[] = [home]
  if (niveau === 1) {
    crumbs.push({ label: findClasseur(db, id)?.titre ?? '' })
  } else if (niveau === 2) {
    const sc = findSousClasseur(db, id)
    const cl = sc ? findClasseur(db, sc.id_classeur) : undefined
    if (cl) crumbs.push({ label: cl.titre, onClick: () => irClasseur(cl.id_classeur) })
    crumbs.push({ label: sc?.titre ?? '' })
  } else if (niveau === 3) {
    const ssc = findSousSousClasseur(db, id)
    const sc = ssc ? findSousClasseur(db, ssc.id_sous_classeur) : undefined
    const cl = sc ? findClasseur(db, sc.id_classeur) : undefined
    if (cl) crumbs.push({ label: cl.titre, onClick: () => irClasseur(cl.id_classeur) })
    if (sc) crumbs.push({ label: sc.titre, onClick: () => irFamille(sc.id_sous_classeur) })
    crumbs.push({ label: ssc?.titre ?? '' })
  } else {
    const sssc = findSousSousSousClasseur(db, id)
    const ssc = sssc ? findSousSousClasseur(db, sssc.id_s_s_classeur) : undefined
    const sc = ssc ? findSousClasseur(db, ssc.id_sous_classeur) : undefined
    const cl = sc ? findClasseur(db, sc.id_classeur) : undefined
    if (cl) crumbs.push({ label: cl.titre, onClick: () => irClasseur(cl.id_classeur) })
    if (sc) crumbs.push({ label: sc.titre, onClick: () => irFamille(sc.id_sous_classeur) })
    if (ssc) crumbs.push({ label: ssc.titre, onClick: () => irSousFamille(ssc.id_s_s_classeur) })
    crumbs.push({ label: sssc?.titre ?? '' })
  }
  return {
    crumbs,
    action: modeAdmin.value ? { label: '+ Ajouter ici', onClick: () => openUpload({ niveau, id }) } : undefined,
  }
})

// ── Modales ──────────────────────────────────────────────────

const uploadOpen = ref(false)
const uploadPreset = ref<Destination | null>(null)
function openUpload(preset: Destination | null) {
  uploadPreset.value = preset
  uploadOpen.value = true
}
function onAddHere() {
  if (viewState.value.type === 'fiches' && viewState.value.destination) openUpload(viewState.value.destination)
}

const arboOpen = ref(false)
const arboPreset = ref<{ type: CategoryType; parentId: number } | null>(null)
function openArbo(preset: { type: CategoryType; parentId: number } | null) {
  arboPreset.value = preset
  arboOpen.value = true
}

const ficheOpenId = ref<number | null>(null)
const ficheStartEdit = ref(false)
function openFiche(id: number, startEdit: boolean) {
  ficheOpenId.value = id
  ficheStartEdit.value = startEdit
}

const pdfState = ref<{ src: string; nom: string; chemin: string } | null>(null)
function onOpenPdf(src: string, nom: string, chemin: string) {
  pdfState.value = { src, nom, chemin }
}
</script>
