import { assert } from "https://deno.land/std@0.207.0/assert/mod.ts";
import * as path from "https://deno.land/std@0.207.0/path/mod.ts";
import { check, newProject } from "../cli.ts";

// The tests need to be run in a fresh docker environment,
// because it tests against dependencies installation

Deno.test(`[new] command should create a new template project`, async () => {
  const templateProjectEntries = new Set([
    `.cargo`,
    `src`,
    `tests`,
    `Cargo.toml`,
    `README.md`,
    `rust-toolchain.toml`,
  ]);

  const tmpDir = await Deno.makeTempDir();
  await Deno.chdir(tmpDir);
  await newProject(undefined, `example-project-name`);
  const templateProjectPath = path.join(tmpDir, `example-project-name`);
  for await (const dirEntry of Deno.readDir(templateProjectPath)) {
    assert(templateProjectEntries.has(dirEntry.name));
  }
  await Deno.remove(templateProjectPath, {
    recursive: true,
  });
});

Deno.test(`[check] command should return false if not all dependencies are installed`, async () => {
  const allPrerequisitesInstalled = await check();
  assert(!allPrerequisitesInstalled);
});
