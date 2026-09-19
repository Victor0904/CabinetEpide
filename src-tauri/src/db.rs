use crate::paths;
use rusqlite::Connection;
use std::sync::Mutex;

pub struct DbState(pub Mutex<Connection>);

const SCHEMA: &str = r#"
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS classeurs (
    id_classeur INTEGER PRIMARY KEY AUTOINCREMENT,
    titre TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sous_classeurs (
    id_sous_classeur INTEGER PRIMARY KEY AUTOINCREMENT,
    id_classeur INTEGER NOT NULL REFERENCES classeurs(id_classeur) ON DELETE CASCADE,
    titre TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sous_sous_classeurs (
    id_s_s_classeur INTEGER PRIMARY KEY AUTOINCREMENT,
    id_sous_classeur INTEGER NOT NULL REFERENCES sous_classeurs(id_sous_classeur) ON DELETE CASCADE,
    titre TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sous_sous_sous_classeurs (
    id_s_s_s_classeur INTEGER PRIMARY KEY AUTOINCREMENT,
    id_s_s_classeur INTEGER NOT NULL REFERENCES sous_sous_classeurs(id_s_s_classeur) ON DELETE CASCADE,
    titre TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fiches_produits (
    id_fiche_produit INTEGER PRIMARY KEY AUTOINCREMENT,
    id_classeur INTEGER REFERENCES classeurs(id_classeur) ON DELETE CASCADE,
    id_sous_classeur INTEGER REFERENCES sous_classeurs(id_sous_classeur) ON DELETE CASCADE,
    id_s_s_classeur INTEGER REFERENCES sous_sous_classeurs(id_s_s_classeur) ON DELETE CASCADE,
    id_s_s_s_classeur INTEGER REFERENCES sous_sous_sous_classeurs(id_s_s_s_classeur) ON DELETE CASCADE,
    titre TEXT NOT NULL,
    descriptif TEXT,
    photo_1 TEXT,
    photo_2 TEXT,
    synonymes TEXT,
    tags TEXT,
    epidemiologie TEXT,
    physiopathologie TEXT,
    clinique TEXT,
    paraclinique TEXT,
    traitement TEXT,
    prevention TEXT,
    bibliographie TEXT
);

CREATE TABLE IF NOT EXISTS fiche_medias (
    id_media INTEGER PRIMARY KEY AUTOINCREMENT,
    id_fiche_produit INTEGER NOT NULL REFERENCES fiches_produits(id_fiche_produit) ON DELETE CASCADE,
    type TEXT NOT NULL,
    chemin TEXT NOT NULL,
    nom_original TEXT
);

CREATE TABLE IF NOT EXISTS parametres (
    cle TEXT PRIMARY KEY,
    valeur TEXT
);
"#;

pub fn init_connection() -> Connection {
    paths::ensure_dirs();
    let conn = Connection::open(paths::db_path()).expect("impossible d'ouvrir la base SQLite");
    conn.execute_batch(SCHEMA).expect("initialisation du schéma");
    conn
}
