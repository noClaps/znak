import type { ServeOptions } from "bun";
import { watch } from "node:fs";
import { render } from "../index.ts";

const serveOptions: ServeOptions = {
  async fetch(req) {
    const path = new URL(req.url).pathname;

    switch (path) {
      case "/render": {
        const markup = await req.text();
        return new Response(render(markup));
      }
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
