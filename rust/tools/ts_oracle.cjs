#!/usr/bin/env node
const fs = require("fs");
const path = require("path");

function fail(message, code = 1) {
  process.stderr.write(`${message}\n`);
  process.exit(code);
}

function readStdin() {
  return new Promise((resolve, reject) => {
    let data = "";
    process.stdin.setEncoding("utf8");
    process.stdin.on("data", (chunk) => {
      data += chunk;
    });
    process.stdin.on("end", () => resolve(data));
    process.stdin.on("error", reject);
  });
}

const repoRoot = path.resolve(__dirname, "..", "..");
const buildRoot = path.join(repoRoot, "build");

function requireBuiltModule(moduleName) {
  const candidate = path.join(buildRoot, moduleName);
  if (!fs.existsSync(`${candidate}.js`) && !fs.existsSync(path.join(candidate, "index.js"))) {
    fail(
      `TypeScript build artifacts not found for '${moduleName}'. Run 'npm install --ignore-scripts' then 'npx tsc -p tsconfig.json'.`
    );
  }
  return require(candidate);
}

function normalizePathMaybe(inputPath) {
  if (!inputPath) {
    return inputPath;
  }
  return path.isAbsolute(inputPath) ? inputPath : path.resolve(repoRoot, inputPath);
}

function mapError(error) {
  return {
    name: error?.name || "Error",
    message: error?.message || String(error),
    stack: typeof error?.stack === "string" ? error.stack : undefined,
  };
}

function handleJson(request) {
  const mod = requireBuiltModule("json");
  switch (request.operation) {
    case "parse":
      return mod.parse(request.input);
    case "build":
      return mod.build(request.value);
    default:
      throw new Error(`Unsupported json operation: ${request.operation}`);
  }
}

function handleScheme(request) {
  const mod = requireBuiltModule("scheme");
  switch (request.operation) {
    case "parse":
      return mod.parse(request.input);
    case "build":
      return mod.build(request.value);
    case "parseManagement":
      return mod.parseManagement(request.input);
    case "buildManagement":
      return mod.buildManagement(request.value);
    default:
      throw new Error(`Unsupported scheme operation: ${request.operation}`);
  }
}

function handleWorkspace(request) {
  const mod = requireBuiltModule("workspace");
  switch (request.operation) {
    case "parse":
      return mod.parse(request.input);
    case "build":
      return mod.build(request.value);
    case "parseChecks":
      return mod.parseChecks(request.input);
    case "buildChecks":
      return mod.buildChecks(request.value);
    default:
      throw new Error(`Unsupported workspace operation: ${request.operation}`);
  }
}

function handleXcconfig(request) {
  const mod = requireBuiltModule("xcconfig");
  switch (request.operation) {
    case "parse":
      return mod.parse(request.input);
    case "parseFile":
      return mod.parseFile(normalizePathMaybe(request.filePath));
    case "build":
      return mod.build(request.value);
    case "flattenBuildSettings":
      return mod.flattenBuildSettings(request.value, request.options || {});
    default:
      throw new Error(`Unsupported xcconfig operation: ${request.operation}`);
  }
}

function handleBreakpoints(request) {
  const mod = requireBuiltModule("breakpoints");
  switch (request.operation) {
    case "parse":
      return mod.parse(request.input);
    case "build":
      return mod.build(request.value);
    default:
      throw new Error(`Unsupported breakpoints operation: ${request.operation}`);
  }
}

function handleSettings(request) {
  const mod = requireBuiltModule("settings");
  switch (request.operation) {
    case "parse":
      return mod.parse(request.input);
    case "build":
      return mod.build(request.value);
    default:
      throw new Error(`Unsupported settings operation: ${request.operation}`);
  }
}

function handleApi(request) {
  const mod = requireBuiltModule("api");

  switch (request.operation) {
    case "projectSummary": {
      const project = new mod.XcodeProject(
        request.filePath || "/tmp/project.pbxproj",
        request.project
      );

      return {
        archiveVersion: project.archiveVersion,
        objectVersion: project.objectVersion,
        rootObjectUuid: project.rootObject.uuid,
        rootObjectIsa: project.rootObject.isa,
        objectCount: project.size,
      };
    }

    case "createModelAndDelete": {
      const project = new mod.XcodeProject(
        request.filePath || "/tmp/project.pbxproj",
        request.project
      );

      const model = project.createModel(request.model);
      const referrersBefore = project
        .getReferrers(model.uuid)
        .map((item) => item.uuid)
        .sort();

      const deleted = project.delete(model.uuid);
      return {
        createdUuid: model.uuid,
        deleted,
        existsAfterDelete: project.has(model.uuid),
        referrersBefore,
        objectCountAfter: project.size,
      };
    }

    case "getReferrers": {
      const project = new mod.XcodeProject(
        request.filePath || "/tmp/project.pbxproj",
        request.project
      );
      return project
        .getReferrers(request.uuid)
        .map((item) => ({ uuid: item.uuid, isa: item.isa }))
        .sort((a, b) => a.uuid.localeCompare(b.uuid));
    }

    default:
      throw new Error(`Unsupported api operation: ${request.operation}`);
  }
}

function dispatch(request) {
  switch (request.module) {
    case "json":
      return handleJson(request);
    case "scheme":
      return handleScheme(request);
    case "workspace":
      return handleWorkspace(request);
    case "xcconfig":
      return handleXcconfig(request);
    case "breakpoints":
      return handleBreakpoints(request);
    case "settings":
      return handleSettings(request);
    case "api":
      return handleApi(request);
    default:
      throw new Error(`Unsupported module: ${request.module}`);
  }
}

(async () => {
  try {
    const raw = (await readStdin()).trim();
    if (!raw) {
      fail("Expected JSON request on stdin");
    }

    const request = JSON.parse(raw);
    const result = dispatch(request);
    process.stdout.write(JSON.stringify({ ok: true, result }));
  } catch (error) {
    process.stdout.write(JSON.stringify({ ok: false, error: mapError(error) }));
    process.exitCode = 1;
  }
})();
