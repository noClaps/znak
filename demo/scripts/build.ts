Bun.write("dist/index.html", Bun.file("src/index.html"));

await Bun.build({
  entrypoints: ["src/style.css"],
  outdir: "dist",
  experimentalCss: true,
  minify: true,
});

await Bun.build({
  entrypoints: ["src/script.ts"],
  outdir: "dist",
  minify: true,
});
