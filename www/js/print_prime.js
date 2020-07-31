import {nth_prime} from "../pkg/index_bg.js";
import React from "react";
import ReactDOM from "react-dom";

ReactDOM.render(<p>Starting calculation of prime 300</p>, document.getElementById("primes"));

const p300 = nth_prime(300);

ReactDOM.render(<p>300th prime is {p300}</p>, document.getElementById("primes"));
