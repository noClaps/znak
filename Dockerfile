FROM homebrew/brew:latest

RUN brew install python
RUN brew install oven-sh/bun/bun

COPY bun.lockb package.json .npmrc ./
RUN bun install --frozen-lockfile

COPY . .

EXPOSE 3000
CMD [ "bun", "dev" ]
