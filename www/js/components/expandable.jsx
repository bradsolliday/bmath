import React from "react";
import ReactDOM from "react-dom";

export class Expandable extends React.Component {

    constructor(props) {
        super(props);

        this.state = {
            expanded: true
        };

        this.handleClick = this.handleClick.bind(this);
    }

    handleClick() {
        this.setState({
            expanded: !this.state.expanded
        })
    }

    render() {
        if (this.state.expanded) {
            return (
                <React.Fragment>
                    <button type="button"
                        className="button"
                        onClick={this.handleClick}>Minimize</button>
                    {this.props.children}
                </React.Fragment>
            );
        }
        return (
            <button type="button"
                className="button"
                onClick={this.handleClick}>Expand</button>
        );
    }
}

