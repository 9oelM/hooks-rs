import { assert } from "https://deno.land/std@0.207.0/assert/mod.ts";
import * as path from "https://deno.land/std@0.207.0/path/mod.ts";
import { build, check, newProject, uninstall, up } from "../cli.ts";
import { DependenciesManager } from "../dependencies_manager/mod.ts";

// The tests need to be run in a fresh docker environment,
// because it tests against dependencies installation

// The tests must be run sequentially, in the order in
// which they are laid out here in this file because
// dependencies installation is shared across test cases

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
  const checksPassing = await check();
  assert(!checksPassing);
});

Deno.test(`[up] command should install all missing dependencies`, async () => {
  await up();
  const checksPassing = await check();
  assert(checksPassing);
});

Deno.test(`[uninstall] command should uninstall all dependencies except git, cargo and wasm-pack`, async () => {
  await uninstall();
  const prerequisitesInstallationStatus = await DependenciesManager
    .checkPrerequisitesInstalled();

  const shouldBeInstalled = new Set([
    `git`,
    `cargo`,
    `wasm-pack`,
  ]);

  Object.entries(prerequisitesInstallationStatus).forEach(
    ([prerequisiteName, isInstalled]) => {
      if (shouldBeInstalled.has(prerequisiteName)) {
        assert(isInstalled);
      } else {
        assert(!isInstalled);
      }
    },
  );
});

Deno.test(`[up] command should install partially missing dependencies`, async () => {
  const prerequisitesInstallationStatus = await DependenciesManager
    .checkPrerequisitesInstalled();

  assert(
    Object.values(prerequisitesInstallationStatus).some((isInstalled) => {
      return !isInstalled;
    }),
  );

  await up();
  const checksPassing = await check();
  assert(checksPassing);
});

Deno.test(`[build] command should build hooks-rs project`, async () => {
  const templateProjectEntries = new Set([
    `.cargo`,
    `src`,
    `tests`,
    `Cargo.toml`,
    `README.md`,
    `rust-toolchain.toml`,
  ]);

  const tmpDir = await Deno.makeTempDir();
  Deno.chdir(tmpDir);
  await newProject(undefined, `example-project-name`);
  const templateProjectPath = path.join(tmpDir, `example-project-name`);
  for await (const dirEntry of Deno.readDir(templateProjectPath)) {
    assert(templateProjectEntries.has(dirEntry.name));
  }

  Deno.chdir(templateProjectPath);

  await build();

  await Deno.remove(templateProjectPath, {
    recursive: true,
  });
});
