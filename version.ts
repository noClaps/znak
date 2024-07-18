import packageJSON from "./package.json";

Bun.write(
  "./jsr.json",
  JSON.stringify({
    name: packageJSON.name,
    version: packageJSON.version,
    exports: packageJSON.module,
    publish: {
      include: [
        "src/**/*.ts",
        "bun.lockb",
        "index.ts",
        "LICENSE",
        "package.json",
        "README.md",
      ],
    },
  })
);
