import React from "react";
import ReactDOM from "react-dom";
import {gcd} from "../pkg/index_bg.js";
import {BinOp} from "./components/binop.jsx";

ReactDOM.render(
    <BinOp opName="greatest common divisor" op={gcd}/>,
    document.getElementById("gcd-demo"));
