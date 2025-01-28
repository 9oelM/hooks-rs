import { Logger } from "../misc/mod.ts";
import { TypedObjectKeys } from "../types/utils.ts";
// @deno-types="../types/command_exists.d.ts"
import commandExists from "npm:command-exists";
import * as path from "jsr:@std/path";
import { download } from "https://deno.land/x/download@v2.0.2/mod.ts";

// "cargo 1.75.0-nightly"
export const CARGO_VERSION_NIGHTLY_REGEX = /cargo\s\d+\.\d+\.\d+-nightly/;

export interface PrerequisitesInstallationStatus {
  git: boolean;
  cargo: boolean;
  "wasm-opt": boolean;
  "hook-cleaner": boolean;
  wasm2wat: boolean;
  guard_checker: boolean;
  "wasm-pack": boolean;
}

export async function checkCargoNightlySelectedAsDefault() {
  const cargoVersionOutput = await new Deno.Command(`cargo`, {
    args: [
      `--version`,
    ],
  }).output();

  const cargoVersionString = new TextDecoder().decode(
    cargoVersionOutput.stdout,
  );

  return CARGO_VERSION_NIGHTLY_REGEX.test(cargoVersionString);
}

export async function checkRustupWasm32UnknownUnknownInstalled() {
  // rustup target list --installed
  const rustupTargetListOutput = await new Deno.Command(`rustup`, {
    args: [
      `target`,
      `list`,
      `--installed`,
    ],
  }).output();

  const decoder = new TextDecoder();
  const rustupTargetListString = decoder.decode(
    rustupTargetListOutput.stdout,
  );

  return rustupTargetListString.includes(`wasm32-unknown-unknown`);
}

export async function checkPrerequisitesInstalled() {
  // Do not change this order, since cargo and git are required for other installations
  // and Deno runtime will keep the order when Object.keys is called
  const prerequisitesInstallationStatus: PrerequisitesInstallationStatus = {
    git: false,
    cargo: false,
    [`wasm-opt`]: false,
    [`hook-cleaner`]: false,
    [`wasm2wat`]: false,
    [`guard_checker`]: false,
    [`wasm-pack`]: false,
  };

  for (const prerequisite of TypedObjectKeys(prerequisitesInstallationStatus)) {
    try {
      await commandExists(prerequisite);
      prerequisitesInstallationStatus[prerequisite] = true;
      // deno-lint-ignore no-empty
    } catch {}
  }

  return prerequisitesInstallationStatus;
}

const BINARYEN_RELEASE_116 =
  `https://github.com/WebAssembly/binaryen/releases/download/version_116/binaryen-version_116`;
const ARM64_MACOS = `arm64-macos`;
const X86_64_MACOS = `x86_64-macos`;
const X86_64_LINUX = `x86_64-linux`;
type BinaryenTarget =
  | typeof ARM64_MACOS
  | typeof X86_64_MACOS
  | typeof X86_64_LINUX;

function createBinaryenDownloadUrl(
  target: BinaryenTarget,
): string {
  return `${BINARYEN_RELEASE_116}-${target}.tar.gz`;
}

// To simplify cross-platform installation, we use cargo to install wasm-opt
async function installWasmOpt() {
  const tmpDir = await Deno.makeTempDir();

  let target: BinaryenTarget | null = null;
  switch (Deno.build.os) {
    case `windows`: {
      throw new Error(`Windows is not supported yet.`);
    }
    case `darwin`: {
      switch (Deno.build.arch) {
        case `x86_64`:
          target = X86_64_MACOS;
          break;
        case `aarch64`:
          target = ARM64_MACOS;
          break;
        default:
          throw new Error(`Unsupported architecture ${Deno.build.arch}`);
      }
      break;
    }
    default: {
      // We just run X86_64_LINUX on all other platforms and architectures
      target = X86_64_LINUX;
    }
  }

  // Should never happen, but for typedef
  if (!target) {
    throw new Error(`Could not determine target for wasm-opt installation.`);
  }

  const downloadUrl = createBinaryenDownloadUrl(target);
  await download(downloadUrl, {
    dir: tmpDir,
    file: `binaryen-version_116`,
  });
  await Logger.handleOutput(
    new Deno.Command(`tar`, {
      args: [
        `-xzf`,
        `binaryen-version_116`,
      ],
      cwd: tmpDir,
      stderr: `piped`,
      stdout: `piped`,
    }).spawn(),
  );
  await Deno.chmod(
    path.join(tmpDir, `binaryen-version_116`, `bin`, `wasm-opt`),
    0o755,
  );
  // the whole directory needs to be moved due to include files
  await Deno.rename(
    path.join(tmpDir, `binaryen-version_116`),
    `/usr/local/binaryen-version_116`,
  );
  await Deno.symlink(
    path.join(`/usr/local/binaryen-version_116`, `bin`, `wasm-opt`),
    `/usr/local/bin/wasm-opt`,
  );
  await Deno.remove(tmpDir, { recursive: true });
}

