import { parse } from "https://deno.land/std@0.202.0/toml/mod.ts";

interface MinimalCargoToml {
  package: {
    name: string,
  },
}

export function isMinimalCargoToml(parsedCargoToml: any): parsedCargoToml is MinimalCargoToml {
  return `package` in parsedCargoToml &&
    typeof parsedCargoToml[`package`]  === "object" &&
    `name` in parsedCargoToml[`package`]
}

export async function readCargoToml() {
  const rawCargoToml = await Deno.readFile("Cargo.toml")
  const parsedCargoToml = parse(new TextDecoder().decode(rawCargoToml))

  return parsedCargoToml
}
