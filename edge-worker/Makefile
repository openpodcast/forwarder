# Needed SHELL since I'm using zsh
SHELL := /bin/bash

.PHONY: help
help: ## This help message
	@echo -e "$$(grep -hE '^\S+:.*##' $(MAKEFILE_LIST) | sed -e 's/:.*##\s*/:/' -e 's/^\(.\+\):\(.*\)/\\x1b[36m\1\\x1b[m:\2/' | column -c2 -t -s :)"

.PHONY: build
build: # Compile project to WebAssembly
	wrangler build

.PHONY: dev 
dev: ## Run Worker in a development environment
	wrangler dev 

.PHONY: publish deploy
publish deploy: ## Deploy worker to Cloudflare
	@wrangler publish --verbose || echo "Try wrangler login?"

.PHONY: logs tail
logs tail: ## Stream worker logs
	wrangler tail --verbose --format=pretty

.PHONY: test
test: ## Test Rust code
	cargo test

.PHONY: lint
lint: ## Lint Rust code
	cargo clippy --all-targets -- --deny warnings

.PHONY: test-rss
test-rss: ## Test request to worker homepage (retrieve RSS feed)
	curl -vvv -A "Spotify/1.0" -c - https://forwarder.mre.workers.dev

.PHONY: test-mp3
test-mp3: ## Test request to worker mp3 file URL
	curl -vvv -A "Spotify/1.0" -c - https://forwarder.mre.workers.dev/r/GLT6565139983.mp3?updated=1626919518&ref=https%253A%252F%252Ftraffic.megaphone.fm%252FGLT6565139983.mp3%253Fupdated%253D1626919518