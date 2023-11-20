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
}
