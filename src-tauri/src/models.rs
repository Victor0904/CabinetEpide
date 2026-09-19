use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Classeur {
    pub id_classeur: i64,
    pub titre: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SousClasseur {
    pub id_sous_classeur: i64,
    pub id_classeur: i64,
    pub titre: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SousSousClasseur {
    pub id_s_s_classeur: i64,
    pub id_sous_classeur: i64,
    pub titre: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SousSousSousClasseur {
    pub id_s_s_s_classeur: i64,
    pub id_s_s_classeur: i64,
    pub titre: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FicheProduit {
    pub id_fiche_produit: i64,
    pub id_classeur: Option<i64>,
    pub id_sous_classeur: Option<i64>,
    pub id_s_s_classeur: Option<i64>,
    pub id_s_s_s_classeur: Option<i64>,
    pub titre: String,
    pub descriptif: Option<String>,
    pub photo_1: Option<String>,
    pub photo_2: Option<String>,
    pub synonymes: Option<String>,
    pub tags: Option<String>,
    pub epidemiologie: Option<String>,
    pub physiopathologie: Option<String>,
    pub clinique: Option<String>,
    pub paraclinique: Option<String>,
    pub traitement: Option<String>,
    pub prevention: Option<String>,
    pub bibliographie: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FicheMedia {
    pub id_media: i64,
    pub id_fiche_produit: i64,
    #[serde(rename = "type")]
    pub type_: String,
    pub chemin: String,
    pub nom_original: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FicheDetail {
    #[serde(flatten)]
    pub fiche: FicheProduit,
    pub medias: Vec<FicheMedia>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AllData {
    pub classeurs: Vec<Classeur>,
    pub sous_classeurs: Vec<SousClasseur>,
    pub sous_sous_classeurs: Vec<SousSousClasseur>,
    pub sous_sous_sous_classeurs: Vec<SousSousSousClasseur>,
    pub fiches: Vec<FicheProduit>,
    pub parametres: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateFichePayload {
    pub id_fiche_produit: i64,
    pub titre: String,
    pub descriptif: Option<String>,
    pub synonymes: Option<String>,
    pub id_classeur: Option<i64>,
    pub id_sous_classeur: Option<i64>,
    pub id_s_s_classeur: Option<i64>,
    pub id_s_s_s_classeur: Option<i64>,
}
