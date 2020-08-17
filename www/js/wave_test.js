import {WaveGrid} from "../pkg/index_bg.js";
import {memory} from "../pkg/index_bg";
import {U16DataPlotter} from "./u16_data_plotter.js";
import React from "react";
import ReactDOM from "react-dom";
import {PlayPause} from "./components/play_pause.jsx";

function colorMap(u16) {
    let val = (u16 >>> 0) | 0 // for some reason, if a: i32, then (a as u32) | 0 = a
    let isNeg = false;
    if (val > ((1 << 15) + 1)) {
        val = ((val ^ -1) << 16) >> 16;
        isNeg = true;
    }
    let hex = (val >>> 7).toString(16);
    if (hex.length === 1) {
        hex = "0" + hex;
    }
    if (isNeg) {
        return "#0000" + hex;
    }
    return "#" + hex + "0000";
}

const canvas = document.getElementById("my-canvas");
let grid = WaveGrid.new();
let plotter = new U16DataPlotter(memory, () => grid.positions(),
                    grid.rows(), grid.cols(), canvas,
                    grid.update.bind(grid), colorMap,
                    (row, col) => grid.toggle_cell(row, col));
ReactDOM.render(
    <PlayPause 
      isPaused={plotter.isPaused.bind(plotter)}
      play={plotter.play.bind(plotter)}
      pause={plotter.pause.bind(plotter)}
    />,
    document.getElementById("animation-button"));
plotter.render();
