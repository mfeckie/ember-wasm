import Component from "@glimmer/component";
import { action } from "@ember/object";
import { tracked } from "@glimmer/tracking";

interface UuidArgs {}

export default class Uuid extends Component<UuidArgs> {
  @tracked uuid?: string;
  @tracked showButton = false;
  uuidLib: any;
  constructor(owner: any, args: UuidArgs) {
    super(owner, args);
    this.initUUID();
  }
  async initUUID() {
    this.uuidLib = await import("rust-uuid");
    /* We want to ensure users cannot interact with the button
     * before the wasm lib is loaded, so set this flag to show button
     * after module is loaded
     */
    this.showButton = true;
  }

  @action
  genV4() {
    this.uuid = this.uuidLib.gen_v4();
  }
}
