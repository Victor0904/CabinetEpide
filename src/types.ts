export interface Classeur {
  id_classeur: number
  titre: string
}

export interface SousClasseur {
  id_sous_classeur: number
  id_classeur: number
  titre: string
}

export interface SousSousClasseur {
  id_s_s_classeur: number
  id_sous_classeur: number
  titre: string
}

export interface SousSousSousClasseur {
  id_s_s_s_classeur: number
  id_s_s_classeur: number
  titre: string
}

export interface FicheProduit {
  id_fiche_produit: number
  id_classeur: number | null
  id_sous_classeur: number | null
  id_s_s_classeur: number | null
  id_s_s_s_classeur: number | null
  titre: string
  descriptif: string | null
  photo_1: string | null
  photo_2: string | null
  synonymes: string | null
  tags: string | null
  epidemiologie: string | null
  physiopathologie: string | null
  clinique: string | null
  paraclinique: string | null
  traitement: string | null
  prevention: string | null
  bibliographie: string | null
}

export interface FicheMedia {
  id_media: number
  id_fiche_produit: number
  type: 'image' | 'pdf'
  chemin: string
  nom_original: string | null
}

export interface FicheDetail extends FicheProduit {
  medias: FicheMedia[]
}

export interface AllData {
  classeurs: Classeur[]
  sous_classeurs: SousClasseur[]
  sous_sous_classeurs: SousSousClasseur[]
  sous_sous_sous_classeurs: SousSousSousClasseur[]
  fiches: FicheProduit[]
  parametres: Record<string, string>
}

export type CategoryType = 'classeur' | 'famille' | 'sous_famille' | 'sous_sous_famille'

/** Une destination pointe vers un des 4 niveaux d'arborescence. */
export interface Destination {
  niveau: 1 | 2 | 3 | 4
  id: number
}
