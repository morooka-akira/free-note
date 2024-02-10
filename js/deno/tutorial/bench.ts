Deno.bench("hello world", () => {
    new URL("https://deno.land");
});
