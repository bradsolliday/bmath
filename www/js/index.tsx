import React from "react";
import ReactDOM from "react-dom";
import GcdDemo from "./gcd_demo";

import("./print_prime.js");
import("./wave_f32.js");

ReactDOM.render(<GcdDemo />, document.getElementById("gcd-demo"));
