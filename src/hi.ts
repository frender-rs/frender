import { formatGreeting, Punctuation } from "./util/format";

export default function hi(
  name: string,
  punctuation: Punctuation = ".",
): string {
  return formatGreeting("hi", name, punctuation);
}
