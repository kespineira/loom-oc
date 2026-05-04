export type ToastTone = "success" | "warning" | "danger" | "info";

export type Toast = {
  id: number;
  tone: ToastTone;
  title: string;
  description?: string;
  duration: number;
};

const DEFAULT_DURATION_MS = 4000;
const MAX_VISIBLE = 3;

function createToasts() {
  let items = $state<Toast[]>([]);
  let nextId = 0;
  const timers = new Map<number, ReturnType<typeof setTimeout>>();

  function dismiss(id: number) {
    items = items.filter((t) => t.id !== id);
    const timer = timers.get(id);
    if (timer) {
      clearTimeout(timer);
      timers.delete(id);
    }
  }

  function push(input: Omit<Toast, "id" | "duration"> & { duration?: number }): number {
    const id = ++nextId;
    const toast: Toast = {
      id,
      tone: input.tone,
      title: input.title,
      description: input.description,
      duration: input.duration ?? DEFAULT_DURATION_MS,
    };
    const nextItems = [...items, toast].slice(-MAX_VISIBLE);
    for (const item of items) {
      if (!nextItems.some((next) => next.id === item.id)) {
        const timer = timers.get(item.id);
        if (timer) clearTimeout(timer);
        timers.delete(item.id);
      }
    }
    items = nextItems;
    if (toast.duration > 0) {
      timers.set(
        id,
        setTimeout(() => dismiss(id), toast.duration),
      );
    }
    return id;
  }

  return {
    get items() {
      return items;
    },
    push,
    dismiss,
    success: (title: string, description?: string) =>
      push({ tone: "success", title, description }),
    warning: (title: string, description?: string) =>
      push({ tone: "warning", title, description }),
    danger: (title: string, description?: string) =>
      push({ tone: "danger", title, description }),
    info: (title: string, description?: string) => push({ tone: "info", title, description }),
  };
}

export const toasts = createToasts();
