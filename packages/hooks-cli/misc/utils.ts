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