async function installCProject(
  githubRepoUrl: string,
  githubRepoName: string,
  resetToHash: string,
  binaryName: string,
) {
  const tempDirPath = await Deno.makeTempDir();
  await Logger.handleOutput(
    new Deno.Command(`git`, {
      args: [
        `clone`,
        githubRepoUrl,
      ],
      cwd: tempDirPath,
      stderr: `piped`,
      stdout: `piped`,
    }).spawn(),
  );
  await Logger.handleOutput(
    new Deno.Command(`git`, {
      args: [
        `reset`,
        `--hard`,
        resetToHash,
      ],
      cwd: path.join(tempDirPath, githubRepoName),
      stderr: `piped`,
      stdout: `piped`,
    }).spawn(),
  );
  await Logger.handleOutput(
    new Deno.Command(`make`, {
      cwd: path.join(tempDirPath, githubRepoName),
      stderr: `piped`,
      stdout: `piped`,
    }).spawn(),
  );

  switch (Deno.build.os) {
    case "windows":
      throw new Error(`Windows is not supported yet.`);
    default: {
      await Deno.chmod(
        path.join(tempDirPath, githubRepoName, binaryName),
        0o755,
      );
      await Deno.rename(
        path.join(tempDirPath, githubRepoName, binaryName),
        `/usr/local/bin/${binaryName}`,
      );
      break;
    }
  }
  await Deno.remove(tempDirPath, { recursive: true });
}

async function installHookCleaner() {
  return await installCProject(
    `https://github.com/XRPLF/hook-cleaner-c`,
    `hook-cleaner-c`,
    `b856a3614c00361f108d07379f5892e7347bb994`,
    `hook-cleaner`,
  );
}

async function installGuardChecker() {
  return await installCProject(
    `https://github.com/RichardAH/guard-checker`,
    `guard-checker`,
    `de69e8aa054d49612dda7046962003beb88c0749`,
    `guard_checker`,
  );
}

export async function installWasmPack() {
  const tmpDir = await Deno.makeTempDir();
  await download(
    `https://rustwasm.github.io/wasm-pack/installer/init.sh`,
    {
      dir: tmpDir,
    },
  );
  await Logger.handleOutput(
    new Deno.Command(`sh`, {
      args: [
        path.join(tmpDir, `init.sh`),
      ],
      stderr: `piped`,
      stdout: `piped`,
    }).spawn(),
  );
  await Deno.remove(tmpDir, { recursive: true });
}

async function installWasm2Wat() {
  const tempDirPath = await Deno.makeTempDir();

  switch (Deno.build.os) {
    case "windows":
      throw new Error(`Windows is not supported yet.`);
    default:
      switch (Deno.build.os) {
        case `darwin`: {
          const downloadedFile = await download(
            "https://github.com/WebAssembly/wabt/releases/download/1.0.34/wabt-1.0.34-macos-12.tar.gz",
            {
              dir: tempDirPath,
              file: `wabt-1.0.34-macos-12.tar.gz`,
            },
          );
          await Logger.handleOutput(
            new Deno.Command(`tar`, {
              args: [
                `-xzf`,
                downloadedFile.fullPath,
              ],
              stderr: `piped`,
              stdout: `piped`,
            }).spawn(),
          );
          break;
        }
        default: {
          await download(
            // just try ubuntu for now
            "https://github.com/WebAssembly/wabt/releases/download/1.0.34/wabt-1.0.34-ubuntu.tar.gz",
            {
              dir: tempDirPath,
              file: `wabt-1.0.34-ubuntu.tar.gz`,
            },
          );
          await Logger.handleOutput(
            new Deno.Command(`tar`, {
              args: [
                `-xzf`,
                `wabt-1.0.34-ubuntu.tar.gz`,
              ],
              cwd: tempDirPath,
              stderr: `piped`,
              stdout: `piped`,
            }).spawn(),
          );
          break;
        }
      }
      await Deno.chmod(
        path.join(tempDirPath, `wabt-1.0.34`, `bin`, `wasm2wat`),
        0o755,
      );
      // the whole directory needs to be moved due to include files
      await Deno.rename(
        path.join(tempDirPath, `wabt-1.0.34`),
        `/usr/local/wabt-1.0.34`,
      );
      await Deno.symlink(
        path.join(`/usr/local/wabt-1.0.34`, `bin`, `wasm2wat`),
        `/usr/local/bin/wasm2wat`,
      );
  }
  await Deno.remove(tempDirPath, { recursive: true });
}

