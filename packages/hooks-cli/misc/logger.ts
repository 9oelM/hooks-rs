import { colors } from "jsr:@cliffy/ansi@1.0.0-rc.7/colors";

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
    if (successOutput) {
      await this.readStream(process.stdout);
    } else {
      await process.stdout.cancel();
    }
    // Some commands write to stderr even if they succeed,
    // so we choose a middle ground as warn
    await this.readStream(process.stderr, `warn`);

    // Wait for the command to complete
    await process.status;

    // TODO: idk wth is going on here
    if (!process.stdout.locked) {
      await process.stdout.cancel();
    }
    if (!process.stderr.locked) {
      await process.stderr.cancel();
    }
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
    await reader.cancel();
  }
}
