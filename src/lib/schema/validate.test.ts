import { describe, expect, it } from "vitest";
import { validateConfig } from "./validate";

describe("validateConfig", () => {
  it("accepts a minimal OpenCode config", () => {
    const result = validateConfig({ $schema: "https://opencode.ai/config.json" });

    expect(result.valid).toBe(true);
    expect(result.issues).toEqual([]);
  });

  it("reports invalid properties", () => {
    const result = validateConfig({ unknown: true } as never);

    expect(result.valid).toBe(false);
    expect(result.issues[0]?.message).toContain("Unknown property");
  });
});
