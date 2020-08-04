
class triangle {

	constructor(destinationSelector, framex, framey) {
		this.selector = destinationSelector;
		this.framex = framex;
		this.framey = framey;
		this.triangleColor = "red";
		this.labelColor = "darkblue";

	}

	render() {
		$(this.selector).append(
			$("canvas")
			.width(this.framex)
			.height(this.framey)
			);
		var canvasElement = document.querySelector(selection + " canvas");
		var context = canvasElement.getContext("2d");

		if (!(this.origin && this.sides)) {
			return;
		}
		var ox, oy, la, lo;
		ox, oy = this.origin;
		la, lo = this.sides;
		context.beginPath();
		context.moveTo(ox, oy);
		context.lineTo(ox + la, oy);
		context.lineTo(ox + la, oy - lo);
		context.closePath();
		context.lineWidth = 5;
		context.strokeStyle = "#666666";
		context.stroke();

		context.fillStyle = this.triangleColor;
		context.fill();
	}

	/*
	context.font = "20px sans-serif";
	context.fillStyle= "red";
	context.fillText("a", 150, 200 + 20);
	context.fillText("b", 200 + 10, 175);
	context.fillText("h", 150 - 10, 175 - 10);
	*/

	setOrigin(x,y) {
		this.origin = [x, y];
	}

	setSides(a, o) {
		this.sides = [a, o];
	}

	setAngleLabel(angle) {
		this.angleLabel = String(angle);
	}

	// The label at the vertex furthest from origin
	setPointLabel(label) {
		this.pointLabel = String(label);
	}

	setTriangleColor(color) {
		this.triangleColor = color;
	}

	setLabelColor(color) {
		this.labelColor = color;
	}

	setALabel(label) {
		this.ALabel = label;
	}

	setBLabel(label) {
		this.BLabel = label;
	}

	setHLabel(label) {
		this.HLabel = label;
	}

}

var drawTriangle = function () {
	let t = new triangle("#canvasLoc",500,500);
	t.setOrigin(100,300);
	t.setSides(200,100)
	t.render();
};

console.log("does this do anything");
drawTriangle();