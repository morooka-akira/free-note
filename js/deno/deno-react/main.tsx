// /** @jsxImportSource https://esm.sh/preact */
// import React from "https://esm.sh/react@17.0.2";
// import { createRoot } from "https://esm.sh/react-dom@18.0.0";
// import * as ReactDOM from "https://jspm.dev/react-dom@17.0.0";
// import * as React from "https://jspm.dev/react@17.0.0";
// import { React, react, ReactDOM } from "./deps.ts";
import App from "./app.tsx";

// // function App() {
// //   return (
// //     <div>
// //       <h1>Hello, world!</h1>
// //     </div>
// //   );
// // }
// ReactDOM.render(
//   <react.StrictMode>
//     <App />
//   </react.StrictMode>,
//   document.getElementById("root")
// );
import { createRoot } from "https://esm.sh/react-dom@18.0.0";
import * as React from "https://esm.sh/react@18.0.0";

// function App() {
//   return (
//     <div>
//       <h1>Hello, world!</h1>
//     </div>
//   );
// }
const container = document.getElementById("root");
const root = createRoot(container); // createRoot(container!) if you're sure the container is non-null
root.render(<App />);
