# MISSION
Tu es un ingénieur système Senior, expert en architecture LLM, Rust/TypeScript, open-source et CLI haute performance.
Ta mission est de concevoir, structurer et générer l'intégralité du projet pour **Hotmilk Code**, un CLI d'assistance au code 100% sur-mesure, local, privé et optimisé pour la distribution open-source sous licence **Apache 2.0** sur GitHub par **Hotmilk Studio**.

---

## 1. IDENTITÉ DE MARQUE & LICENCE (@biz & @design)
- **Nom du Produit** : `Hotmilk Code` (Nom du dépôt/binaire : `hotmilk-code`)
- **Éditeur / Organisation Parent** : Hotmilk Studio
- **Copyright** : Copyright (c) Hotmilk Studio
- **Licence** : Apache License 2.0 (Inclure le fichier `LICENSE` complet et les en-têtes de licence dans les fichiers sources importants).
- **Branding & Assets** :
  - Banner ASCII Art "Hotmilk Code" personnalisée affichée au lancement du CLI.
  - Dossier `assets/` contenant le logo et la bannière pour le dépôt GitHub (`assets/logo.png`, `assets/banner.png`).
  - Charte TUI : Couleurs chaleureuses et modernes (accents ambre/crème/orange, syntax highlighting très lisible).

---

## 2. VISION PRODUIT & ARCHITECTURE SYSTÈME (@ai & @biz)
- **Objectif** : Un CLI d'assistance au code **100% custom**, indépendant, rapide et axé sur la confidentialité. Basé sur les principes d'agents et de boucle ReAct du repo OpenAI Codex, mais réécrit intégralement pour **Hotmilk Code**.
- **Moteur d'Inférence Local (Backend)** :
  - **Abstraction Multi-Provider** : Support natif de **Ollama** (API REST/WebSocket) et **llama.cpp / llama-server** (bindings direct C++ ou HTTP server).
  - **Fallback & Switch** : Détection et bascule dynamique entre les moteurs locaux selon les ressources système (RAM / VRAM).
- **Fonctionnalités Clés** :
  - **Mode Chat Interactif (REPL)** : Session de discussion fluide dans le terminal avec complétion contextuelle.
  - **Mode Agentic Code Execution** : Auto-édition de fichiers (patch/diff), création de code et exécution de commandes shell sous supervision.
  - **Indexation Contextuelle (Local RAG)** : Parsing et vectorisation locale du workspace pour enrichir le prompt système.

---

## 3. STRUCTURATION DU DOSSIER POUR GITHUB (@design)
Génère l'arborescence complète, propre et prête pour une publication GitHub sous **Hotmilk Studio** :

.
├── .github/
│   ├── ISSUE_TEMPLATE/       # Templates d'issues Bug / Feature Request
│   └── workflows/            # GitHub Actions (CI, tests, release binaries, lint)
├── assets/
│   ├── logo.svg              # Logo officiel Hotmilk Studio / Hotmilk Code
│   └── banner.png            # Bannière pour le README GitHub
├── bin/                      # Point d'entrée binaire (hotmilk-code)
├── src/
│   ├── core/                 # Moteur principal Hotmilk Code
│   │   ├── agent.rs          # Boucle de décision agentique (ReAct loop)
│   │   ├── context.rs        # Gestionnaire de fenêtres de tokens et contexte
│   │   └── prompt.rs         # Prompts système optimisés
│   ├── providers/            # Connecteurs d'inférence locale
│   │   ├── traits.rs         # Interface Provider universelle
│   │   ├── ollama.rs         # Client Ollama API
│   │   └── llamacpp.rs       # Client llama.cpp HTTP/Native
│   ├── tools/                # Outils d'agent (FS, Shell, Search)
│   │   ├── fs_ops.rs         # Modifications de fichiers (Patch / Diff)
│   │   ├── shell.rs          # Exécution sécurisée de commandes
│   │   └── search.rs         # Recherche rapide dans le workspace (ripgrep)
│   ├── ui/                   # Interface utilisateur Terminal (TUI Hotmilk Code)
│   │   ├── banner.rs         # ASCII Art Banner & branding Hotmilk Studio
│   │   ├── components/       # Streamer, Diff Viewer, Dialogs
│   │   └── theme.rs          # Palette de couleurs Hotmilk Studio
│   └── config.rs             # Configuration locale (~/.config/hotmilk-code/config.toml)
├── templates/                # System prompts optimisés pour LLMs locaux (Qwen, DeepSeek-Coder, Llama)
├── tests/                    # Tests d'intégration et benchmarks
├── .gitignore
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md           # Guide de contribution open-source
├── LICENSE                   # Apache License 2.0
├── NOTICE                    # Mention légale Copyright (c) Hotmilk Studio
├── Cargo.toml / package.json # Config dépendances avec métadonnées Hotmilk Studio
└── README.md                 # README GitHub premium (badges, logo, installation, usage)

---

## 4. PLAN D'ACTION CLAUDE CODE (Livrables)

Exécute les étapes suivantes dans le workspace courant :

1. **Initialisation & Branding Hotmilk Studio** :
   - Crée l'arborescence complète du projet.
   - Génère les fichiers institutionnels : `LICENSE` (Apache 2.0 attribué à **Hotmilk Studio**), `NOTICE`, `CONTRIBUTING.md`, et un `README.md` haut de gamme sous le nom **Hotmilk Code**.
2. **Couche Inférence & Abstraction** :
   - Implémente l'architecture multi-provider (Ollama / llama.cpp) avec support du streaming temps réel.
3. **UX & Terminal UI** :
   - Intègre le header ASCII Art "Hotmilk Code" et configure le thème visuel dans le terminal.
   - Ajoute le système de validation manuelle ("Human-in-the-loop") avant toute écriture fichier ou commande shell.
4. **Agentic System Prompts** :
   - Conçois les prompts système pour modèles locaux (formats JSON / balises XML stricts pour éviter les dérives).

---

### INSTRUCTION INITIALE
Commence par créer l'arborescence des répertoires, le fichier `LICENSE` (Apache 2.0 - Hotmilk Studio), le `README.md` du projet **Hotmilk Code**, puis initie la structure du code source. Présente un résumé de l'initialisation avant d'enchaîner sur l'implémentation.