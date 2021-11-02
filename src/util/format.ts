export type Punctuation = "!" | "~" | "?" | ".";

export function formatGreeting(
  message: string,
  name: string,
  punctuation: Punctuation = "!",
): string {
  return `${message} ${name}${punctuation}`;
}
