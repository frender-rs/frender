import { hello } from ".";

test("hello world!", () => {
  expect(hello("world")).toBe("hello world!");
});
