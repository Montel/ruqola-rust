Pour créer la premiere fois:
find -name \*.slint | xargs /travail/kdab/git/slint/./target/debug/slint-tr-extractor -o ruqola-rust.pot

=> slint-tr-extractor est généré dans le source slint (peut être installé après)

Pour mettre à jour il faut lancer le script update_translations.sh
