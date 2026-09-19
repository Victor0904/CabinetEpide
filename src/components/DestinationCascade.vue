<template>
  <div>
    <div class="dest-cascade">
      <select v-model.number="niv1" @change="onNiv1Change">
        <option value="">— Choisir un classeur —</option>
        <option v-for="c in db.classeurs" :key="c.id_classeur" :value="c.id_classeur">{{ c.titre }}</option>
      </select>
      <select v-if="niv2Options.length" v-model.number="niv2" @change="onNiv2Change">
        <option value="">— Choisir une famille —</option>
        <option v-for="c in niv2Options" :key="c.id_sous_classeur" :value="c.id_sous_classeur">{{ c.titre }}</option>
      </select>
      <select v-if="niv3Options.length" v-model.number="niv3" @change="onNiv3Change">
        <option value="">— Choisir une sous-famille —</option>
        <option v-for="c in niv3Options" :key="c.id_s_s_classeur" :value="c.id_s_s_classeur">{{ c.titre }}</option>
      </select>
      <select v-if="niv4Options.length" v-model.number="niv4" @change="onNiv4Change">
        <option value="">— Choisir une sous-sous-famille —</option>
        <option v-for="c in niv4Options" :key="c.id_s_s_s_classeur" :value="c.id_s_s_s_classeur">{{ c.titre }}</option>
      </select>
    </div>
    <div v-if="previewText" class="dest-preview">📍 {{ previewText }}</div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { db } from '../store'
import {
  cheminClasseur,
  cheminFamille,
  cheminSousFamille,
  cheminSousSousFamille,
  famillesDe,
  sousFamillesDe,
  sousSousFamillesDe,
} from '../tree'
import type { Destination } from '../types'

const emit = defineEmits<{ change: [v: Destination | null] }>()

const niv1 = ref<number | ''>('')
const niv2 = ref<number | ''>('')
const niv3 = ref<number | ''>('')
const niv4 = ref<number | ''>('')
const current = ref<Destination | null>(null)

const niv2Options = computed(() => (niv1.value ? famillesDe(db, niv1.value as number) : []))
const niv3Options = computed(() => (niv2.value ? sousFamillesDe(db, niv2.value as number) : []))
const niv4Options = computed(() => (niv3.value ? sousSousFamillesDe(db, niv3.value as number) : []))

function setAndEmit(v: Destination | null) {
  current.value = v
  emit('change', v)
}

function onNiv1Change() {
  niv2.value = ''
  niv3.value = ''
  niv4.value = ''
  if (!niv1.value) return setAndEmit(null)
  if (famillesDe(db, niv1.value as number).length === 0) setAndEmit({ niveau: 1, id: niv1.value as number })
  else setAndEmit(null)
}
function onNiv2Change() {
  niv3.value = ''
  niv4.value = ''
  if (!niv2.value) return setAndEmit(null)
  if (sousFamillesDe(db, niv2.value as number).length === 0) setAndEmit({ niveau: 2, id: niv2.value as number })
  else setAndEmit(null)
}
function onNiv3Change() {
  niv4.value = ''
  if (!niv3.value) return setAndEmit(null)
  if (sousSousFamillesDe(db, niv3.value as number).length === 0) setAndEmit({ niveau: 3, id: niv3.value as number })
  else setAndEmit(null)
}
function onNiv4Change() {
  if (!niv4.value) return setAndEmit(null)
  setAndEmit({ niveau: 4, id: niv4.value as number })
}

const previewText = computed(() => {
  const d = current.value
  if (!d) return ''
  if (d.niveau === 1) return cheminClasseur(db, d.id).join(' › ')
  if (d.niveau === 2) return cheminFamille(db, d.id).join(' › ')
  if (d.niveau === 3) return cheminSousFamille(db, d.id).join(' › ')
  return cheminSousSousFamille(db, d.id).join(' › ')
})

/** Pré-remplit les selects à partir d'une destination existante (édition / pré-sélection depuis un dossier). */
function setPreset(d: Destination | null) {
  niv1.value = ''
  niv2.value = ''
  niv3.value = ''
  niv4.value = ''
  if (!d) {
    setAndEmit(null)
    return
  }
  if (d.niveau === 1) {
    niv1.value = d.id
  } else if (d.niveau === 2) {
    const sc = db.sous_classeurs.find((s) => s.id_sous_classeur === d.id)
    if (sc) {
      niv1.value = sc.id_classeur
      niv2.value = d.id
    }
  } else if (d.niveau === 3) {
    const ssc = db.sous_sous_classeurs.find((s) => s.id_s_s_classeur === d.id)
    const sc = ssc ? db.sous_classeurs.find((s) => s.id_sous_classeur === ssc.id_sous_classeur) : undefined
    if (sc && ssc) {
      niv1.value = sc.id_classeur
      niv2.value = sc.id_sous_classeur
      niv3.value = d.id
    }
  } else {
    const sssc = db.sous_sous_sous_classeurs.find((s) => s.id_s_s_s_classeur === d.id)
    const ssc = sssc ? db.sous_sous_classeurs.find((s) => s.id_s_s_classeur === sssc.id_s_s_classeur) : undefined
    const sc = ssc ? db.sous_classeurs.find((s) => s.id_sous_classeur === ssc.id_sous_classeur) : undefined
    if (sc && ssc && sssc) {
      niv1.value = sc.id_classeur
      niv2.value = sc.id_sous_classeur
      niv3.value = ssc.id_s_s_classeur
      niv4.value = d.id
    }
  }
  setAndEmit(d)
}

defineExpose({ setPreset })
</script>
