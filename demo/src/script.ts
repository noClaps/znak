import { render } from "../../index.ts";

const inputElem = document.querySelector<HTMLTextAreaElement>("#input")!;
const outputElem = document.querySelector("output")!;

inputElem.addEventListener("input", async () => {
  outputElem.innerHTML = await render(inputElem.value);
});

inputElem.addEventListener("keydown", (e) => {
  if (e.key === "Tab") {
    e.preventDefault();
    const start = inputElem.selectionStart;
    const end = inputElem.selectionEnd;
    const value = inputElem.value;

    inputElem.value = value.substring(0, start) + "\t" + value.substring(end);
    inputElem.selectionStart = start + 1;
    inputElem.selectionEnd = end + 1;
  }
});
