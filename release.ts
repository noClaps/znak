import packageJSON from "./package.json";

Bun.write(
  "./jsr.json",
  JSON.stringify({
    name: packageJSON.name,
    version: packageJSON.version,
    exports: packageJSON.module,
  }),
);
