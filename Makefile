default: native

# Compile une version "release" du jeu
# C'est une version sans information de debug, et avec plus d'optimisations
# activées. Ça servira quand on aura une version finie.
# Ça compile à la fois la version web et une version « native »
release:
	cargo build --target wasm-unknown-unknown --features web --release
	wasm-bindgen --out-dir target --out-name wasm --target web --no-typescript target/wasm32-unknown-unknown/release/serpent.wasm
	# TODO : compiler la version native

# Compile et lance la version native du jeu
native:
	cargo run --features native

# Compile la version web du jeu et la lance dans le navigateur
web: build-web
	xdg-open http://127.0.0.1:7878
	httplz -p 7878 &

# Compile la version web du jeu mais la lance pas
build-web:
	cargo build --target wasm32-unknown-unknown --features web
	wasm-bindgen --out-dir target --out-name wasm --target web --no-typescript target/wasm32-unknown-unknown/debug/serpent.wasm

# Pour dire de toujours recompiler, même si a priori rien n'a changé
# (make est bête et ne regarde pas si il y a eu du changement dans les fichiers .rs)
# (mais cargo est assez intelligent pour recompiler que si il y a besoin, LUI AU MOINS,
# donc on perd pas de temps)
.PHONY: build-web run-native

