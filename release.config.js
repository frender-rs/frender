const CARGO_PROJECTS = [
  //
  "frender",
];

const CARGO_FILES = CARGO_PROJECTS.map((dir) => `${dir}/Cargo.toml`);

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
            files: CARGO_FILES,
            from: 'version = ".*" # replace version',
            to: 'version = "${nextRelease.version}" # replace version',
            results: CARGO_FILES.map((file) => ({
              file,
              hasChanged: true,
              numMatches: 1,
              numReplacements: 1,
            })),
            countMatches: true,
          },
        ],
      },
    ],
    ["@semantic-release/exec", { prepareCmd: "cargo check" }],
    ...CARGO_PROJECTS.map((pro) => [
      "@semantic-release/exec",
      { publishCmd: "cargo publish", execCwd: pro },
    ]),
    [
      "@semantic-release/git",
      { assets: ["CHANGELOG.md", "Cargo.lock", ...CARGO_FILES] },
    ],
    "@semantic-release/github",
  ],
};
