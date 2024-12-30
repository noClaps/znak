import {
  bundledLanguages,
  highlight,
  type BundledLanguage,
} from "@noclaps/highlight";
import type { CodeTheme } from "../../index.ts";

export const githubTheme: CodeTheme = {
  bg: "#0d1117ff",
  fg: "#f0f6fcff",
  highlights: {
    boolean: {
      color: "#79c0ffff",
    },
    comment: {
      color: "#9198a1ff",
    },
    "comment.doc": {
      color: "#9198a1ff",
    },
    constant: {
      color: "#79c0ffff",
    },
    constructor: {
      color: "#79c0ffff",
    },
    embedded: {
      color: "#ff7b72ff",
    },
    emphasis: {
      fontStyle: "italic",
    },
    "emphasis.strong": {
      fontWeight: 700,
    },
    enum: {
      color: "#ffa657ff",
    },
    function: {
      color: "#d2a8ffff",
    },
    "function.method": {
      color: "#d2a8ffff",
    },
    "function.special.definition": {
      color: "#d2a8ffff",
    },
    hint: {
      color: "#9198a1ff",
      fontWeight: 700,
    },
    keyword: {
      color: "#ff7b72ff",
    },
    label: {
      color: "#79c0ffff",
    },
    link_text: {
      color: "#a5d6ffff",
      fontStyle: "italic",
    },
    link_uri: {
      color: "#79c0ffff",
    },
    number: {
      color: "#79c0ffff",
    },
    operator: {
      color: "#f0f6fcff",
    },
    predictive: {
      color: "#9198a1ff",
      fontStyle: "italic",
    },
    preproc: {
      color: "#ff7b72ff",
    },
    primary: {
      color: "#f0f6fcff",
    },
    property: {
      color: "#79c0ffff",
    },
    punctuation: {
      color: "#f0f6fcff",
    },
    "punctuation.bracket": {
      color: "#f0f6fcff",
    },
    "punctuation.delimiter": {
      color: "#f0f6fcff",
    },
    "punctuation.list_marker": {
      color: "#ffa657ff",
    },
    "punctuation.special": {
      color: "#ff7b72ff",
    },
    string: {
      color: "#a5d6ffff",
    },
    "string.escape": {
      color: "#7ee787ff",
      fontWeight: 700,
    },
    "string.regex": {
      color: "#a5d6ffff",
    },
    "string.special": {
      color: "#79c0ffff",
    },
    "string.special.symbol": {
      color: "#79c0ffff",
    },
    tag: {
      color: "#7ee787ff",
    },
    "text.literal": {
      color: "#79c0ffff",
    },
    title: {
      color: "#79c0ffff",
      fontWeight: 700,
    },
    type: {
      color: "#ffa657ff",
    },
    variable: {
      color: "#f0f6fcff",
    },
    "variable.special": {
      color: "#ff7b72ff",
    },
    variant: {
      color: "#ffa657ff",
    },
  },
};

export function highlightSyntax(
  code: string,
  theme?: CodeTheme,
  lang?: BundledLanguage,
): HastText {
  return {
    type: "text",
    value: highlight(code, lang || "plaintext", theme),
  };
}
