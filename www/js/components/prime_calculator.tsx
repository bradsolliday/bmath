import React from "react";

const suffixes = {
  1: "st",
  2: "nd",
  3: "rd",
};

const suffix = (n: number) => {
  let val = suffixes[n];
  if (val) {
    return val;
  }
  return "th";
};

interface PrimeCache {
  nth_prime: (n: number) => number;
}

interface Props {
  primeCache: PrimeCache;
}

function PrimeCalculator(props: Props) {
  let [autoCalculate, setAutoCalculate] = React.useState(true);
  let [inputN, setInputN] = React.useState(1);
  let [{ n, nthPrime, msToCompute }, setComputedState] = React.useState({
    n: 1,
    nthPrime: 2,
    msToCompute: 0,
  });
  let [primeCache, _] = React.useState(props.primeCache);

  function calculate(new_n: number) {
    let start = Date.now();
    let prime = primeCache.nth_prime(new_n);
    let delta = Date.now() - start;
    setComputedState({
      n: new_n,
      nthPrime: prime,
      msToCompute: delta,
    });
  }

  function handleNewInputN(event: React.ChangeEvent<HTMLInputElement>) {
    let new_n = Math.floor(Number(event.target.value));
    if (new_n <= 0) {
      new_n = 1;
    }
    setInputN(new_n);
    if (autoCalculate) {
      calculate(new_n);
    }
  }

  return (
    <>
      <label htmlFor="auto">Auto Calculate</label>
      <input
        type="checkbox"
        id="auto"
        checked={autoCalculate}
        onChange={(event: React.ChangeEvent<HTMLInputElement>) =>
          setAutoCalculate(event.target.checked)
        }
      />
      <br />
      <label htmlFor="nth">Which prime</label>
      <input
        type="number"
        id="nth"
        min="1"
        step="1"
        value={inputN}
        onChange={handleNewInputN}
      />
      {autoCalculate ? null : (
        <button type="button" onClick={() => calculate(inputN)}>
          Calculate
        </button>
      )}
      <p>
        The {n + suffix(n)} prime is {nthPrime}.
      </p>
      <p>(Time to calculate: {msToCompute} miliseconds)</p>
    </>
  );
}

export default PrimeCalculator;
