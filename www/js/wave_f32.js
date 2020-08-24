import {WaveGridF32} from "../pkg/index_bg.js";
import {memory} from "../pkg/index_bg";
import {DataPlotter} from "./data_plotter.js";
import React from "react";
import ReactDOM from "react-dom";
import {PlayPause} from "./components/play_pause.jsx";


function colorMap(f32) {
    f32 = f32 + 1;
    if (f32 < 0) {
        alert("pixel value was negative even after adjustment");
    }
    let hex = Math.floor(f32 * 100).toString(16);
    if (hex.length === 1) {
        hex = "0" + hex;
    }
    return "#" + hex + hex + hex;
}
    

const canvas = document.getElementById("my-canvas");
let grid = WaveGridF32.new();
let plotter = new DataPlotter(grid, memory, canvas,
                    colorMap, Float32Array);

ReactDOM.render(
    <PlayPause 
      isPaused={plotter.isPaused.bind(plotter)}
      play={plotter.play.bind(plotter)}
      pause={plotter.pause.bind(plotter)}
    />,
    document.getElementById("animation-button"));
plotter.render();
