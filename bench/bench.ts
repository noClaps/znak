import { $ } from "bun";

const t0 = Bun.nanoseconds();
await $`./znak bench/bench.md -t ./theme.toml`.quiet();
const t1 = Bun.nanoseconds();

console.log(`Time to output: ${(t1 - t0) / (1000 * 1000)}ms`);
