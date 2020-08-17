
const CELL_SIZE = 5;
let count = 0;

// Plots the data where the data on the grid is assumed to go like
// (0,0), (1, 0), (2,0), ..., (cols-1,0), (0,1), (1, 1), ...
// where we are ploting on the first quadrant of the cartesian plane
export class U16DataPlotter {

    constructor(memory, dataPtrGetter, rows, cols, canvas, updater,
        colorMap, toggle_cell) {
        canvas.addEventListener("click", event => {
            this.toggleListener(event, canvas);
        });
        canvas.height = rows * CELL_SIZE;
        canvas.width  = cols * CELL_SIZE;
        this.ctx = canvas.getContext('2d');
        this.ROWS = rows;
        this.COLS = cols;
        this.memory = memory;
        this.dataPtrGetter = dataPtrGetter;
        this.updater = updater;
        this.colorMap = colorMap;
        this.toggle_cell = toggle_cell;
        this.animationId = null;
        this.renderLoop = () => {
            //console.log(count);
            count++;
            this.updater();
            this.drawCells();
            this.animationId = requestAnimationFrame(this.renderLoop);
        };
    }

    getIndex(row, col) {
        return row * this.COLS + col;
    }


    drawCells() {
        const dataPtr = this.dataPtrGetter();
        const cells = new Uint16Array(this.memory.buffer, dataPtr,
                            this.ROWS * this.COLS);
        const ctx = this.ctx;

        ctx.beginPath();

        for (let row = 0; row < this.ROWS; row++) {
            for (let col = 0; col < this.COLS; col++) {
                const idx = this.getIndex(row, col);
                ctx.fillStyle = this.colorMap(cells[idx]);

                ctx.fillRect(
                    col * CELL_SIZE,
                    (this.ROWS - 1 - row) * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE
                );
            }
        }

        ctx.stroke();
    };

    isPaused() {
        return this.animationId === null;
    }

    play() {
        this.renderLoop();
    }

    pause() {
        cancelAnimationFrame(this.animationId);
        this.animationId = null;
    }

    render() {
        this.drawCells();
        this.animationId = requestAnimationFrame(this.renderLoop);
    }

    toggleListener(event, canvas) {
        const boundingRect = canvas.getBoundingClientRect();

        const scaleX = canvas.width / boundingRect.width;
        const scaleY = canvas.height / boundingRect.height;

        
        const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
        const canvasTop  = (event.clientY - boundingRect.top)  * scaleY;


        const row = this.COLS - 1 - Math.max(
                        Math.min(Math.floor(canvasTop / CELL_SIZE), this.COLS - 1),
                        0
                    );
        const col = Math.max(
                        Math.min(Math.floor(canvasLeft / CELL_SIZE), this.ROWS - 1),
                        0
                    );


        const dataPtr = this.dataPtrGetter();
        const cells = new Uint8Array(this.memory.buffer, dataPtr,
                            this.ROWS * this.COLS);
        this.toggle_cell(row, col);
        this.drawCells();
    }
                
}
