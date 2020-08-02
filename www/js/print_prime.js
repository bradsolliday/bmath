import {nth_prime} from "../pkg/index_bg.js";
import React from "react";
import ReactDOM from "react-dom";
import {Expandable} from "./components/expandable.jsx";
import {PrimeCalculator} from "./components/prime_calculator.jsx";

ReactDOM.render(<p>Starting calculation of prime 300</p>, document.getElementById("primes"));


ReactDOM.render(
    <Expandable>
      <PrimeCalculator />
    </Expandable>,
    document.getElementById("primes"));
