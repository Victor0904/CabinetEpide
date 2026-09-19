import { WebviewWindow } from '@tauri-apps/api/webviewWindow'

/** Ouvre une fenêtre de projection (mosaïque / présentation plein écran) pour un panier. */
export function openProjection(numero: 1 | 2, ids: number[]) {
  const label = `projection-${numero}-${Date.now()}`
  const url = `index.html#/projection/${numero}?ids=${ids.join(',')}`
  new WebviewWindow(label, {
    url,
    title: `Projection — Panier ${numero}`,
    width: 1280,
    height: 800,
  })
}

/** Ouvre la fenêtre d'impression du rapport de consultation (fiches des deux paniers). */
export function openConsultation(ids: number[]) {
  const label = `consultation-${Date.now()}`
  const url = `index.html#/consultation?ids=${ids.join(',')}`
  new WebviewWindow(label, {
    title: 'Consultation',
    url,
    width: 900,
    height: 700,
  })
}
