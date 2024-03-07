import App from "./app.tsx";
import { createRoot } from "https://esm.sh/react-dom@18.0.0";
import { React } from "./deps.ts";

const container = document.getElementById("root");
const root = createRoot(container);
root.render(<App />);
