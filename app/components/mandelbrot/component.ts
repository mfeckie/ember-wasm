import Component from "@glimmer/component";

export default class Mandelbrot extends Component {
  async initMandelbrot(canvas: HTMLCanvasElement) {
    const { Chart } = await import("rust-plotter");
    Chart.mandelbrot(canvas);
  }
}
