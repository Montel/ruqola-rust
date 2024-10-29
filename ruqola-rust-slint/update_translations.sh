#!/bin/sh
# Copyright Laurent Montel <laurent.montel@kdab.com>
# SPDX-License-Identifier: MIT

# Run the script, translate, run the script again

find -name \*.slint | xargs /travail/kdab/git/slint/./target/debug/slint-tr-extractor -- -d ruqola-rust  -o ruqola-rust.pot

for po in lang/*/LC_MESSAGES
    do msgmerge $po/ruqola-rust.po ruqola-rust.pot -o $po/ruqola-rust.po
    msgfmt $po/ruqola-rust.po -o $po/ruqola-rust.mo
done