export async function installPrerequisite(
  prerequisite: keyof PrerequisitesInstallationStatus,
) {
  switch (prerequisite) {
    // These dependencies are too hard to install automatically
    case "git":
      throw new Error(
        `You do not have git installed, but you need to install it manually. 
Refer to https://git-scm.com/book/en/v2/Getting-Started-Installing-Git for more information.`,
      );
    case "cargo":
      throw new Error(
        `You do not have cargo installed, but you need to install it manually. 
Refer to https://forge.rust-lang.org/infra/other-installation-methods.html for more information.`,
      );
    case "wasm-opt":
      Logger.log(`info`, `Installing wasm-opt`);
      await installWasmOpt();
      Logger.log(`success`, `Installed wasm-opt`);
      break;
    case "hook-cleaner":
      Logger.log(`info`, `Installing hook-cleaner`);
      await installHookCleaner();
      Logger.log(`success`, `Installed hook-cleaner`);
      break;
    case "wasm2wat":
      Logger.log(`info`, `Installing wasm2wat`);
      await installWasm2Wat();
      Logger.log(`success`, `Installed wasm2wat`);
      break;
    case "guard_checker":
      Logger.log(`info`, `Installing guard_checker`);
      await installGuardChecker();
      Logger.log(`success`, `Installed guard_checker`);
      break;
    case `wasm-pack`:
      Logger.log(`info`, `Installing wasm-pack`);
      await installWasmPack();
      Logger.log(`success`, `Installed wasm-pack`);
      break;
    default:
      throw new Error(`Unknown prerequisite ${prerequisite}.`);
  }
}

async function uninstallBinary(
  binaryName: string,
) {
  await Deno.remove(`/usr/local/bin/${binaryName}`);
}

async function uninstallWasmOpt() {
  await Deno.remove(`/usr/local/binaryen-version_116`, { recursive: true });
  await uninstallBinary(`wasm-opt`);
}

async function uninstallWasm2wat() {
  await Deno.remove(`/usr/local/wabt-1.0.34`, { recursive: true });
  await uninstallBinary(`wasm2wat`);
}

export async function uninstallPrerequisite(
  prerequisite: keyof PrerequisitesInstallationStatus,
) {
  switch (prerequisite) {
    case "wasm-opt":
      Logger.log(`info`, `Uninstalling wasm-opt`);
      await uninstallWasmOpt();
      Logger.log(`success`, `Uninstalled wasm-opt`);
      break;
    case "hook-cleaner":
      Logger.log(`info`, `Uninstalling hook-cleaner`);
      await uninstallBinary(`hook-cleaner`);
      Logger.log(`success`, `Uninstalled hook-cleaner`);
      break;
    case "wasm2wat":
      Logger.log(`info`, `Uninstalling wasm2wat`);
      await uninstallWasm2wat();
      Logger.log(`success`, `Uninstalled wasm2wat`);
      break;
    case "guard_checker":
      Logger.log(`info`, `Uninstalling guard_checker`);
      await uninstallBinary(`guard_checker`);
      Logger.log(`success`, `Uninstalled guard_checker`);
      break;
    case `wasm-pack`:
      // wasm-pack installation location is hard to be known
      // depending on the machine, so we just leave it for now
      Logger.log(
        `warn`,
        `Note: wasm-pack installation detected. If you want, you need to uninstall wasm-pack yourself.`,
      );
      break;
    default:
      // Do nothing
  }
}
