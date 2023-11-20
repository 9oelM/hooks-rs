import { Logger } from "../misc/mod.ts";
import { TypedObjectKeys } from "../types/utils.ts";
// @deno-types="../types/command_exists.d.ts"
import commandExists from "npm:command-exists";
import * as path from "https://deno.land/std@0.207.0/path/mod.ts";
import { download } from "https://deno.land/x/download@v2.0.2/mod.ts";
import { Untar } from "https://deno.land/std@0.207.0/archive/untar.ts";
import { gunzip } from "https://deno.land/x/compress@v0.4.5/zlib/inflate.ts";

export interface PrerequisitesInstallationStatus {
    git: boolean;
    cargo: boolean;
    "wasm-opt": boolean;
    "hook-cleaner": boolean;
    wasm2wat: boolean;
    guard_checker: boolean;
}

async function uncompressTarGz(filePath: string, outputDir: string) {
  // const file = await Deno.open(filePath);
  // const stat = await file.stat();
  // const buffer = new Uint8Array(stat.size);
  // const uncompressed = await gunzip(
  //   buffer
  // );
  // const toBeUntarred: ConstructorParameters<typeof Untar>[0] = {
  //   read(p: Uint8Array): Promise<number | null> {
  //     // p.uncompressed
  //   }
  // };
  // await new Untar(toBeUntarred);
  // file.close();
  // console.log("File uncompressed successfully!");
}


// check inside cargo workplace
export async function installCargo() {
}

export async function checkPrerequisitesInstalled() {
  const prerequisitesInstallationStatus: PrerequisitesInstallationStatus = {
    git: false,
    cargo: false,
    ["wasm-opt"]: false,
    ["hook-cleaner"]: false,
    ["wasm2wat"]: false,
    ["guard_checker"]: false,
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

async function installWasmOpt(
  platform: typeof Deno.build.os,
) {
  
}

async function installHookCleaner(
  platform: typeof Deno.build.os,
) {
  const tempDirPath = await Deno.makeTempDir();
  Logger.handleOutput(await new Deno.Command(`git`, {
    args: [
      `clone`,
      `https://github.com/XRPLF/hook-cleaner-c`
    ],
    cwd: tempDirPath,
  }).output());
  Logger.handleOutput(await new Deno.Command(`git`, {
    args: [
      `reset`,
      `--hard`,
      `b856a3614c00361f108d07379f5892e7347bb994`
    ],
    cwd: path.join(tempDirPath, `hook-cleaner-c`),
  }).output());
  Logger.handleOutput(await new Deno.Command(`make`, {
    cwd: path.join(tempDirPath, `hook-cleaner-c`),
  }).output());
  Logger.handleOutput(await new Deno.Command(`make`, {
    cwd: path.join(tempDirPath, `hook-cleaner-c`),
  }).output());

  switch (platform) {
    case "windows":
      throw new Error(`Windows is not supported yet.`);
    default: {
      await Deno.chmod(path.join(tempDirPath, `hook-cleaner-c`, `hook-cleaner`), 0o755);
      await Deno.rename(path.join(tempDirPath, `hook-cleaner-c`, `hook-cleaner`), `/usr/local/bin/hook-cleaner`);
      break;
    }
  }
}

async function installWasm2wat(
  platform: typeof Deno.build.os,
) {
  const tempDirPath = await Deno.makeTempDir();

  switch (platform) {
    case "windows":
      throw new Error(`Windows is not supported yet.`);
    case `darwin`: {
      // download and save https://github.com/WebAssembly/wabt/releases/download/1.0.34/wabt-1.0.34-macos-12.tar.gz
      const downloadedFile = await download("https://github.com/WebAssembly/wabt/releases/download/1.0.34/wabt-1.0.34-macos-12.tar.gz", {
        dir: tempDirPath,
        file: `wabt-1.0.34-macos-12.tar.gz`
      });
      // unpack tar.gz
      Logger.handleOutput(await new Deno.Command(`tar`, {
        args: [
          `-xzf`,
          downloadedFile.fullPath,
        ],
        cwd: tempDirPath,
      }).output());
      break
    }
    default: {

    }
  }
}

function installGuardChecker(
  platform: typeof Deno.build.os,
) {

}

export async function installPrerequisite(
  prerequisite: keyof PrerequisitesInstallationStatus,
) {
  const platform = Deno.build.os;
  const arch = Deno.build.arch;

  if (arch === "x86_64") {
    throw new Error(`32 bits architecture is not supported.`)
  }

  switch (prerequisite) {
    case "git":
      throw new Error(`You do not have git installed, but you need to install it manually. 
Refer to https://git-scm.com/book/en/v2/Getting-Started-Installing-Git for more information.`)
    case "cargo":
      throw new Error(`You do not have cargo installed, but you need to install it manually. 
Refer to https://forge.rust-lang.org/infra/other-installation-methods.html for more information.`)
    case "wasm-opt":
      // TODO
      break;
    case "hook-cleaner":
      // TODO
      break;
    case "wasm2wat":
      // TODO
      break;
    case "guard_checker":
      // TODO
      break;
  }
}