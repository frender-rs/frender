const CARGO_PROJECTS = [
  //
  { dir: "crates/frender-macros", replace: 1, ensure: true },
  { dir: "crates/react", replace: 2, ensure: true },
  { dir: "crates/react-html", replace: 2, ensure: true },
  { dir: "frender", replace: 2 },
].map((pro) => ({ ...pro, file: `${pro.dir}/Cargo.toml` }));

module.exports = {
  branches: [
    "+([0-9])?(.{+([0-9]),x}).x",
    "main",
    "next",
    "next-major",
    { name: "beta", prerelease: true },
    { name: "alpha", prerelease: true },
  ],
  plugins: [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    "@semantic-release/changelog",
    [
      "@google/semantic-release-replace-plugin",
      {
        replacements: [
          {
            files: CARGO_PROJECTS.map((pro) => pro.file),
            from: 'version = ".*" # replace version',
            to: 'version = "${nextRelease.version}" # replace version',
            results: CARGO_PROJECTS.map((pro) => ({
              file: pro.file,
              hasChanged: true,
              numMatches: pro.replace,
              numReplacements: pro.replace,
            })),
            countMatches: true,
          },
        ],
      },
    ],
    ["@semantic-release/exec", { prepareCmd: "cargo check" }],
    ...CARGO_PROJECTS.map((pro) => [
      "@semantic-release/exec",
      {
        publishCmd: pro.ensure
          ? `cargo publish && node ${require("path").posix.relative(
              pro.dir,
              require("path").posix.resolve(
                __dirname,
                "./scripts/ensure-crate.js",
              ),
            )} \${nextRelease.version}`
          : "cargo publish",
        execCwd: pro.dir,
      },
    ]),
    [
      "@semantic-release/git",
      {
        assets: [
          "CHANGELOG.md",
          "Cargo.lock",
          ...CARGO_PROJECTS.map((pro) => pro.file),
        ],
      },
    ],
    "@semantic-release/github",
  ],
};
