import type { AllData, FicheProduit } from './types'

/** Fonctions utilitaires de parcours de l'arborescence à 4 niveaux :
 *  Classeur > Famille (sous_classeur) > Sous-famille (sous_sous_classeur) > Sous-sous-famille (sous_sous_sous_classeur)
 */

export function findClasseur(db: AllData, id: number | null | undefined) {
  return id == null ? undefined : db.classeurs.find((c) => c.id_classeur === id)
}
export function findSousClasseur(db: AllData, id: number | null | undefined) {
  return id == null ? undefined : db.sous_classeurs.find((c) => c.id_sous_classeur === id)
}
export function findSousSousClasseur(db: AllData, id: number | null | undefined) {
  return id == null ? undefined : db.sous_sous_classeurs.find((c) => c.id_s_s_classeur === id)
}
export function findSousSousSousClasseur(db: AllData, id: number | null | undefined) {
  return id == null ? undefined : db.sous_sous_sous_classeurs.find((c) => c.id_s_s_s_classeur === id)
}

export function famillesDe(db: AllData, idClasseur: number) {
  return db.sous_classeurs.filter((sc) => sc.id_classeur === idClasseur)
}
export function sousFamillesDe(db: AllData, idSousClasseur: number) {
  return db.sous_sous_classeurs.filter((ssc) => ssc.id_sous_classeur === idSousClasseur)
}
export function sousSousFamillesDe(db: AllData, idSousSousClasseur: number) {
  return db.sous_sous_sous_classeurs.filter((sssc) => sssc.id_s_s_classeur === idSousSousClasseur)
}

/** Chaîne de labels menant à un dossier donné, du classeur jusqu'au niveau demandé (exclus lui-même). */
export function cheminClasseur(db: AllData, idClasseur: number) {
  const cl = findClasseur(db, idClasseur)
  return cl ? [cl.titre] : []
}
export function cheminFamille(db: AllData, idSousClasseur: number) {
  const sc = findSousClasseur(db, idSousClasseur)
  if (!sc) return []
  return [...cheminClasseur(db, sc.id_classeur), sc.titre]
}
export function cheminSousFamille(db: AllData, idSousSousClasseur: number) {
  const ssc = findSousSousClasseur(db, idSousSousClasseur)
  if (!ssc) return []
  return [...cheminFamille(db, ssc.id_sous_classeur), ssc.titre]
}
export function cheminSousSousFamille(db: AllData, idSousSousSousClasseur: number) {
  const sssc = findSousSousSousClasseur(db, idSousSousSousClasseur)
  if (!sssc) return []
  return [...cheminSousFamille(db, sssc.id_s_s_classeur), sssc.titre]
}

/** Chemin (libellés) menant à une fiche, selon le niveau où elle est classée. */
export function cheminFiche(db: AllData, f: FicheProduit): string[] {
  if (f.id_s_s_s_classeur != null) return cheminSousSousFamille(db, f.id_s_s_s_classeur)
  if (f.id_s_s_classeur != null) return cheminSousFamille(db, f.id_s_s_classeur)
  if (f.id_sous_classeur != null) return cheminFamille(db, f.id_sous_classeur)
  if (f.id_classeur != null) return cheminClasseur(db, f.id_classeur)
  return []
}

/** IDs de tous les sous_classeurs sous un classeur. */
function idsFamillesSous(db: AllData, idClasseur: number): number[] {
  return famillesDe(db, idClasseur).map((f) => f.id_sous_classeur)
}
/** IDs de tous les sous_sous_classeurs sous un classeur (via ses familles). */
function idsSousFamillesSousClasseur(db: AllData, idClasseur: number): number[] {
  const familles = idsFamillesSous(db, idClasseur)
  return db.sous_sous_classeurs.filter((s) => familles.includes(s.id_sous_classeur)).map((s) => s.id_s_s_classeur)
}
/** IDs de tous les sous_sous_sous_classeurs sous un classeur. */
function idsSousSousFamillesSousClasseur(db: AllData, idClasseur: number): number[] {
  const sousFamilles = idsSousFamillesSousClasseur(db, idClasseur)
  return db.sous_sous_sous_classeurs
    .filter((s) => sousFamilles.includes(s.id_s_s_classeur))
    .map((s) => s.id_s_s_s_classeur)
}
function idsSousSousFamillesSousFamille(db: AllData, idSousClasseur: number): number[] {
  const sousFamilles = sousFamillesDe(db, idSousClasseur).map((s) => s.id_s_s_classeur)
  return db.sous_sous_sous_classeurs
    .filter((s) => sousFamilles.includes(s.id_s_s_classeur))
    .map((s) => s.id_s_s_s_classeur)
}

/** Compte récursif du nombre de fiches classées dans un dossier et tous ses descendants. */
export function compterFichesClasseur(db: AllData, idClasseur: number): number {
  const familles = idsFamillesSous(db, idClasseur)
  const sousFamilles = idsSousFamillesSousClasseur(db, idClasseur)
  const sousSousFamilles = idsSousSousFamillesSousClasseur(db, idClasseur)
  return db.fiches.filter(
    (f) =>
      f.id_classeur === idClasseur ||
      (f.id_sous_classeur != null && familles.includes(f.id_sous_classeur)) ||
      (f.id_s_s_classeur != null && sousFamilles.includes(f.id_s_s_classeur)) ||
      (f.id_s_s_s_classeur != null && sousSousFamilles.includes(f.id_s_s_s_classeur)),
  ).length
}
export function compterFichesFamille(db: AllData, idSousClasseur: number): number {
  const sousFamilles = sousFamillesDe(db, idSousClasseur).map((s) => s.id_s_s_classeur)
  const sousSousFamilles = idsSousSousFamillesSousFamille(db, idSousClasseur)
  return db.fiches.filter(
    (f) =>
      f.id_sous_classeur === idSousClasseur ||
      (f.id_s_s_classeur != null && sousFamilles.includes(f.id_s_s_classeur)) ||
      (f.id_s_s_s_classeur != null && sousSousFamilles.includes(f.id_s_s_s_classeur)),
  ).length
}
export function compterFichesSousFamille(db: AllData, idSousSousClasseur: number): number {
  const sousSousFamilles = sousSousFamillesDe(db, idSousSousClasseur).map((s) => s.id_s_s_s_classeur)
  return db.fiches.filter(
    (f) =>
      f.id_s_s_classeur === idSousSousClasseur ||
      (f.id_s_s_s_classeur != null && sousSousFamilles.includes(f.id_s_s_s_classeur)),
  ).length
}
export function compterFichesSousSousFamille(db: AllData, idSousSousSousClasseur: number): number {
  return db.fiches.filter((f) => f.id_s_s_s_classeur === idSousSousSousClasseur).length
}

export function fichesPourDestination(db: AllData, niveau: 1 | 2 | 3 | 4, id: number): FicheProduit[] {
  if (niveau === 1) return db.fiches.filter((f) => f.id_classeur === id)
  if (niveau === 2) return db.fiches.filter((f) => f.id_sous_classeur === id)
  if (niveau === 3) return db.fiches.filter((f) => f.id_s_s_classeur === id)
  return db.fiches.filter((f) => f.id_s_s_s_classeur === id)
}
