
const CELL_SIZE = 5;

// Plots the data where the data on the grid is assumed to go like
// (0,0), (1, 0), (2,0), ..., (cols-1,0), (0,1), (1, 1), ...
// where we are ploting on the first quadrant of the cartesian plane
export class U8DataPlotter {

    constructor(memory, dataPtrGetter, rows, cols, canvas, updater,
        colorMap) {
        canvas.height = rows * CELL_SIZE;
        canvas.width  = cols * CELL_SIZE;
        this.ctx = canvas.getContext('2d');
        this.ROWS = rows;
        this.COLS = cols;
        this.memory = memory;
        this.dataPtrGetter = dataPtrGetter;
        this.updater = updater;
        this.colorMap = colorMap;
    }

    getIndex(row, col) {
        return row * this.COLS + col;
    }


    drawCells() {
        const dataPtr = this.dataPtrGetter();
        const cells = new Uint8Array(this.memory.buffer, dataPtr,
                            this.ROWS * this.COLS);
        const ctx = this.ctx;

        ctx.beginPath();

        for (let row = 0; row < this.ROWS; row++) {
            for (let col = 0; col < this.COLS; col++) {
                const idx = this.getIndex(row, col);
                ctx.fillStyle = this.colorMap(cells[idx]);

                ctx.fillRect(
                    (this.COLS - col) * CELL_SIZE,
                    row * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE
                );
            }
        }

        ctx.stroke();
    };

    render() {
        const renderLoop = () => {
            this.updater();
            this.drawCells();
            requestAnimationFrame(renderLoop);
        };
        this.drawCells();
        requestAnimationFrame(renderLoop);
    }
                
}
