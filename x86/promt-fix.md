Ceci s'inspire du fork de la documentation du Rust Book par l'université Brown.
Dans ce fork, il existe une version interactive avec des utilisateurs : un dossier `quizzes` a été créé à la racine contenant des fichiers `.toml`.

Nous travaillons sur un fork du Rust Book réalisé par l'université Brown, qui diffère sensiblement du document original. Lis le contenu de l'introduction : `experiment-intro.md` / `experiment-intro.html`.

- Dans `src` se trouvent des fichiers qui ne sont pas pertinents pour notre objectif : ici nous nous concentrons sur x86-64, pas sur Rust.
- Un dossier `js-extensions` contient des packages copiés depuis la base ; vérifie si son contenu nous est utile.
- Corrige les fautes de grammaire et de vocabulaire. Relis chaque chapitre et chaque section ; si une reformulation est nécessaire, reformule sans perdre l'essence du contenu.
- Les fichiers `.toml` peuvent être mal écrits ou contenir des éléments inutiles pour notre contexte.
- Le dossier `quizzes` contient des fichiers `.toml` dans `quizzes/examples/` qui montrent la structure des questions ; ils sont tirés du livre Programming Rust.
- Vérifie les fichiers liés à l'outillage (`rust-toolchain`, `rustfmt.toml`, `makefile.toml`) et le dossier `ci/` pour déterminer s'ils sont utiles ou s'il faut les corriger.

Voir aussi : https://github.com/cognitive-engineering-lab/mdbook-quiz

Pour les quizzes, chaque dossier contenant des exercices inclut un fichier `quizzes-n.txt` qui contient les questions ; une ligne commençant par `R:` indique la réponse.

- Vérifie `style-guide.md` et corrige les éléments qui doivent l'être.
- Ajuste le contenu pour qu'il corresponde au message que nous voulons transmettre.
- Explique pourquoi certains mots sont en **gras**, en `code` ou en *italique* ; si possible, définis un modèle de phrasé (templates) et des exemples.
- Propose un modèle de titre et un template pour le contenu des fichiers.
- Corrige les parenthèses mal placées, les explications au mauvais endroit et les structures mal organisées.
- Ajoute les numérotations par chapitre (1.1, 1.2, etc.) là où elles manquent.
- Corrige les syntaxes d'insertion d'images si nécessaire.

Si tu veux, je peux appliquer automatiquement ces corrections aux fichiers du dossier `x86` (en commençant par `promt-fix.md`) ou préparer une pull request avec les changements proposés.

