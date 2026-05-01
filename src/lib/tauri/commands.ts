import { invoke } from "@tauri-apps/api/core";
import type { OpenCodeConfig } from "$lib/schema/opencode";

export type ConfigPaths = {
  global: string;
  global_exists: boolean;
  project: string | null;
  project_exists: boolean;
};

export type ReadResult = {
  path: string;
  exists: boolean;
  raw: string | null;
  value: OpenCodeConfig | null;
};

export function configPaths(cwd?: string): Promise<ConfigPaths> {
  return invoke<ConfigPaths>("config_paths", { cwd: cwd ?? null });
}

export function readConfig(path: string): Promise<ReadResult> {
  return invoke<ReadResult>("read_config", { path });
}

export function writeConfig(path: string, value: OpenCodeConfig): Promise<void> {
  return invoke<void>("write_config", { path, value });
}

export function appVersion(): Promise<string> {
  return invoke<string>("app_version");
}
