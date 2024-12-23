import { render } from "../index.ts";
import { Hono } from "hono";

const app = new Hono();

app
  .get("/", async (c) => {
    return c.html(await Bun.file(`${import.meta.dir}/src/index.html`).text());
  })
  .get("/style.css", async (c) => {
    c.header("Content-Type", "text/css");
    return c.body(await Bun.file(`${import.meta.dir}/src/style.css`).text());
  })
  .get("/script.js", async (c) => {
    const script = await Bun.build({
      entrypoints: [`${import.meta.dir}/src/script.ts`],
    }).then((bo) => bo.outputs[0].text());

    c.header("Content-Type", "text/javascript");
    return c.body(script);
  })
  .post("/render", async (c) => {
    const markup = await c.req.text();
    return c.text(render(markup));
  });

export default app;
