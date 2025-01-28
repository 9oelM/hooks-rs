export function enumToMap<T extends string>(
  myEnum: { [key: string]: T },
): Map<T, string> {
  const map = new Map<T, string>();
  Object.keys(myEnum).forEach((key) => {
    const value = myEnum[key];
    map.set(value, key);
  });
  return map;
}

export async function pathExists(path: string): Promise<boolean> {
  try {
    await Deno.stat(path); // Check the path
    return true; // Path exists
  } catch (error) {
    if (error instanceof Deno.errors.NotFound) {
      return false; // Path does not exist
    }
    throw error; // Re-throw for other errors
  }
}