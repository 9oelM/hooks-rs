import { assert } from "jsr:@std/assert";
import { DependenciesManager } from "../dependencies_manager/mod.ts";

Deno.test(`Should identify nightly version`, () => {
  assert(DependenciesManager.CARGO_VERSION_NIGHTLY_REGEX.test(
    "cargo 1.75.0-nightly (6fa6fdc76 2023-10-10)",
  ));
});

Deno.test(`Should identify non-nightly version`, () => {
  assert(
    !DependenciesManager.CARGO_VERSION_NIGHTLY_REGEX.test(
      "cargo 1.75.0-blah (6fa6fdc76 2023-10-10)",
    ),
  );
  assert(
    !DependenciesManager.CARGO_VERSION_NIGHTLY_REGEX.test(
      "cargo 1.75.0 (6fa6fdc76 2023-10-10)",
    ),
  );
});
