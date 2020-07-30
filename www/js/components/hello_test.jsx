import React from "react";

export class Hello extends React.Component {
    
    constructor(props) {
        super(props);

        this.state = {
            clicked: false
        };

        this.handleClick = this.handleClick.bind(this);

    }

    handleClick() {
        this.setState({
            clicked: !this.state.clicked
        });
    }

    render() {
        let message = null;
        if (this.state.clicked) {
            message = <h1>You have clicked the button an 
            odd number of times!</h1>;
        }
        console.log(this.state.clicked);
        return (
            <React.Fragment>
              <input type="button" onClick={this.handleClick} />
              {message}
            </React.Fragment>
        );
    }
}
