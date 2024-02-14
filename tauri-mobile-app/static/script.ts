
function draw() {
  var canvas = document.getElementById("canvas1") as HTMLCanvasElement;

  canvas.height = 500;
  canvas.width = 500;

  var ctx = canvas.getContext("2d");
  if (ctx) {
    ctx.fillStyle = "rgb(200, 0, 0)";
    ctx.fillRect(10, 10, 50, 50);

    ctx.fillStyle = "rgba(0, 0, 200, 0.5)";
    ctx.fillRect(30, 30, 50, 50);
  }

}
