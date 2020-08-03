import React from 'react';
import ReactDOM from 'react-dom';
import {PCache} from "../../pkg/index_bg.js";

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
            cache: PCache.new(1000),
            nth_prime: 2,
            auto_calculate: true
        }

        this.handleChange = this.handleChange.bind(this);
        this.calculate = this.calculate.bind(this);
        this.handleAutoCheck = this.handleAutoCheck.bind(this);
    }

    handleChange(event) {
        /* new_n must be between 1 and 6350 because the nth prime number
         * where n is non-positive is not well defined, and because the first
         * multiple of a prime number p that's not a multiple of a prime number
         * less than p is p^2, sqrt(u32::MAX + 1) = 65536 which is about equal
         * to the 6500th prime number, meaning we'll have integer overflow when
         * n gets too close to 6500
         * */
        let new_n = Number(event.target.value);
        if (new_n <= 0) {
            new_n = 1;
        }
        if (new_n > 6300) {
            new_n = 6300;
        }
        if (this.state.auto_calculate) {
            this.setState({
                input_n: new_n,
                n: new_n,
                nth_prime: this.state.cache.nth_prime(new_n)
            });
        } else {
            this.setState({
                input_n: new_n
            });
        }
    }


    calculate() {
        this.setState({
            n: this.state.input_n,
            nth_prime: this.state.cache.nth_prime(Number(this.state.input_n))
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
                        onClick={this.calculate}>
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
                <label htmlFor="nth">Which prime</label>
                <input type="number"
                       id="nth"
                       min="1"
                       step='1'
                       value={this.state.input_n}
                       onChange={this.handleChange} />
                {calculate_button}
                <p>The {nth} prime is {this.state.nth_prime}.</p>
            </React.Fragment>
        );
    }
}

