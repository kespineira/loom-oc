import Ajv2020, { type ErrorObject } from "ajv/dist/2020";
import schema from "./opencode.schema.json";
import type { OpenCodeConfig } from "./opencode";

export type ValidationIssue = {
  path: string;
  message: string;
};

export type ValidationResult = {
  valid: boolean;
  issues: ValidationIssue[];
};

const ajv = new Ajv2020({ allErrors: true, strict: false });

ajv.addSchema({
  $id: "https://models.dev/model-schema.json",
  $defs: {
    Model: { type: "string" },
  },
});

const validate = ajv.compile(schema);

export function validateConfig(config: OpenCodeConfig): ValidationResult {
  const valid = validate(config);
  return {
    valid,
    issues: valid ? [] : (validate.errors ?? []).map(formatIssue),
  };
}

function formatIssue(error: ErrorObject): ValidationIssue {
  const path = error.instancePath || "config";
  if (error.keyword === "additionalProperties" && "additionalProperty" in error.params) {
    return {
      path,
      message: `Unknown property "${String(error.params.additionalProperty)}"`,
    };
  }
  if (error.keyword === "enum" && "allowedValues" in error.params) {
    return {
      path,
      message: `Expected one of ${error.params.allowedValues.map(String).join(", ")}`,
    };
  }
  return {
    path,
    message: error.message ?? "Invalid value",
  };
}
