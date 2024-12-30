FROM homebrew/brew

RUN brew install oven-sh/bun/bun

COPY bun.lockb package.json bunfig.toml ./
RUN bun install

COPY . .

EXPOSE 3000
CMD [ "bun", "dev" ]
