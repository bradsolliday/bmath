import React from "react";
import ReactDOM from "react-dom";

export class BinOp extends React.Component {

    constructor(props) {
        super(props);

        this.state = {
            input1: 1,
            input2: 1,
            arg1:   1,
            arg2:   1,
            output: props.op(1, 1),
            dtime:  0
        }
        
        this.handleChange = this.handleChange.bind(this);
        this.calculate = this.calculate.bind(this);

    }

    handleChange(event) {
        const name = event.target.name;
        let value = Number(event.target.value);
        if (value <= 0) {
            value = 1;
        }
        const MAX = this.props.maxInput;
        if (value > MAX) {
            value = MAX;
        }
        this.setState({
            [name]: value
        });
        let args = {
            input1: this.state.input1,
            input2: this.state.input2
        }
        args[name] = value;
        this.calculate(args.input1, args.input2);
    }


    calculate(arg1, arg2) {
        const start  = Date.now();
        const output = this.props.op(arg1, arg2);
        const dtime  = Date.now() - start;
        this.setState({
            arg1: arg1,
            arg2: arg2,
            output: output
        });
    }

    render() {
        return (
            <React.Fragment>
              <input name="input1" type="number" step="1"
                  value={this.state.input1}
                  onChange={this.handleChange} />
              <input name="input2" type="number" step="1"
                  value={this.state.input2}
                  onChange={this.handleChange} />
              <p>{this.props.outputMessage(this.state.arg1, this.state.arg2,
                  this.state.output)}</p>
              <p>(Time to calculate: {this.state.dtime} miliseconds)</p>
            </React.Fragment>
        );
    };

}

