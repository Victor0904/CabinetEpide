import { computed, reactive, ref } from 'vue'
import { api } from './api'
import { famillesDe, sousFamillesDe, sousSousFamillesDe } from './tree'
import type { AllData, FicheProduit } from './types'

export const db = reactive<AllData>({
  classeurs: [],
  sous_classeurs: [],
  sous_sous_classeurs: [],
  sous_sous_sous_classeurs: [],
  fiches: [],
  parametres: {},
})

export const paniers = reactive<Record<1 | 2, FicheProduit[]>>({ 1: [], 2: [] })
export const panierActif = ref(true)
export const modeAdmin = ref(false)

export type Nav =
  | { kind: 'home' }
  | { kind: 'families'; id: number }
  | { kind: 'subfamilies'; id: number }
  | { kind: 'subsubfamilies'; id: number }
  | { kind: 'fiches'; niveau: 1 | 2 | 3 | 4; id: number }
  | { kind: 'all_fiches' }

export const nav = ref<Nav>({ kind: 'home' })

export const toast = reactive<{ message: string; error: boolean; visible: boolean }>({
  message: '',
  error: false,
  visible: false,
})
let toastTimer: number | undefined
export function showToast(message: string, error = false) {
  toast.message = message
  toast.error = error
  toast.visible = true
  if (toastTimer) window.clearTimeout(toastTimer)
  toastTimer = window.setTimeout(() => {
    toast.visible = false
  }, 3000)
}

export async function chargerBaseDeDonnees() {
  try {
    const data = await api.getAllData()
    Object.assign(db, data)
    panierActif.value = (db.parametres.paniers_actif ?? '1') === '1'
  } catch (e) {
    showToast('❌ Erreur de connexion à la base : ' + String(e), true)
  }
}

export function togglePaniers() {
  panierActif.value = !panierActif.value
  db.parametres.paniers_actif = panierActif.value ? '1' : '0'
  api.setParametre('paniers_actif', db.parametres.paniers_actif)
}

// ── Navigation ────────────────────────────────────────────────

export function irAccueil() {
  nav.value = { kind: 'home' }
}
export function irClasseur(id: number) {
  nav.value = famillesDe(db, id).length ? { kind: 'families', id } : { kind: 'fiches', niveau: 1, id }
}
export function irFamille(id: number) {
  nav.value = sousFamillesDe(db, id).length ? { kind: 'subfamilies', id } : { kind: 'fiches', niveau: 2, id }
}
export function irSousFamille(id: number) {
  nav.value = sousSousFamillesDe(db, id).length ? { kind: 'subsubfamilies', id } : { kind: 'fiches', niveau: 3, id }
}
export function irSousSousFamille(id: number) {
  nav.value = { kind: 'fiches', niveau: 4, id }
}
export function irToutesFiches() {
  nav.value = { kind: 'all_fiches' }
}
export function irFiches(niveau: 1 | 2 | 3 | 4, id: number) {
  nav.value = { kind: 'fiches', niveau, id }
}

// ── Recherche ─────────────────────────────────────────────────

export const searchTerm = ref('')
export const searchFilters = reactive({ titre: true, synonymes: true, descriptif: true })

export const searchResults = computed<FicheProduit[] | null>(() => {
  const terme = searchTerm.value.trim().toLowerCase()
  if (!terme) return null
  const champs: ('titre' | 'synonymes' | 'descriptif')[] = []
  if (searchFilters.titre) champs.push('titre')
  if (searchFilters.synonymes) champs.push('synonymes')
  if (searchFilters.descriptif) champs.push('descriptif')
  const actifs: ('titre' | 'synonymes' | 'descriptif')[] = champs.length ? champs : ['titre']
  return db.fiches.filter((f) => actifs.some((c) => (f[c] ?? '').toLowerCase().includes(terme)))
})

// ── Paniers ───────────────────────────────────────────────────

export function ajouterAuPanier(fiche: FicheProduit, numero: 1 | 2) {
  if (paniers[numero].length >= 4) {
    showToast('⚠️ Maximum 4 fiches par panier.', true)
    return
  }
  if (paniers[numero].some((f) => f.id_fiche_produit === fiche.id_fiche_produit)) {
    showToast('ℹ️ Déjà dans le panier.')
    return
  }
  paniers[numero] = [...paniers[numero], fiche]
  showToast(`✅ "${fiche.titre}" → Panier ${numero}`)
}

export function retirerDuPanier(idFiche: number, numero: 1 | 2) {
  paniers[numero] = paniers[numero].filter((f) => f.id_fiche_produit !== idFiche)
}

export function clearPanier(numero: 1 | 2) {
  paniers[numero] = []
}

export function estDansUnPanier(idFiche: number): (1 | 2)[] {
  const res: (1 | 2)[] = []
  if (paniers[1].some((f) => f.id_fiche_produit === idFiche)) res.push(1)
  if (paniers[2].some((f) => f.id_fiche_produit === idFiche)) res.push(2)
  return res
}

export function retirerFicheDeTousLesPaniers(idFiche: number) {
  paniers[1] = paniers[1].filter((f) => f.id_fiche_produit !== idFiche)
  paniers[2] = paniers[2].filter((f) => f.id_fiche_produit !== idFiche)
}
