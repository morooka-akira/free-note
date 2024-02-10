import express from "npm:express@4";

const app = express();

app.get("/", (_request: any, response: any) => {
  response.send("Hello from Express!");
});

app.listen(3000);
