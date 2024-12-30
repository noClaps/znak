import { render } from "../index.ts";

const md = await Bun.file("bench/bench.md").text();

const t0 = Bun.nanoseconds();
render(md);
const t1 = Bun.nanoseconds();

console.log(`Time to output: ${(t1 - t0) / (1000 * 1000)}ms`);

// Code blocks take a while when there's a recognised language (up to 30ms)
// Lists take slightly too long for my liking (0.3-1ms)
// Math blocks take slightly longer than I'd like (a little over 1ms)
// Inline math is also a bit slow (a little over 2ms)
