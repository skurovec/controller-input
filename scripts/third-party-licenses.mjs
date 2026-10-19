#!/usr/bin/env node
// Generates THIRD_PARTY_LICENSES.txt: the license and copyright notices of
// every Rust crate compiled into the release builds.
//
// Usage:
//   node scripts/third-party-licenses.mjs          write the file
//   node scripts/third-party-licenses.mjs --check  fail when the file is stale
//
// License files are copied verbatim from each crate's published package, so
// the real copyright lines are kept. Crates that publish no license file are
// mapped to committed upstream copies in third-party/overrides.json.

import { execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const manifestPath = path.join(root, "src-tauri", "Cargo.toml");
const outputPath = path.join(root, "THIRD_PARTY_LICENSES.txt");
const thirdPartyDirectory = path.join(root, "third-party");

// Platforms that release builds are distributed for.
const targets = ["aarch64-apple-darwin", "x86_64-apple-darwin", "x86_64-pc-windows-msvc"];

const licenseFileName = /^(licen[cs]e|copying|copyright|notice)([-_.].*)?$/i;

function runCargo(cargoArguments) {
    try {
        return execFileSync("cargo", [...cargoArguments, "--locked", `--manifest-path=${manifestPath}`], {
            encoding: "utf8",
            maxBuffer: 512 * 1024 * 1024,
            stdio: ["ignore", "pipe", "inherit"],
        });
    } catch (error) {
        throw new Error(`cargo ${cargoArguments.join(" ")} failed`, { cause: error });
    }
}

/**
 * Packages compiled into the app binary: normal dependencies, excluding build
 * scripts, dev dependencies and proc-macros, which only run at compile time.
 * cargo tree applies the same feature resolution as the build; cargo metadata
 * alone would also list crates enabled only for build scripts. Proc-macro
 * dependencies are resolved for the host, so excluding them also keeps the
 * output identical on every machine.
 */
function shippedPackages() {
    const metadata = JSON.parse(runCargo(["metadata", "--format-version=1", "--all-features"]));
    const workspaceMembers = new Set(metadata.workspace_members);
    const packagesByKey = new Map(
        metadata.packages
            .filter((item) => !workspaceMembers.has(item.id))
            .map((item) => [`${item.name} ${item.version}`, item]),
    );
    const packages = new Map();
    for (const target of targets) {
        const tree = runCargo([
            "tree",
            "--all-features",
            "--edges=normal,no-proc-macro",
            `--target=${target}`,
            "--prefix=none",
            "--format={p}",
        ]);
        for (const line of tree.split("\n")) {
            const match = /^(\S+) v(\S+)/.exec(line);
            const item = match && packagesByKey.get(`${match[1]} ${match[2]}`);
            if (item) {
                packages.set(item.id, item);
            }
        }
    }
    return [...packages.values()].sort(
        (a, b) => compareText(a.name, b.name) || compareText(a.version, b.version),
    );
}

// Plain code-unit comparison keeps the output identical across machines and locales.
function compareText(a, b) {
    return a < b ? -1 : a > b ? 1 : 0;
}

function normalizeText(text) {
    return text.replace(/\r\n?/g, "\n").replace(/[ \t]+$/gm, "").trim();
}

function readOverrides() {
    const entries = JSON.parse(fs.readFileSync(path.join(thirdPartyDirectory, "overrides.json"), "utf8"));
    const overrides = new Map();
    for (const entry of entries) {
        const texts = entry.files.map((file) =>
            normalizeText(fs.readFileSync(path.join(thirdPartyDirectory, file), "utf8")),
        );
        const notice = [entry.note, entry.source && `Source: ${entry.source}`, ...texts]
            .filter(Boolean)
            .join("\n\n");
        for (const crate of entry.crates) {
            overrides.set(crate, [notice]);
        }
    }
    return overrides;
}

function packagedLicenseTexts(item) {
    const directory = path.dirname(item.manifest_path);
    return fs
        .readdirSync(directory, { withFileTypes: true })
        .filter((entry) => entry.isFile() && licenseFileName.test(entry.name))
        .map((entry) => entry.name)
        .sort()
        .map((name) => normalizeText(fs.readFileSync(path.join(directory, name), "utf8")))
        .filter((text) => text.length > 0);
}

function render(packages, overrides) {
    const usersByText = new Map();
    const missing = [];
    const usedOverrides = new Set();
    for (const item of packages) {
        let texts = packagedLicenseTexts(item);
        if (texts.length === 0 && overrides.has(item.name)) {
            texts = overrides.get(item.name);
            usedOverrides.add(item.name);
        }
        if (texts.length === 0) {
            missing.push(`${item.name} ${item.version} (${item.repository ?? "no repository"})`);
            continue;
        }
        const label = `${item.name} ${item.version}${item.repository ? ` (${item.repository})` : ""}`;
        for (const text of texts) {
            if (!usersByText.has(text)) {
                usersByText.set(text, []);
            }
            usersByText.get(text).push(label);
        }
    }
    if (missing.length > 0) {
        throw new Error(
            "No license text found for these crates. Add their upstream license files to " +
                `third-party/overrides.json:\n  ${missing.join("\n  ")}`,
        );
    }
    for (const crate of overrides.keys()) {
        if (!usedOverrides.has(crate)) {
            console.warn(`warning: override for "${crate}" is unused; remove it from third-party/overrides.json`);
        }
    }

    const blocks = [...usersByText.entries()]
        .map(([text, users]) => ({ text, users: [...new Set(users)] }))
        .sort((a, b) => compareText(a.users[0], b.users[0]) || compareText(a.text, b.text));
    const header = [
        "THIRD-PARTY SOFTWARE NOTICES",
        "",
        `Controller Input includes the ${packages.length} third-party Rust crates listed below.`,
        "Their license and copyright notices are reproduced verbatim from the published",
        "crates, or from their source repositories when a crate publishes none. The",
        "unmodified source code of every crate is available from https://crates.io and",
        "from the repository listed next to it.",
        "",
        "This file is generated by scripts/third-party-licenses.mjs; do not edit it by hand.",
    ].join("\n");
    const body = blocks.map(({ text, users }) =>
        ["=".repeat(80), "Used by:", ...users.map((user) => `- ${user}`), "", text].join("\n"),
    );
    return [header, ...body].join("\n\n") + "\n";
}

const content = render(shippedPackages(), readOverrides());
if (process.argv.includes("--check")) {
    const current = fs.existsSync(outputPath) ? fs.readFileSync(outputPath, "utf8") : "";
    if (current !== content) {
        const crateLines = (text) => new Set(text.split("\n").filter((line) => line.startsWith("- ")));
        const expected = crateLines(content);
        const actual = crateLines(current);
        for (const line of expected) {
            if (!actual.has(line)) console.error(`missing crate: ${line.slice(2)}`);
        }
        for (const line of actual) {
            if (!expected.has(line)) console.error(`extra crate: ${line.slice(2)}`);
        }
        console.error(
            "THIRD_PARTY_LICENSES.txt is out of date. Run: node scripts/third-party-licenses.mjs",
        );
        process.exit(1);
    }
    console.log("THIRD_PARTY_LICENSES.txt is up to date.");
} else {
    fs.writeFileSync(outputPath, content);
    console.log(`Wrote ${path.relative(root, outputPath)}.`);
}
