import re
import sqlite3
import sys

DUMP_PATH = r"C:\xampp\htdocs\cabinet_epidemiologie\cabinet_epidemiologie.sql"


def extract_block(text, header):
    start = text.index(header)
    end = text.index(";", start)
    return text[start:end]


def unescape(s):
    return s.replace("\\'", "'").replace('\\"', '"').replace("\\\\", "\\")


def parse_pair_string(block):
    # (id, id2, 'titre')
    pat = re.compile(r"\((\d+),\s*(\d+),\s*'((?:[^'\\]|\\.)*)'\)")
    return [(int(a), int(b), unescape(c)) for a, b, c in pat.findall(block)]


def parse_id_string_null(block):
    # (id, 'titre', NULL)
    pat = re.compile(r"\((\d+),\s*'((?:[^'\\]|\\.)*)',\s*NULL\)")
    return [(int(a), unescape(b)) for a, b in pat.findall(block)]


def main():
    with open(DUMP_PATH, "r", encoding="utf-8") as f:
        text = f.read()

    classeurs_block = extract_block(text, "INSERT INTO `classeurs`")
    sous_classeurs_block = extract_block(text, "INSERT INTO `sous_classeurs`")
    sous_sous_classeurs_block = extract_block(text, "INSERT INTO `sous_sous_classeurs`")

    classeurs = parse_id_string_null(classeurs_block)
    sous_classeurs = parse_pair_string(sous_classeurs_block)
    sous_sous_classeurs = parse_pair_string(sous_sous_classeurs_block)

    print(f"classeurs: {len(classeurs)} lignes")
    print(f"sous_classeurs: {len(sous_classeurs)} lignes")
    print(f"sous_sous_classeurs: {len(sous_sous_classeurs)} lignes")

    if "--check-only" in sys.argv:
        for row in classeurs:
            print(row)
        return

    targets = sys.argv[1:]
    for db_path in targets:
        print(f"\n-> Import dans {db_path}")
        conn = sqlite3.connect(db_path)
        conn.execute("PRAGMA foreign_keys = ON")
        conn.executemany(
            "INSERT OR IGNORE INTO classeurs (id_classeur, titre) VALUES (?, ?)", classeurs
        )
        conn.executemany(
            "INSERT OR IGNORE INTO sous_classeurs (id_sous_classeur, id_classeur, titre) VALUES (?, ?, ?)",
            sous_classeurs,
        )
        conn.executemany(
            "INSERT OR IGNORE INTO sous_sous_classeurs (id_s_s_classeur, id_sous_classeur, titre) VALUES (?, ?, ?)",
            sous_sous_classeurs,
        )
        conn.execute(
            """INSERT INTO parametres (cle, valeur) VALUES ('paniers_actif', '0')
               ON CONFLICT(cle) DO UPDATE SET valeur = excluded.valeur"""
        )
        conn.commit()

        n1 = conn.execute("SELECT COUNT(*) FROM classeurs").fetchone()[0]
        n2 = conn.execute("SELECT COUNT(*) FROM sous_classeurs").fetchone()[0]
        n3 = conn.execute("SELECT COUNT(*) FROM sous_sous_classeurs").fetchone()[0]
        print(f"   classeurs en base: {n1}, sous_classeurs: {n2}, sous_sous_classeurs: {n3}")
        conn.close()


if __name__ == "__main__":
    main()
