import React from "react";
import { gcd, gcd_factors, GCDCoefficients } from "../pkg/index";
import BinOp from "./components/binop";

function gcd_message(m: number, n: number, d: number): string {
  return "The gcd of " + m + " and " + n + " is " + d + ".";
}

function gcd_factors_message(
  m: number,
  n: number,
  factors: GCDCoefficients
): string {
  let symbol = " + ";
  let b = factors.b();
  if (b < 0) {
    symbol = " - ";
    b = -b;
  }
  return factors.a() + "*" + m + symbol + b + "*" + n + " = " + factors.gcd();
}

function GcdDemo() {
  return (
    <>
      <div className="demo">
        <BinOp op={gcd} outputMessage={gcd_message} maxInput={4294967295} />
      </div>
      <div className="demo">
        <BinOp
          op={gcd_factors}
          outputMessage={gcd_factors_message}
          maxInput={2147483647}
        />
      </div>
    </>
  );
}

export default GcdDemo;
