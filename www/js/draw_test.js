import {MyGrid} from "../pkg/index_bg.js";
import {memory} from "../pkg/index_bg";
import {U8DataPlotter} from "./u8_data_plotter.js";

function colorMap(u8) {
    let hex = u8.toString(16);
    if (hex.length === 1) {
        hex = "0" + hex;
    }
    return "#" + hex + hex + hex;
}

const canvas = document.getElementById("my-canvas");
let grid = MyGrid.new();
let plotter = new U8DataPlotter(memory, () => grid.cells(),
                    grid.rows(), grid.cols(), canvas,
                    () => grid.update(), colorMap);
plotter.render();
