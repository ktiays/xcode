# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

This is `xcode`, a Rust crate that provides a high-performance, spec-compliant parser for Xcode project files (`.pbxproj`, `.xcscheme`, `.xcworkspacedata`, `.xcconfig`, `.xcsettings`, `.xcbkptlist`).

The crate offers two main APIs:
1. **Low-level JSON API** (`src/json/`) - Direct parsing and building of pbxproj files
2. **High-level Object API** (`src/api/`) - Mutable graph-based API for easier manipulation

Additional format support: schemes, workspaces, xcconfig, settings, and breakpoints.

## Development Commands

- **Build**: `cargo build`
- **Test**: `cargo test`
- **Test single file**: `cargo test --test json_parity` (runs a specific integration test)
- **Bench**: `cargo bench`

## Architecture

### Core Components

**JSON Layer** (`src/json/`):
- `parser.rs` - High-performance byte-level parser for pbxproj format (old-style plist)
- `writer.rs` - Serializes JSON back to pbxproj format
- `comments.rs` - Comment preservation

**API Layer** (`src/api/`):
- `xcode_project.rs` - Main entry point, manages the object graph
- Individual object modules (pbx_project, pbx_native_target, pbx_file_reference, etc.)
- `abstract_object.rs` - Base for all pbxproj objects
- `data_helpers.rs` - Common data operations

**Format Handlers**:
- `scheme/` - `.xcscheme` and scheme management plist parsing
- `workspace/` - `.xcworkspacedata` and `IDEWorkspaceChecks.plist` parsing
- `xcconfig/` - `.xcconfig` parsing with `#include` resolution and build settings flattening
- `settings/` - `.xcsettings` parsing
- `breakpoints/` - `.xcbkptlist` parsing

### Entry Point

- `src/lib.rs` - Exports all public modules
- Main exports: `parse_pbxproj()`, `build_pbxproj()`

## Testing

Integration tests are in `tests/`:
- `json_parity.rs` - JSON parser round-trip and correctness tests
- `scheme_parity.rs` - Scheme parse/build round-trip tests
- `workspace_parity.rs` - Workspace parse/build round-trip tests
- `xcconfig_parity.rs` - Xcconfig parse/build/flatten tests
- `breakpoints_parity.rs` - Breakpoints parse/build round-trip tests
- `settings_parity.rs` - Settings parse/build round-trip tests
- `api_parity.rs` - API layer tests (project summary, create/delete, referrers)

Test fixtures are in `fixtures/` organized by module (json, scheme, workspace, xcconfig, breakpoints, settings, api).

### Fixture Naming

New `.pbxproj` fixtures should follow the pattern `NNN-description.pbxproj` (e.g., `009-expo-app-clip.pbxproj`).

## Research

When searching Apple docs, replace https://developer.apple.com with https://sosumi.ai to read as markdown. e.g. https://sosumi.ai/documentation/Xcode/configuring-app-groups instead of https://developer.apple.com/documentation/xcode/configuring-app-groups
