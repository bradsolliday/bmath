import React from 'react';
import ReactDOM from 'react-dom';

const suffixes = {
    1: "st",
    2: "nd",
    3: "rd"
};

const suffix = (n) => {
    let val = suffixes[n];
    if (val) {
        return val;
    }
    return "th";
}

export class PrimeCalculator extends React.Component {

    constructor(props) {
        super(props);

        this.state = {
            input_n: 1,
            n: 1, // n as in nth prime
            cache: props.cache_initializer(), //PCache.new(1000000),
            nth_prime: 2,
            auto_calculate: true,
            dtime: 0
        }

        this.handleChange = this.handleChange.bind(this);
        this.calculate = this.calculate.bind(this);
        this.handleAutoCheck = this.handleAutoCheck.bind(this);
    }

    handleChange(event) {
        let new_n = Math.floor(Number(event.target.value));
        if (new_n <= 0) {
            new_n = 1;
        }
        if (this.state.auto_calculate) {
            this.setState({
                input_n: new_n,
            });
            this.calculate(new_n);
        } else {
            this.setState({
                input_n: new_n
            });
        }
    }


    calculate(input_n) {
        let start = Date.now();
        let prime = this.state.cache.nth_prime(input_n);
        let delta = Date.now() - start;
        this.setState({
            n: input_n,
            nth_prime: prime,
            dtime: delta
        });
    }

    handleAutoCheck(event) {
        this.setState({
            auto_calculate: event.target.checked
        });
    }

    render() {
        const nth = this.state.n + suffix(this.state.n);

        let calculate_button = null;
        if (!this.state.auto_calculate) {
            calculate_button = (
                <button type="button"
                        onClick={this.calculate.bind(this, this.state.input_n)}>
                    Calculate
                </button>
            );

        }

        return (
            <React.Fragment>
                <label htmlFor="auto">Auto Calculate</label>
                <input type="checkbox"
                       id="auto"
                       checked={this.state.auto_calculate}
                       onChange={this.handleAutoCheck} />
                <br/>
                <label htmlFor="nth">Which prime</label>
                <input type="number"
                       id="nth"
                       min="1"
                       step='1'
                       value={this.state.input_n}
                       onChange={this.handleChange} />
                {calculate_button}
                <p>The {nth} prime is {this.state.nth_prime}.</p>
                <p>(Time to calculate: {this.state.dtime} miliseconds)</p>
            </React.Fragment>
        );
    }
}

