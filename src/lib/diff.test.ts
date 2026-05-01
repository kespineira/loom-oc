import { describe, expect, it } from "vitest";
import { diffJson, lineDiff } from "./diff";

describe("lineDiff", () => {
  it("returns all context lines when inputs are equal", () => {
    const out = lineDiff("a\nb\nc", "a\nb\nc");
    expect(out.every((l) => l.kind === "context")).toBe(true);
    expect(out).toHaveLength(3);
  });

  it("marks added lines", () => {
    const out = lineDiff("a\nb", "a\nb\nc");
    expect(out).toEqual([
      { kind: "context", text: "a" },
      { kind: "context", text: "b" },
      { kind: "add", text: "c" },
    ]);
  });

  it("marks removed lines", () => {
    const out = lineDiff("a\nb\nc", "a\nc");
    expect(out).toEqual([
      { kind: "context", text: "a" },
      { kind: "remove", text: "b" },
      { kind: "context", text: "c" },
    ]);
  });

  it("handles a replacement as remove + add", () => {
    const out = lineDiff("foo", "bar");
    expect(out).toEqual([
      { kind: "remove", text: "foo" },
      { kind: "add", text: "bar" },
    ]);
  });
});

describe("diffJson", () => {
  it("emits no add/remove when objects are equal", () => {
    const out = diffJson({ a: 1 }, { a: 1 });
    expect(out.some((l) => l.kind !== "context")).toBe(false);
  });

  it("shows changed keys as add + remove", () => {
    const out = diffJson({ a: 1 }, { a: 2 });
    expect(out.some((l) => l.kind === "add" && l.text.includes('"a": 2'))).toBe(true);
    expect(out.some((l) => l.kind === "remove" && l.text.includes('"a": 1'))).toBe(true);
  });
});
