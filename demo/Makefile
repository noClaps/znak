build:
	@mkdir -p dist/
	@cp src/index.html src/style.css dist/
	@bun build --minify --outfile dist/script.js src/script.ts

serve:
	@make build
	@bunx serve -n dist/

dev:
	@watchexec -r "make serve"
