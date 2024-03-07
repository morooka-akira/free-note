import { bundle } from "https://deno.land/x/emit@0.38.2/mod.ts";

const url = new URL("./main.tsx", import.meta.url);
const result = await bundle(url, {
  minify: false,
});
console.log(result.code);
await Deno.writeTextFile("./index.js", result.code);
