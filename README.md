# SSSSSSS

Ssssss sssss ss ssss ssss, sssss ss sssss. Ss ssss Ssss ss Ssss.

Ss ssss :

- sss ssssss sssssss ;
- ss ssss ssssssss sssss ;
- sss sssssssssss ssss ;
- ss ssss sssss s'sssss sssss.

Harry Potter pour vous traduire :

Jeu du serpent :

- fond d'écran 
- Papple
- Collisions
- Timer, Score

## Compiler le truc

### Version web

Dépendences :

- Rust et cargo (via rustup)
- Make
- WASM bindgen (`cargo install wasm-bindgen-cli`)
- httplz
- xdg-open

Si vous avez Nix/NixOS `nix-shell` peut installer tout ça pour vous.

```
rustup target add wasm32-unknown-unknown
make web
```

### Version native

Dépendences :

- Rust et cargo (via rustup)
- Make

Pareil, `nix-shell` peut les installer.

```
make native
```

Ou si vous avez pas `make` (genre sous Windows ça peut être galère à installer),
cette commande devrait suffire :

```
cargo run --features native
```
