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
└── main.rs       # Programme principal (lexer, parser, AST, interprétation, SVG)
output.svg        # Dessin fais l'utilisateur
```
---

## Utilisation

```sh
cargo run --bin TP2 -- forward 100 right 90 forward 100 right 90 forward 100 right 90 forward 100

cargo run --bin TP2 -- repeat 4 [ forward 100 right 90 ]
```

---

## Exemple de sortie

<img width="1709" height="852" alt="image" src="https://github.com/user-attachments/assets/04964a62-0e90-44f5-bd75-ee17ca6e9e0c" />


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
---

## Auteur

**Loick GOMES GRANCHO & Abdoul Nouroudine SANA**
