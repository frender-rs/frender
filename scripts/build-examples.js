const { spawn } = require("child_process");
const fsp = require("fs/promises");
const path = require("path");

const BASE_URL = "/frender/";

function wrapHtml(title, content) {
  return `\
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8" />
    <title>${title}</title>
  </head>
  <body>
    <div style="max-width: 768px; margin: 0 auto; padding: 16px;">${content}</div>
  </body>
</html>
`;
}

function indexPageHtml(examplesListHtml) {
  return wrapHtml(
    "Frender examples",
    `\
<div style="max-width: 768px; margin: auto; text-align: center">
<img src="${BASE_URL}logo.svg" alt="frender logo" style="max-width: 100%; max-height: 240px">
<p>
<a href="https://crates.io/crates/frender" rel="nofollow"><img src="https://img.shields.io/crates/v/frender?style=for-the-badge" alt="Crates.io"></a>
<a href="https://docs.rs/frender" rel="nofollow"><img src="https://img.shields.io/docsrs/frender/latest?style=for-the-badge" alt="docs.rs"></a>
<a href="https://github.com/frender-rs/frender/blob/main/LICENSE" rel="nofollow"><img src="https://img.shields.io/github/license/frender-rs/frender?style=for-the-badge" alt="GitHub license"></a>
<a href="https://github.com/frender-rs/frender/stargazers" rel="nofollow"><img src="https://img.shields.io/github/stars/frender-rs/frender?style=for-the-badge&logo=github" alt="GitHub stars"></a>
</p>
<p>Functional Rendering: <code>React</code> in <code>Rust</code></p>
</div>
<div style="margin: 32px 0;">
<h2>Examples</h2>
${examplesListHtml}
</div>
<div>
<script type='text/javascript' src='https://storage.ko-fi.com/cdn/widget/Widget_2.js'></script><script type='text/javascript'>kofiwidget2.init('Support frender on Ko-fi', '#ff8ad8', 'N4N26J11L');kofiwidget2.draw();</script>
</div>
`,
  );
}

function examplesPageHtml(examplesListHtml) {
  return wrapHtml(
    "Frender examples",
    `\
<h1><i>frender</i> examples</h1>
${examplesListHtml}
`,
  );
}

function getExamplesListHtml(examples) {
  return `\
<ul>
${examples.map(({ name }) => {
  const nameEncoded = encodeURIComponent(name);
  return `<li><a target="_blank" href="${BASE_URL}examples/${nameEncoded}">${name}</a></li>`;
})}
</ul>
`;
}

async function listExamples(examplesFolder) {
  let dirents = await fsp.readdir(examplesFolder, { withFileTypes: true });

  return dirents
    .filter((ent) => ent.isDirectory())
    .map((ent) => {
      const folder = path.join(examplesFolder, ent.name);
      const indexFile = path.join(folder, "index.html");
      return {
        name: ent.name,
        folder,
        indexFile,
      };
    });
}

async function main() {
  const DIST_DIR = "dist";
  const DIST_EXAMPLES_DIR = "dist/examples";
  await fsp.mkdir(DIST_EXAMPLES_DIR, { recursive: true });
  const examples = await listExamples("examples");
  for (const { name, indexFile } of examples) {
    const child = spawn("trunk", [
      "build",
      "-d",
      `dist/examples/${name}`,
      "--public-url",
      `${BASE_URL}examples/${name}`,
      indexFile,
    ]);

    const { code, outputChunks } = await new Promise((resolve) => {
      const outputChunks = [];
      child.stdout.on("data", (chunk) => {
        outputChunks.push(chunk);
      });
      child.stderr.on("data", (chunk) => {
        outputChunks.push(chunk);
      });
      child.on("exit", (code) => {
        resolve({ code, outputChunks });
      });
    });

    if (code !== 0) {
      const out = Buffer.concat(outputChunks).toString();
      throw new Error(
        `Example ${name} failed to build. Exited with status ${code}\n${out}`,
      );
    } else {
      console.log(`[Example built success] ${name}`);
    }
  }

  const examplesListHtml = getExamplesListHtml(examples);

  await Promise.all([
    fsp.writeFile(
      path.join(DIST_DIR, "index.html"),
      indexPageHtml(examplesListHtml),
    ),
    fsp.writeFile(
      path.join(DIST_EXAMPLES_DIR, "index.html"),
      examplesPageHtml(examplesListHtml),
    ),
    fsp.copyFile("logo.svg", path.join(DIST_DIR, "logo.svg")),
  ]);
}

main().catch((err) => {
  process.exitCode = 1;
  console.error(err);
});
