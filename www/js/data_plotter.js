
const CELL_SIZE = 5;
let count = 0;

// Plots the data where the data on the grid is assumed to go like
// (0,0), (1, 0), (2,0), ..., (cols-1,0), (0,1), (1, 1), ...
// where we are ploting on the first quadrant of the cartesian plane
export class DataPlotter {

    constructor(data_grid, memory, canvas, colorMap, typeArray) {
        canvas.addEventListener("click", event => {
            this.toggleListener(event, canvas);
        });
        canvas.height = data_grid.rows() * CELL_SIZE;
        canvas.width  = data_grid.cols() * CELL_SIZE;
        this.data = data_grid;
        this.typeArray = typeArray;
        this.ctx = canvas.getContext('2d');
        this.memory = memory;
        this.colorMap = colorMap;
        this.animationId = null;
        this.renderLoop = () => {
            this.data.update();
            this.data.update();
            this.data.update();
            this.data.update();
            this.data.update();
            this.data.update();
            this.data.update();
            this.data.update();
            this.data.update();
            this.data.update();
            this.data.update();
            this.data.update();
            this.data.update();
            this.data.update();
            this.data.update();
            this.drawCells();
            this.animationId = requestAnimationFrame(this.renderLoop);
        };
    }

    getIndex(row, col) {
        return row * this.data.rows() + col;
    }


    drawCells() {
        const ROWS = this.data.rows();
        const COLS = this.data.cols();
        const cells = new this.typeArray(this.memory.buffer,
                            this.data.data_pointer(),
                            ROWS * COLS);
        const ctx = this.ctx;

        ctx.beginPath();

        for (let row = 0; row < ROWS; row++) {
            for (let col = 0; col < COLS; col++) {
                const idx = this.getIndex(row, col);
                ctx.fillStyle = this.colorMap(cells[idx]);

                ctx.fillRect(
                    col * CELL_SIZE,
                    (ROWS - 1 - row) * CELL_SIZE,
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

        const ROWS = this.data.rows();
        const COLS = this.data.cols();

        const row = COLS - 1 - Math.max(
                        Math.min(Math.floor(canvasTop / CELL_SIZE), COLS - 1),
                        0
                    );
        const col = Math.max(
                        Math.min(Math.floor(canvasLeft / CELL_SIZE), ROWS - 1),
                        0
                    );

        this.data.toggle_cell(row, col);
        this.drawCells();
    }
                
}
