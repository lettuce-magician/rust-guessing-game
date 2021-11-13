CARGO = cargo

guessing_game: src/main.rs src/game.rs
	$(cargo) build --target-dir ./