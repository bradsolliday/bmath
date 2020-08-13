import React from "react";

export class PlayPause extends React.Component {

    constructor(props) {
        super(props);

        this.state = {
            text: "⏸" // A pause button
        }
    }

    render() {
        return (
            <button onClick={ () => {
                if (this.props.isPaused()) {
                    this.props.play();
                    this.setState({
                        text: "⏸" // a pause symbol
                    });
                } else {
                    this.props.pause();
                    this.setState({
                        text: "▶" // a play symbol
                    });
                }
            }}>
                {this.state.text}
            </button>
        );
    }
}

