import React from "react";
import ReactDOM from "react-dom";
import Expandable from "./components/expandable";
import PrimeCalculator from "./components/prime_calculator";
import { PCache, NaivePCache } from "../pkg/index";

ReactDOM.render(
  <p>Starting calculation of prime 300</p>,
  document.getElementById("primes")
);

const PCacheIntro =
  "Here you can demo the prime number calculator detailed in the documentation for bmath. It's written in Rust and compiled to Web Assembly";

const NaiveWasmIntro =
  "This below also calculates prime numbers, but it uses the naive method of mainaining a list of prime numbers checking each number to see if it's prime by seeing if any of our existing prime numbers divide it. This is a point of comparison to demonstrate the speed gains we get above. This implementation also caches previously calculated primes.";

const NaiveJsIntro =
  "This below is the same as the above, using even the same naive algorithm, yet is written in pure Javascript. You'll notice it is faster than the Web Assembly version, but I'm not yet sure why this is.";

class NaiveJSPCache {
  primes: number[];
  max_checked: number;

  constructor() {
    this.primes = [];
    this.max_checked = 1;
  }

  nth_prime(n: number): number {
    let smallest_unchecked = this.max_checked + 1;
    let found_divisor = false;
    while (this.primes.length < n) {
      for (let i = 0; i < this.primes.length; i++) {
        if (smallest_unchecked % this.primes[i] === 0) {
          found_divisor = true;
          break;
        }
      }
      if (!found_divisor) {
        this.primes.push(smallest_unchecked);
        this.max_checked = smallest_unchecked;
      }
      found_divisor = false;
      smallest_unchecked++;
    }
    return this.primes[n - 1];
  }
}

function PrimeDemo() {
  return (
    <>
      <p>Reset cached primes by minimizing and reexpanding.</p>
      <Expandable>
        <br />
        <p>{PCacheIntro}</p>
        <div className="demo">
          <PrimeCalculator primeCache={PCache.new(1000000)} />
        </div>
        <p>{NaiveWasmIntro}</p>
        <div className="demo">
          <PrimeCalculator primeCache={NaivePCache.new()} />
        </div>
        <p>{NaiveJsIntro}</p>
        <div className="demo">
          <PrimeCalculator primeCache={new NaiveJSPCache()} />
        </div>
      </Expandable>
    </>
  );
}

export default PrimeDemo;
