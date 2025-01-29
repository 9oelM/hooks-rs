import { parse } from "jsr:@std/toml@1.0.2";

interface MinimalCargoToml {
  package: {
    name: string;
  };
}

export function isMinimalCargoToml(
  // any is meant to be used because the type of the parsed TOML is not known
  // deno-lint-ignore no-explicit-any
  parsedCargoToml: any,
): parsedCargoToml is MinimalCargoToml {
  return `package` in parsedCargoToml &&
    typeof parsedCargoToml[`package`] === "object" &&
    `name` in parsedCargoToml[`package`];
}

export async function readCargoToml() {
  const rawCargoToml = await Deno.readFile("Cargo.toml");
  const parsedCargoToml = parse(new TextDecoder().decode(rawCargoToml));

  return parsedCargoToml;
}
