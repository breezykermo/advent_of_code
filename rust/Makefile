.PHONY: run
SHELL := /bin/bash

today: src/template.rs
	source .env
	rm -f src/today.rs
	source.env && touch data/${YEAR}/${DAY}.txt
	source .env && touch src/${YEAR}/${DAY}.rs
	cp src/template.rs src/today.rs

save:
	source .env && rm -r src/${YEAR}/${DAY}.rs
	source .env && cp src/today.rs src/${YEAR}/${DAY}.rs

run:
	source .env && cargo run | bat
