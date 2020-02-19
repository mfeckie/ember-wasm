import { module, test } from "qunit";
import { setupRenderingTest } from "ember-qunit";
import { render, click, waitFor } from "@ember/test-helpers";
import hbs from "htmlbars-inline-precompile";

module("Integration | Component | uuid", function(hooks) {
  setupRenderingTest(hooks);

  test("Renders a UUID", async function(assert) {
    await render(hbs`<Uuid />`);
    assert.dom("p").hasText("Generated UUID:");

    await waitFor('button');

    await click("button");

    assert
      .dom("span")
      .matchesText(
        /[0-9A-F]{8}-[0-9A-F]{4}-4[0-9A-F]{3}-[89AB][0-9A-F]{3}-[0-9A-F]{12}/i
      );
  });
});
