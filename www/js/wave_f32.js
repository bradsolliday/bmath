import {WaveGridF32, wasm_memory} from "../pkg/index.js";
import {DataPlotter} from "./data_plotter.js";
import React from "react";
import ReactDOM from "react-dom";
import {PlayPause} from "./components/play_pause.jsx";

let colorArray = [];
for (let i = 0; i < 256; i++) {
    let hex = i.toString(16);
    if (hex.length === 1) {
        hex = "0" + hex;
    }
    colorArray[i] = "#" + hex + hex + hex;
}

function colorMap(f32) {
    if (f32 < 0) {
        f32 = 0;
    } else if (f32 > 255) {
        f32 = 255;
    }
    return colorArray[Math.floor(f32)];
}

const canvas = document.getElementById("my-canvas");
let grid = WaveGridF32.new();
let plotter = new DataPlotter(grid, wasm_memory(), canvas,
                    colorMap, Float32Array);

ReactDOM.render(
    <React.Fragment>
      <PlayPause 
        isPaused={plotter.isPaused.bind(plotter)}
        play={plotter.play.bind(plotter)}
        pause={plotter.pause.bind(plotter)}
      />
      <button onClick={plotter.reset.bind(plotter)}>
        Reset Field
      </button>
    </React.Fragment>,
    document.getElementById("animation-button"));
plotter.render();
