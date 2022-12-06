# Needed SHELL since I'm using zsh
SHELL := /bin/bash

CONFIG ?= wrangler.toml

.PHONY: help
help: ## This help message
	@echo -e "$$(grep -hE '^\S+:.*##' $(MAKEFILE_LIST) | sed -e 's/:.*##\s*/:/' -e 's/^\(.\+\):\(.*\)/\\x1b[36m\1\\x1b[m:\2/' | column -c2 -t -s :)"

.PHONY: docker-build
docker-build: ## Build the docker image
	docker build -t openpodcast/forwarder .

.PHONY: docker-run
docker-run: ## Run the docker image
	docker run -it --rm -p 9000:9000 openpodcast/forwarder

.PHONY: build
build: # Compile project to WebAssembly
	wrangler build --config $(CONFIG)

.PHONY: dev 
dev: ## Run Worker in a development environment
	wrangler dev --config $(CONFIG)

.PHONY: publish deploy
publish deploy: ## Deploy worker to Cloudflare
	@wrangler publish --config $(CONFIG) --verbose || echo "Try wrangler login?"

.PHONY: logs tail
logs tail: ## Stream worker logs
	wrangler tail --config $(CONFIG) --verbose --format=pretty

.PHONY: test
test: ## Test Rust code
	cargo test

.PHONY: lint
lint: ## Lint Rust code
	cargo clippy --all-targets -- --deny warnings

.PHONY: test-rss
test-rss: ## Test request to worker homepage (retrieve RSS feed)
	curl -vvv -A "Spotify/1.0" -c - https://redcircle.mre.workers.dev/

.PHONY: test-rss-head
test-rss-head: ## Test HEAD request to worker (retrieve RSS feed)
	curl --head -vvv -A "Forwarder Test" -c - https://redcircle.mre.workers.dev/

.PHONY: test-mp3
test-mp3: ## Test request to worker mp3 file URL
	curl -L -vvv -A "Spotify/1.0" -c - https://redcircle.mre.workers.dev/r/episodes/3c0f3763-a825-456a-87c3-2e1c5a55cc25/stream.mp3?ref=https%3A%2F%2Fstream.redcircle.com%2Fepisodes%2F3c0f3763-a825-456a-87c3-2e1c5a55cc25%2Fstream.mp3