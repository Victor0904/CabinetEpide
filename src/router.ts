import { createRouter, createWebHashHistory } from 'vue-router'
import HomeView from './views/HomeView.vue'
import ProjectionView from './views/ProjectionView.vue'
import ConsultationView from './views/ConsultationView.vue'

// Hash history : l'app est servie comme des fichiers statiques par la
// webview (pas de serveur pour réécrire les routes), le hash fonctionne
// donc aussi bien en dev qu'une fois le bundle final construit.
export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', component: HomeView },
    { path: '/projection/:numero', component: ProjectionView },
    { path: '/consultation', component: ConsultationView },
  ],
})
