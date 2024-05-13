import React from "react";

interface Props<T> {
  op: (m: number, n: number) => T;
  outputMessage: (input1: number, input2: number, output: T) => string;
  maxInput: number;
}

function NumberToValidInt(float: number, maxValid: number): number {
  let value = Math.floor(float > maxValid ? maxValid : float);
  if (value <= 0) {
    value = 1;
  }
  return value;
}

function BinOp<T>({ op, outputMessage, maxInput }: Props<T>) {
  let [inputs, setInputs] = React.useState([1, 1]);
  let [output, setOutput] = React.useState(op(1, 1));
  let [dTime, setDTime] = React.useState(0);

  function apply_op(
    arg1: number,
    arg2: number
  ): { result: T; time_elapsed: number } {
    const start = Date.now();
    const output = op(arg1, arg2);
    const dtime = Date.now() - start;
    return {
      result: output,
      time_elapsed: dtime,
    };
  }

  function handleChange(event) {
    let args = [...inputs];
    args[Number(event.target.id)] = NumberToValidInt(
      event.target.value,
      maxInput
    );
    setInputs(args);
    const res = apply_op(args[0], args[1]);
    setDTime(res.time_elapsed);
    setOutput(res.result);
  }

  return (
    <>
      <input
        id="0"
        type="number"
        step="1"
        value={inputs[0]}
        onChange={handleChange}
      />
      <input
        id="1"
        type="number"
        step="1"
        value={inputs[1]}
        onChange={handleChange}
      />
      <p>{outputMessage(inputs[0], inputs[1], output)}</p>
      <p>(Time to calculate: {dTime} miliseconds)</p>
    </>
  );
}

export default BinOp;
