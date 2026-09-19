<template>
  <div class="modal-overlay" @click.self="$emit('close')">
    <div class="modal-content" style="max-width:500px;">
      <div class="modal-header">
        <h2>Gérer les dossiers</h2>
        <button class="btn-fermer" @click="$emit('close')">&times;</button>
      </div>

      <div class="arbo-tabs">
        <button class="arbo-tab" :class="{ actif: tab === 'creer' }" @click="tab = 'creer'">➕ Créer</button>
        <button class="arbo-tab" :class="{ actif: tab === 'renommer' }" @click="tab = 'renommer'">✏️ Renommer</button>
        <button class="arbo-tab" :class="{ actif: tab === 'supprimer' }" @click="tab = 'supprimer'">🗑️ Supprimer</button>
      </div>

      <!-- Créer -->
      <form v-if="tab === 'creer'" @submit.prevent="onCreer">
        <div class="form-group">
          <label>Que voulez-vous créer ?</label>
          <select v-model="creerType">
            <option value="classeur">1. Un grand Classeur principal</option>
            <option value="famille">2. Une Famille (dans un Classeur)</option>
            <option value="sous_famille">3. Une Sous-famille (dans une Famille)</option>
            <option value="sous_sous_famille">4. Une Sous-sous-famille (dans une Sous-famille)</option>
          </select>
        </div>
        <div v-if="creerType === 'famille'" class="form-group">
          <label>Dans quel Classeur ?</label>
          <select v-model.number="creerParentClasseur">
            <option v-for="c in db.classeurs" :key="c.id_classeur" :value="c.id_classeur">{{ c.titre }}</option>
          </select>
        </div>
        <div v-if="creerType === 'sous_famille'" class="form-group">
          <label>Dans quelle Famille ?</label>
          <select v-model.number="creerParentFamille">
            <option v-for="c in db.sous_classeurs" :key="c.id_sous_classeur" :value="c.id_sous_classeur">{{ c.titre }}</option>
          </select>
        </div>
        <div v-if="creerType === 'sous_sous_famille'" class="form-group">
          <label>Dans quelle Sous-famille ?</label>
          <select v-model.number="creerParentSousFamille">
            <option v-for="c in db.sous_sous_classeurs" :key="c.id_s_s_classeur" :value="c.id_s_s_classeur">
              {{ labelSousFamille(c) }}
            </option>
          </select>
        </div>
        <div class="form-group">
          <label>Titre du nouveau dossier</label>
          <input v-model="creerTitre" type="text" required />
        </div>
        <button type="submit" class="btn-enregistrer" style="width:100%;background:#3b82f6;">Créer</button>
      </form>

      <!-- Renommer -->
      <div v-else-if="tab === 'renommer'">
        <div class="form-group">
          <label>Niveau</label>
          <select v-model="renommerType" @change="onRenommerTypeChange">
            <option value="classeur">Classeur principal</option>
            <option value="famille">Famille</option>
            <option value="sous_famille">Sous-famille</option>
            <option value="sous_sous_famille">Sous-sous-famille</option>
          </select>
        </div>
        <div class="form-group">
          <label>Lequel ?</label>
          <select v-model.number="renommerCible" @change="onRenommerCibleChange">
            <option v-for="o in renommerOptions" :key="o.id" :value="o.id">{{ o.label }}</option>
          </select>
        </div>
        <div class="form-group">
          <label>Nouveau nom</label>
          <input v-model="renommerTitre" type="text" placeholder="Nouveau nom…" />
        </div>
        <button class="btn-enregistrer" style="width:100%;background:#3b82f6;" @click="onRenommer">✏️ Renommer</button>
      </div>

      <!-- Supprimer -->
      <div v-else>
        <div class="form-group">
          <label>Niveau</label>
          <select v-model="supprimerType">
            <option value="classeur">Classeur principal</option>
            <option value="famille">Famille</option>
            <option value="sous_famille">Sous-famille</option>
            <option value="sous_sous_famille">Sous-sous-famille</option>
          </select>
        </div>
        <div class="form-group">
          <label>Lequel ?</label>
          <select v-model.number="supprimerCible">
            <option v-for="o in supprimerOptions" :key="o.id" :value="o.id">{{ o.label }}</option>
          </select>
        </div>
        <p class="avertissement">⚠️ La suppression efface toutes les fiches et médias associés. Action irréversible.</p>
        <button class="btn-enregistrer" style="width:100%;background:#dc2626;" @click="onSupprimer">
          🗑️ Supprimer définitivement
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { db, chargerBaseDeDonnees, showToast } from '../store'
import { api } from '../api'
import { confirmDialog } from '../dialogs'
import type { CategoryType, SousSousClasseur } from '../types'

const props = defineProps<{ preset?: { type: CategoryType; parentId: number } | null }>()
const emit = defineEmits<{ close: [] }>()

const tab = ref<'creer' | 'renommer' | 'supprimer'>('creer')

// ── Créer ──────────────────────────────────────────────────────
const creerType = ref<CategoryType>('classeur')
const creerParentClasseur = ref<number | ''>('')
const creerParentFamille = ref<number | ''>('')
const creerParentSousFamille = ref<number | ''>('')
const creerTitre = ref('')

