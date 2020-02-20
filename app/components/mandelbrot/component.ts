import Component from "@glimmer/component";

interface MandelbrotArgs {}

export default class Mandelbrot extends Component<MandelbrotArgs> {
  canvas?: HTMLCanvasElement;
  constructor(owner: any, args: MandelbrotArgs) {
    super(owner, args);
    this.initMandelbrot();
  }

  async initMandelbrot() {
    const { Chart } = await import("rust-plotter");
    Chart.mandelbrot(this.canvas);
  }
}
