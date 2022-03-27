import init, { parse } from 'layout-engine';

init().then(() => {
  const canvas = document.getElementById('canvas') as HTMLCanvasElement;

  const ctx = canvas.getContext("2d");

  const dpr = window.devicePixelRatio || 1;
  canvas.style.width = canvas.width + "px";
  canvas.style.height = canvas.height + "px";
  canvas.width *= dpr;
  canvas.height *= dpr;
  ctx.scale(dpr, dpr);


  const [width, height, ...color] = parse();

  const imageData = ctx.createImageData(width, height);
  for (let i = 0; i < imageData.data.length; i += 4) {
    imageData.data[i + 0] = color[i + 0];
    imageData.data[i + 1] = color[i + 1];
    imageData.data[i + 2] = color[i + 2];
    imageData.data[i + 3] = color[i + 3];
  }

  ctx.putImageData(imageData, 0, 0);

}).catch((e) => {
  console.log(e)
}
);


