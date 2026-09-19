<template>
  <header>
    <div class="btn-admin-toggle" @click="onToggleAdmin">
      <span>{{ modeAdmin ? '⬤ ADMIN' : 'ÉPIDÉMIOLOGIE' }}</span>
    </div>

    <div class="search-bar">
      <input v-model="searchTerm" type="text" placeholder="Rechercher une pathologie, un toxique..." />
      <div class="search-filters">
        <button
          class="filter-chip"
          :class="{ actif: searchFilters.titre }"
          @click="searchFilters.titre = !searchFilters.titre"
        >
          Noms
        </button>
        <button
          class="filter-chip"
          :class="{ actif: searchFilters.synonymes }"
          @click="searchFilters.synonymes = !searchFilters.synonymes"
        >
          Synonymes
        </button>
        <button
          class="filter-chip"
          :class="{ actif: searchFilters.descriptif }"
          @click="searchFilters.descriptif = !searchFilters.descriptif"
        >
          Commentaire
        </button>
      </div>
    </div>

    <div class="header-actions">
      <button class="btn-header-action btn-vert" @click="$emit('open-upload')">+ Ajouter Fiche</button>
      <button class="btn-header-action btn-bleu" @click="$emit('open-arbo')">📁 Gérer Dossiers</button>
      <label class="panier-toggle" title="Activer / désactiver les paniers">
        <input id="chkPaniers" type="checkbox" :checked="panierActif" @change="togglePaniers" />
        <span class="panier-toggle-switch"></span>
        <span class="panier-toggle-text">🛒 Paniers</span>
      </label>
    </div>
  </header>
</template>

<script setup lang="ts">
import { watchEffect } from 'vue'
import { modeAdmin, panierActif, searchFilters, searchTerm, showToast, togglePaniers } from '../store'
import { promptDialog } from '../dialogs'

defineEmits<{ 'open-upload': []; 'open-arbo': [] }>()

async function onToggleAdmin() {
  if (modeAdmin.value) {
    modeAdmin.value = false
    showToast('Mode administrateur désactivé.')
    return
  }
  const mdp = await promptDialog('Mot de passe administrateur :')
  if (mdp === null) return
  if (mdp === 'Admin') {
    modeAdmin.value = true
    showToast('✅ Mode administrateur activé.')
  } else {
    showToast('❌ Mot de passe incorrect.', true)
  }
}

watchEffect(() => {
  document.body.classList.toggle('mode-admin', modeAdmin.value)
})
watchEffect(() => {
  document.body.classList.toggle('paniers-off', !panierActif.value)
})
</script>
