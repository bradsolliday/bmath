import {nth_prime} from "../pkg/index_bg.js";
import React from "react";
import ReactDOM from "react-dom";
import {Expandable} from "./components/expandable.jsx";
import {PrimeCalculator} from "./components/prime_calculator.jsx";
import {PCache, NaivePCache} from "../pkg/index_bg.js";

ReactDOM.render(<p>Starting calculation of prime 300</p>, document.getElementById("primes"));


ReactDOM.render(
    <React.Fragment>
        <p>Reset cached primes by minimizing and reexpanding.</p>
        <Expandable>
          <br/>
          <PrimeCalculator cache_initializer={() => PCache.new(1000000)}/>
          <p>Naive implementation:</p>
          <PrimeCalculator cache_initializer={NaivePCache.new}/>
        </Expandable>
    </React.Fragment>,
    document.getElementById("primes"));
