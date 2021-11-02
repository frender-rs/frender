# @tlib/package-template

[![npm package @tlib/package-template](https://img.shields.io/npm/v/@tlib/package-template?style=flat-square)](http://npm.im/@tlib/package-template)
[![GitHub package.json dependency version (dev dep on branch)](https://img.shields.io/github/package-json/dependency-version/tlibjs/package-template/dev/typescript?style=flat-square)]()
[![semantic-release](https://img.shields.io/badge/%20%20%F0%9F%93%A6%F0%9F%9A%80-semantic--release-e10079.svg?style=flat-square)](https://github.com/semantic-release/semantic-release)

a template to quickly code and publish your TypeScript package

[![Use This Template](https://img.shields.io/badge/use%20this%20template-blue?logo=github&logoColor=white&style=for-the-badge)](https://github.com/tlibjs/package-template/generate)

## Features

- Yarn
- TypeScript
- Babel
  - auto polyfilled with `corejs`
- lint
  - eslint
  - lint before committing with
    [husky](https://github.com/typicode/husky)
    and
    [lint-staged](https://github.com/okonet/lint-staged)
- rollup
  - build your projects targeting `CommonJS` / `ES Modules` / `bundle`
  - auto generate `package.json` entry points
- test with [jest](https://github.com/facebook/jest)
- semantic versioning and releasing

  Auto versioning and publishing with [semantic-release](https://github.com/semantic-release/semantic-release) and [GitHub actions](.github/workflows/release.yml)

  To use this feature, please edit meta fields in `package.json`. Especially, please replace `name` will your new package name.
