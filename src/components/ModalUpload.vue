<template>
  <div class="modal-overlay" @click.self="$emit('close')">
    <div class="modal-content">
      <div class="modal-header">
        <h2>Nouvelle Fiche Médicale</h2>
        <button class="btn-fermer" @click="$emit('close')">&times;</button>
      </div>
      <form @submit.prevent="onSubmit">
        <div class="form-group">
          <label>Titre de la fiche</label>
          <input v-model="titre" type="text" required />
        </div>
        <div class="form-group">
          <label>Destination</label>
          <DestinationCascade ref="cascadeRef" @change="destination = $event" />
        </div>
        <div class="form-group">
          <label>Commentaire (optionnel)</label>
          <textarea v-model="descriptif" rows="2"></textarea>
        </div>
        <div class="form-group">
          <label>Image</label>
          <input type="file" accept="image/*" required @change="onFile" />
        </div>
        <button type="submit" class="btn-enregistrer" style="width: 100%;" :disabled="submitting">
          {{ submitting ? 'Enregistrement…' : 'Enregistrer la fiche' }}
        </button>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import DestinationCascade from './DestinationCascade.vue'
import { api, fileToBase64 } from '../api'
import { chargerBaseDeDonnees, showToast } from '../store'
import type { Destination } from '../types'

const props = defineProps<{ preset?: Destination | null }>()
const emit = defineEmits<{ close: [] }>()

const titre = ref('')
const descriptif = ref('')
const destination = ref<Destination | null>(null)
const file = ref<File | null>(null)
const submitting = ref(false)
const cascadeRef = ref<InstanceType<typeof DestinationCascade> | null>(null)

onMounted(() => {
  if (props.preset) cascadeRef.value?.setPreset(props.preset)
})

function onFile(e: Event) {
  file.value = (e.target as HTMLInputElement).files?.[0] ?? null
}

async function onSubmit() {
  if (!destination.value) {
    showToast('⚠️ Veuillez choisir une destination pour la fiche.', true)
    return
  }
  if (!file.value) {
    showToast('⚠️ Veuillez choisir une image.', true)
    return
  }
  submitting.value = true
  try {
    const base64 = await fileToBase64(file.value)
    await api.uploadFiche({
      titre: titre.value,
      descriptif: descriptif.value,
      niveau: String(destination.value.niveau),
      destination_id: destination.value.id,
      file_name: file.value.name,
      file_base64: base64,
    })
    showToast('✅ Fiche ajoutée avec succès !')
    await chargerBaseDeDonnees()
    emit('close')
  } catch (e) {
    showToast('❌ Erreur : ' + String(e), true)
  } finally {
    submitting.value = false
  }
}
</script>
