const INTERVAL = 2000;
const MAX_RETRY = 10;

const { spawn, exec } = require("child_process");
const { promisify } = require("util");
const execAsync = promisify(exec);

const fsp = require("fs/promises");
const os = require("os");
const path = require("path");

async function withTmpDir(run) {
  let dir = await fsp.mkdtemp(path.join(os.tmpdir(), "cargo-ensure-crate-"));

  try {
    const ret = await run(dir);
    return ret;
  } finally {
    await fsp.rm(dir, { recursive: true, force: true });
  }
}

async function ensureCrate(pkgName, version) {
  try {
    await withTmpDir(async (dir) => {
      await execAsync("cargo init --vcs none", { cwd: dir });
      await fsp.appendFile(
        path.join(dir, "Cargo.toml"),
        `\n${pkgName} = "=${version}"\n`
      );
      await execAsync("cargo check", { cwd: dir });
    });
  } catch (err) {
    throw new Error(`${pkgName}:${version} not available: ${err.message}`);
  }
}

async function main() {
  const version = process.argv[2];

  if (!version) {
    throw new Error(`Parameter version is missing`);
  }

  const toml = await fsp.readFile("./Cargo.toml", "utf-8");
  const pkgName = toml.match(/name\s*=\s*"(.+)"/)?.[1];

  if (!pkgName) {
    throw new Error(`No valid Cargo.toml: ${process.cwd()}`);
  }

  console.log(`Checking crate ${pkgName}:${version}`);

  let ok = false;

  for (let i = 0; i < MAX_RETRY; i++) {
    if (i > 0) {
      await new Promise((resolve) => setTimeout(resolve, INTERVAL));
    }

    try {
      await ensureCrate(pkgName, version);
      ok = true;
      console.log(`[${i}] OK for ${pkgName}:${version}`);
      break;
    } catch (err) {
      console.log(`[${i}] Error for ${pkgName}:${version} ${err.message}`);
    }
  }

  if (!ok) process.exitCode = 2;
}

main().catch((err) => {
  process.exitCode = 1;
  console.error(err);
});
