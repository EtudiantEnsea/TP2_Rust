<h1 align="center"> WiFi Analyzer (Rust) </h1>
<p align="center">
  <img alt="Version" src="https://img.shields.io/badge/version-1.0-blue.svg?cacheSeconds=2592000" />
</p>

> Analyse des trames WiFi (Beacon) et détection de DroneID à partir de fichiers PCAP

---

### 🏠 [Homepage](https://github.com/EtudiantEnsea)

# TP2_rust

Un compilateur et interpréteur pour un sous-ensemble du langage Logo développé en Rust.

---

## Description

`TP2_rust` est un TP visant à implémenter une chaîne de compilation complète pour un langage simple inspiré de Logo.

Le programme :

Analyse un programme Logo en entrée (ligne de commande)
Effectue une analyse lexicale (lexer) et syntaxique (parser)
Construit un arbre de syntaxe abstraite (AST)
Permet une interprétation (affichage console) et une compilation en SVG (dessin vectoriel)
---

## Fonctionnalités

* Définition d’une grammaire Logo (BNF simplifiée)
* Analyse lexicale avec santiago
* Analyse syntaxique (parser)
* Génération d’un AST
* Interpréteur Logo (console)
* Compilateur Logo → SVG
* Simulation d’une tortue graphique (turtle)

---

## Architecture

```text
src/
├── main.rs        # Interface CLI
├── lib.rs         # Bibliothèque principale
├── parser.rs      # Analyse des paquets
├── tlv.rs         # Parsing TLV
└── output.rs      # Export JSON / CSV
```

---

## Compilation

```sh
cargo build
```

---

## Utilisation

```sh
cargo run -- --pcap data/capture.pcap --output-format json --output-file results.json
```

---

## Exemple de sortie

```json
[
  {
    "mac": "34:27:92:37:ff:2a",
    "ssid": "Freebox-37FF29",
    "is_drone": false
  }
]
```

---

## Fonctionnement

1. Lecture du fichier PCAP
2. Extraction de l’en-tête Radiotap
3. Analyse des trames 802.11
4. Filtrage des trames Beacon (type 0, subtype 8)
5. Parsing TLV :

   * `0x00` → SSID
   * `0xdd` → Vendor Specific (DroneID)
6. Export des résultats

---

## Prérequis

* Rust installé
* Npcap (Windows)

---

## Documentation

```sh
cargo doc --no-deps --open
```

---

## Auteur

**Loick GOMES GRANCHO & Abdoul Nouroudine SANA**