function labelSousFamille(ssc: SousSousClasseur) {
  const sc = db.sous_classeurs.find((s) => s.id_sous_classeur === ssc.id_sous_classeur)
  return (sc ? sc.titre + ' › ' : '') + ssc.titre
}

onMounted(() => {
  if (db.classeurs.length) creerParentClasseur.value = db.classeurs[0].id_classeur
  if (db.sous_classeurs.length) creerParentFamille.value = db.sous_classeurs[0].id_sous_classeur
  if (db.sous_sous_classeurs.length) creerParentSousFamille.value = db.sous_sous_classeurs[0].id_s_s_classeur
  if (props.preset) {
    tab.value = 'creer'
    creerType.value = props.preset.type
    if (props.preset.type === 'famille') creerParentClasseur.value = props.preset.parentId
    else if (props.preset.type === 'sous_famille') creerParentFamille.value = props.preset.parentId
    else if (props.preset.type === 'sous_sous_famille') creerParentSousFamille.value = props.preset.parentId
  }
})

async function onCreer() {
  const titre = creerTitre.value.trim()
  if (!titre) {
    showToast('⚠️ Le titre ne peut pas être vide.', true)
    return
  }
  const parentId =
    creerType.value === 'famille'
      ? (creerParentClasseur.value as number)
      : creerType.value === 'sous_famille'
        ? (creerParentFamille.value as number)
        : creerType.value === 'sous_sous_famille'
          ? (creerParentSousFamille.value as number)
          : null
  try {
    await api.addCategory({ type: creerType.value, titre, parent_id: parentId })
    showToast('✅ Dossier créé avec succès !')
    creerTitre.value = ''
    await chargerBaseDeDonnees()
    emit('close')
  } catch (e) {
    showToast('❌ Erreur : ' + String(e), true)
  }
}

// ── Options partagées (niveau -> liste {id,label}) ──────────────
function optionsPourType(type: CategoryType): { id: number; label: string }[] {
  if (type === 'classeur') return db.classeurs.map((c) => ({ id: c.id_classeur, label: c.titre }))
  if (type === 'famille') return db.sous_classeurs.map((c) => ({ id: c.id_sous_classeur, label: c.titre }))
  if (type === 'sous_famille')
    return db.sous_sous_classeurs.map((c) => ({ id: c.id_s_s_classeur, label: labelSousFamille(c) }))
  return db.sous_sous_sous_classeurs.map((sssc) => {
    const ssc = db.sous_sous_classeurs.find((s) => s.id_s_s_classeur === sssc.id_s_s_classeur)
    const sc = ssc ? db.sous_classeurs.find((s) => s.id_sous_classeur === ssc.id_sous_classeur) : undefined
    const prefix = [sc?.titre, ssc?.titre].filter(Boolean).join(' › ')
    return { id: sssc.id_s_s_s_classeur, label: (prefix ? prefix + ' › ' : '') + sssc.titre }
  })
}

// ── Renommer ─────────────────────────────────────────────────
const renommerType = ref<CategoryType>('classeur')
const renommerCible = ref<number | ''>('')
const renommerTitre = ref('')
const renommerOptions = computed(() => optionsPourType(renommerType.value))

function onRenommerTypeChange() {
  renommerCible.value = renommerOptions.value[0]?.id ?? ''
  onRenommerCibleChange()
}
function onRenommerCibleChange() {
  const opt = renommerOptions.value.find((o) => o.id === renommerCible.value)
  renommerTitre.value = opt ? opt.label.split(' › ').pop()! : ''
}
watch(renommerOptions, () => {
  if (!renommerOptions.value.some((o) => o.id === renommerCible.value)) onRenommerTypeChange()
})

async function onRenommer() {
  const titre = renommerTitre.value.trim()
  if (!titre || !renommerCible.value) {
    showToast('⚠️ Le nom est vide.', true)
    return
  }
  try {
    await api.renameCategory({ type: renommerType.value, id: renommerCible.value, titre })
    showToast('✅ Renommé avec succès.')
    await chargerBaseDeDonnees()
    onRenommerTypeChange()
  } catch (e) {
    showToast('❌ ' + String(e), true)
  }
}

// ── Supprimer ────────────────────────────────────────────────
const supprimerType = ref<CategoryType>('classeur')
const supprimerCible = ref<number | ''>('')
const supprimerOptions = computed(() => optionsPourType(supprimerType.value))
watch(
  supprimerOptions,
  () => {
    if (!supprimerOptions.value.some((o) => o.id === supprimerCible.value))
      supprimerCible.value = supprimerOptions.value[0]?.id ?? ''
  },
  { immediate: true },
)

async function onSupprimer() {
  if (!supprimerCible.value) return
  const opt = supprimerOptions.value.find((o) => o.id === supprimerCible.value)
  const ok = await confirmDialog(
    `Supprimer "${opt?.label ?? ''}" et TOUT son contenu (fiches, médias) ? Cette action est irréversible.`,
    true,
  )
  if (!ok) return
  try {
    await api.deleteCategory({ type: supprimerType.value, id: supprimerCible.value })
    showToast('🗑️ Supprimé avec succès.')
    await chargerBaseDeDonnees()
  } catch (e) {
    showToast('❌ ' + String(e), true)
  }
}
</script>
