import { $ } from "bun";

Bun.build({
  entrypoints: ["index.ts"],
  outdir: "dist",
  target: "node",
  external: ["@noclaps/highlight", "temml"],
});

await $`tsc --project tsconfig.json`;
