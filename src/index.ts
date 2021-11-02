export function hello(name: string): string {
  return `hello ${name}!`;
}

export { default as bye } from "./bye";
export { default as hi } from "./hi";
export { Punctuation } from "./util/format";
