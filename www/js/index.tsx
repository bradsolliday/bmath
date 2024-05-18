import React from "react";
import ReactDOM from "react-dom";
import GcdDemo from "./gcd_demo";
import PrimeDemo from "./print_prime";

import("./wave_f32.js");

ReactDOM.render(<GcdDemo />, document.getElementById("gcd-demo"));

ReactDOM.render(<PrimeDemo />, document.getElementById("primes"));
