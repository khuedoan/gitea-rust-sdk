.POSIX:

default: generate format

generate:
	docker run --rm \
		--volume ${PWD}:/local openapitools/openapi-generator-cli generate \
		--input-spec https://gitea.com/swagger.v1.json \
		--invoker-package Gitea \
		--generator-name rust \
		--additional-properties packageName=gitea-sdk \
		--output /local/

format:
	cargo fmt
