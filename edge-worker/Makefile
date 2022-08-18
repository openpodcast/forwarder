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

.PHONY: publish 
publish: ## Deploy worker to Cloudflare
	wrangler publish 

.PHONY: test
test: ## Test request to worker
	curl -A "Spotify/1.0" -c - https://forwarder.mre.workers.dev