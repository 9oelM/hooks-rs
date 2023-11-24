import { colors } from "https://deno.land/x/cliffy@v1.0.0-rc.3/ansi/colors.ts";

export class Logger {
  private static success = colors.bold.green;
  private static error = colors.bold.red;
  private static warn = colors.bold.yellow;
  private static info = colors.bold.blue;

  public static log(
    level: "success" | "error" | "warn" | "info" = "info",
    message: string,
  ) {
    const color = this[level];

    console.log(color(message));
  }

  public static async handleOutput(
    process: Deno.ChildProcess,
    successOutput = true,
  ) {
    if (successOutput) await this.readStream(process.stdout);
    // Some commands write to stderr even if they succeed,
    // so we choose a middle ground as warn
    await this.readStream(process.stderr, `warn`);

    // Wait for the command to complete
    await process.status;
    // await process.stdout.cancel();
    // await process.stderr.cancel();
  }

  public static async readStream(
    stream: ReadableStream<Uint8Array>,
    logLevel: `info` | `warn` = `info`,
  ): Promise<void> {
    const reader = stream.getReader();
    const decoder = new TextDecoder();
    let done = false;
    do {
      const result = await reader.read();
      done = result.done;

      if (result.value) {
        this.log(logLevel, decoder.decode(result.value));
      }
    } while (!done);
  }
}
