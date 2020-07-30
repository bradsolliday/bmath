import { Hello } from "./components/hello_test.jsx";
import React from "react";
import ReactDOM from "react-dom";
import("../pkg/index.js").catch(console.error);

ReactDOM.render(<Hello />, document.getElementById("hello"));
