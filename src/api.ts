import { invoke } from '@tauri-apps/api/core'
import type { AllData, CategoryType, FicheDetail, FicheMedia } from './types'

export interface UpdateFichePayload {
  id_fiche_produit: number
  titre: string
  descriptif: string | null
  synonymes: string | null
  id_classeur: number | null
  id_sous_classeur: number | null
  id_s_s_classeur: number | null
  id_s_s_s_classeur: number | null
}

export const api = {
  getAllData: () => invoke<AllData>('get_all_data'),

  getFicheDetail: (id: number) => invoke<FicheDetail>('get_fiche_detail', { id }),

  updateFiche: (payload: UpdateFichePayload) => invoke<void>('update_fiche', { payload }),

  uploadFiche: (args: {
    titre: string
    descriptif: string
    niveau: string
    destination_id: number
    file_name: string
    file_base64: string
  }) => invoke<number>('upload_fiche', args),

  deleteFiche: (id: number) => invoke<void>('delete_fiche', { id }),

  uploadMedia: (args: { id_fiche_produit: number; file_name: string; file_base64: string }) =>
    invoke<FicheMedia>('upload_media', args),

  deleteMedia: (id_media: number) => invoke<void>('delete_media', { id_media }),

  readMedia: (chemin: string) => invoke<string>('read_media', { chemin }),

  getMediaPath: (chemin: string) => invoke<string>('get_media_path', { chemin }),

  addCategory: (args: { type: CategoryType; titre: string; parent_id: number | null }) =>
    invoke<void>('add_category', args),

  renameCategory: (args: { type: CategoryType; id: number; titre: string }) =>
    invoke<void>('rename_category', args),

  deleteCategory: (args: { type: CategoryType; id: number }) => invoke<void>('delete_category', args),

  setParametre: (cle: string, valeur: string) => invoke<void>('set_parametre', { cle, valeur }),

  exportTextFile: (path: string, content: string) => invoke<void>('export_text_file', { path, content }),
}

/** Lit un fichier File du navigateur et retourne son contenu encodé en base64 (sans préfixe data:). */
export function fileToBase64(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.onload = () => {
      const result = reader.result as string
      const base64 = result.split(',')[1] ?? ''
      resolve(base64)
    }
    reader.onerror = () => reject(reader.error)
    reader.readAsDataURL(file)
  })
}
