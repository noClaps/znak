FROM homebrew/brew

RUN brew install oven-sh/bun/bun

COPY bun.lock package.json ./
RUN bun install

COPY . .

EXPOSE 3000
CMD [ "bun", "dev" ]
