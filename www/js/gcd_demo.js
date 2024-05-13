import React from "react";
import ReactDOM from "react-dom";
import {gcd, gcd_factors} from "../pkg/index.js";
import {BinOp} from "./components/binop.jsx";

function gcd_message(m, n, d) {
    return "The gcd of " + m + " and " + n + " is " + d + ".";
}

function gcd_factors_message(m, n, factors) {
    let symbol = " + ";
    let b = factors.b()
    if (b < 0) {
        symbol = " - ";
        b = -b;
    }
    return factors.a() + "*" + m + symbol + b + "*" + n + " = " + factors.gcd();
}

ReactDOM.render(
    <React.Fragment>
      <div className="demo">
        <BinOp
            op={gcd}
            outputMessage={gcd_message}
            maxInput={4294967295}
        />
      </div>
      <div className="demo">
        <BinOp
            op={gcd_factors}
            outputMessage={gcd_factors_message}
            maxInput={2147483647}
        />
      </div>
    </React.Fragment>,
    document.getElementById("gcd-demo"));
