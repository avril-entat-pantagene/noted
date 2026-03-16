# noted

Starter project Rust avec `ratatui` et environnement Nix (`flake.nix`).

## Prérequis

- Nix avec les flakes activées

## Démarrage rapide

Entrer dans le shell de dev (toolchain + outils) :

```bash
nix develop
```

Générer le lockfile puis compiler :

```bash
cargo generate-lockfile
cargo check
```

Lancer l'application TUI :

```bash
cargo run
```

Quitter avec la touche `q`.

## Fichiers clés

- `flake.nix` : dépendances système + toolchain Rust
- `rust-toolchain.toml` : composants Rust (`clippy`, `rustfmt`, `rust-analyzer`, `rust-src`)
- `src/main.rs` : hello world `ratatui`