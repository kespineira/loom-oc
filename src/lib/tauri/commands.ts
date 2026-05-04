import { invoke } from "@tauri-apps/api/core";
import type { OpenCodeConfig } from "$lib/schema/opencode";

export type ConfigPaths = {
  global: string;
  global_exists: boolean;
  project: string | null;
  project_exists: boolean;
};

export type ConfigTarget = "global" | "project";

export type ReadResult = {
  path: string;
  exists: boolean;
  raw: string | null;
  value: OpenCodeConfig | null;
  revision: string | null;
  mtime_ms: number | null;
};

export type WriteResult = {
  path: string;
  revision: string;
  mtime_ms: number | null;
  backup_path: string | null;
};

export function configPaths(cwd?: string): Promise<ConfigPaths> {
  return invoke<ConfigPaths>("config_paths", { cwd: cwd ?? null });
}

export function readConfig(target: ConfigTarget, cwd?: string): Promise<ReadResult> {
  return invoke<ReadResult>("read_config", { target, cwd: cwd ?? null });
}

export function writeConfig(
  target: ConfigTarget,
  value: OpenCodeConfig,
  expectedRevision: string | null,
  cwd?: string,
): Promise<WriteResult> {
  return invoke<WriteResult>("write_config", {
    target,
    cwd: cwd ?? null,
    value,
    expectedRevision,
  });
}

export function appVersion(): Promise<string> {
  return invoke<string>("app_version");
}
