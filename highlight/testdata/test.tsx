import config from "@/config.json";
import { Search, Undo2 } from "lucide-static";

export function IndexPage() {
  return (
    <>
      {`<!doctype html>`}
      <html
        lang="en"
        class={"font-[Zodiak,serif] bg-[--background] text-[--text]"}
      >
        <head>
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width" />
          <meta name="description" content={config.description} />
          <title>{config.title}</title>
          <link
            href="https://api.fontshare.com/v2/css?f[]=zodiak@1,2&display=swap"
            rel="stylesheet"
          />
          <link rel="stylesheet" href="/style.css" />
          <link rel="stylesheet" href="/uno.css" />
          <script type="module" src="/script.js"></script>
        </head>
        <body
          class={"min-h-dvh grid grid-rows-[auto_1fr_auto] mx-auto max-w-88dvw"}
        >
          <header class={"text-center flex justify-between flex-items-center"}>
            <span class={"grow"}>
              <h1 class={"text-3rem font-900"}>{config.title}</h1>
              <p
                class={
                  "text-[red] text-1.125rem hidden [@media(any-pointer:coarse)]:block"
                }
              >
                {config.touchWarning}
              </p>
            </span>
            <button
              class={[
                "cursor-pointer",
                "bg-[--background]",
                "border-solid border-0.125rem border-[--accent]",
                "rounded-0.5rem",
                "flex justify-center items-center",
                "transition duration-250",
                "h-fit",
                "hover:bg-[--accent]",
              ]}
              id="search-button"
            >
              {Search}
            </button>
          </header>
          <main
            class={
              "grid grid-cols-[repeat(auto-fit,minmax(20rem,1fr))] gap-1rem"
            }
          >
            {config.sections.map(({ title, links }) => (
              <section
                class={
                  "border-solid border-0.125rem border-[--text] p-1.5rem rounded-0.5rem"
                }
              >
                <h2 class={"text-center m-0 pb-1rem text-2rem font-750"}>
                  {title}
                </h2>
                <div class={"flex flex-col items-center gap-1rem"}>
                  {links.map((link) => (
                    <a
                      href={link.url}
                      class={[
                        "bg-[--accent]",
                        "p-0.75rem",
                        "w-[calc(100%-1.5rem)]",
                        "text-center text-1.125rem text-[--text]",
                        "rounded-0.5rem",
                        "no-underline",
                        "transition duration-250",
                        "hover:shadow-[0_0_0.5rem_0.25rem_var(--accent)]",
                      ]}
                    >
                      {link.title}
                    </a>
                  ))}
                </div>
              </section>
            ))}
          </main>
          <footer class={"p-1rem text-center"}>
            Copyright ©{" "}
            <a class={"text-[--text]"} href="https://zerolimits.dev">
              noClaps
            </a>{" "}
            2024 •{" "}
            <a class={"text-[--text]"} href="/legal/">
              Legal
            </a>
          </footer>
          <dialog
            class={[
              "[@starting-style]:opacity-0 [@starting-style]:invisible",
              "absolute",
              "h-[calc(100dvh-4rem)] w-100dvw",
              "inset-0",
              "bg-[--translucent] backdrop-blur-1rem",
              "pt-4rem",
              "opacity-100 visible",
              "open:flex flex-col items-center",
            ]}
            id="search-overlay"
          >
            <span class={"flex items-center gap-1rem"}>
              <form method="dialog">
                <button
                  class={[
                    "flex items-center justify-center",
                    "h-max",
                    "bg-transparent",
                    "border-none",
                    "cursor-pointer",
                    "text-[--text]",
                  ]}
                  id="return"
                >
                  {Undo2}
                </button>
              </form>
              <input
                class={[
                  "bg-[--background]",
                  "border-solid border-0.125rem border-[--accent] rounded-0.5rem",
                  "font-[Zodiak,serif] text-[--text] text-1.125rem",
                  "p-0.5rem",
                  "transition duration-250",
                  "focus:shadow-[0_0_0.5rem_0.25rem_var(--accent)] focus:outline-none",
                ]}
                type="search"
                id="search"
                placeholder="Search..."
                autofocus=""
              />
            </span>
            <ul
              class={[
                "flex flex-col",
                "list-none",
                "border-b-0.125rem border-b-solid border-b-[--text]",
                "pl-0",
                "w-[min(64rem,88dvw)]",
                "text-1.125rem",
              ]}
              id="results"
            ></ul>
          </dialog>
        </body>
      </html>
    </>
  );
}
