import { formatGreeting, Punctuation } from "../util/format";

export default function bye(
  name: string,
  punctuation: Punctuation = "~",
): string {
  return formatGreeting("bye", name, punctuation);
}
