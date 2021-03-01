<div align="center">
  <h1><code>tree-sitter-facade</code></h1>
  <p>
    <strong>A uniform interface for tree-sitter (rust) and web-tree-sitter (wasm-bindgen)</strong>
  </p>
  <p style="margin-bottom: 0.5ex;">
    <a href="https://hvithrafn.github.io/tree-sitter-facade/tree_sitter_facade"><img
        src="https://img.shields.io/badge/docs-latest-blueviolet?logo=Read-the-docs&logoColor=white"
        /></a>
    <a href="https://github.com/hvithrafn/tree-sitter-facade/actions"><img
        src="https://github.com/hvithrafn/tree-sitter-facade/workflows/main/badge.svg"
        /></a>
    <!-- <a href="https://codecov.io/gh/hvithrafn/tree-sitter-facade"><img
        src="https://codecov.io/gh/hvithrafn/tree-sitter-facade/branches/main/graph/badge.svg"
        /></a> -->
  </p>
</div>

## Description

This crate is intended to be used for projects that use
[`tree-sitter`](https://github.com/tree-sitter) and which target both native and
web platforms (via [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen)).
