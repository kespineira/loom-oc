import { describe, expect, it, vi, beforeEach, afterEach } from "vitest";
import { toasts } from "./toast.svelte";

describe("toasts store", () => {
  beforeEach(() => {
    vi.useFakeTimers();
    while (toasts.items.length) toasts.dismiss(toasts.items[0]!.id);
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("pushes toasts onto the queue", () => {
    toasts.success("saved");
    expect(toasts.items).toHaveLength(1);
    expect(toasts.items[0]!.tone).toBe("success");
    expect(toasts.items[0]!.title).toBe("saved");
  });

  it("auto-dismisses after the duration", () => {
    toasts.info("hi");
    expect(toasts.items).toHaveLength(1);
    vi.advanceTimersByTime(4001);
    expect(toasts.items).toHaveLength(0);
  });

  it("caps the visible queue at 3", () => {
    toasts.info("a");
    toasts.info("b");
    toasts.info("c");
    toasts.info("d");
    expect(toasts.items.map((t) => t.title)).toEqual(["b", "c", "d"]);
  });

  it("dismiss removes a specific toast", () => {
    const id = toasts.warning("careful");
    toasts.dismiss(id);
    expect(toasts.items).toHaveLength(0);
  });
});
