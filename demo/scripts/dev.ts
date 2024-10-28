import type { ServeOptions } from "bun";
import { watch } from "node:fs";

const serveOptions: ServeOptions = {
  async fetch({ url }) {
    const path = new URL(url).pathname;

    switch (path) {
      case "/":
        return new Response(Bun.file("src/index.html"));
      case "/style.css":
        return new Response(Bun.file("src/style.css"));
      case "/script.js":
        const script = await Bun.build({
          entrypoints: ["src/script.ts"],
        }).then((bo) => bo.outputs[0].text());
        return new Response(script, {
          headers: { "content-type": "application/javascript" },
        });
      default:
        return new Response("Not found", { status: 404 });
    }
  },
};

const server = Bun.serve(serveOptions);
console.log(`Server started at ${server.url}`);

watch("src", (event, filename) => {
  console.log(`Detected ${event} on ${filename}`);
  server.reload(serveOptions);
  console.log("Reloaded.");
});
