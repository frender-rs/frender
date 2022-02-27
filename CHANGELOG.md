# [1.0.0-alpha.4](https://github.com/frender-rs/frender/compare/v1.0.0-alpha.3...v1.0.0-alpha.4) (2022-02-27)


### Bug Fixes

* big numbers should be converted to string to be used as Node ([8ba7355](https://github.com/frender-rs/frender/commit/8ba7355f9473ba267ca708475d7137961dedaa2c))
* component macro ([5f8d6f4](https://github.com/frender-rs/frender/commit/5f8d6f45ef57748513e556567788e6f9750a3092))
* component macro not works with props ([607aa84](https://github.com/frender-rs/frender/commit/607aa84463bf1f22cedfb7de5d8f2bdb9b02b4bb))
* rsx prop without value should be interpreted as `IntoPropValue::into_prop_value(true)` instead of `true` ([eae5fe8](https://github.com/frender-rs/frender/commit/eae5fe8255dfc6680633a08fe3e125161523a583))


### Features

* allow frender::Element to be cloned safely ([926c04f](https://github.com/frender-rs/frender/commit/926c04fc18535d6b25458474bd3abc002b8c9c88))
* attribute macro component ([c583ae9](https://github.com/frender-rs/frender/commit/c583ae9918c5e33709ac43b3b888690f3884fefb))
* auto debug props ([acc3c94](https://github.com/frender-rs/frender/commit/acc3c9412065349bdfc092262954498307a559bf))
* CssProperties and style macro ([7519b9b](https://github.com/frender-rs/frender/commit/7519b9bf0019a1c72ec140c86dec84f0476d4fd5))
* event handlers ([1628a64](https://github.com/frender-rs/frender/commit/1628a64fcd29fba55b504d3538927292c21a8d39))
* Fragment and StrictMode ([2a9e8e3](https://github.com/frender-rs/frender/commit/2a9e8e3c01bc61356a54b7e4ba767215f1167655))
* hooks ([a7f8088](https://github.com/frender-rs/frender/commit/a7f8088823da6fc6af4122530f6fb512714e2583))
* html intrinsic components ([2e95708](https://github.com/frender-rs/frender/commit/2e957087e5aca4d4aa32ada513a9e6292962517d))
* impl IntoPropValue<WrapFn> for Rc<Fn> and &Rc<Fn> ([85a8fa2](https://github.com/frender-rs/frender/commit/85a8fa21daa00b90902d44e6809964d5d99fa2e6))
* impl PartialEq and Eq for StateSetter ([de7132e](https://github.com/frender-rs/frender/commit/de7132e5b5b2c410ff5ab1766828503f151f2e26))
* include some traits in frender::prelude ([8f6854b](https://github.com/frender-rs/frender/commit/8f6854ba75c9f762f1c39471a3e187c9f7a726e1))
* macro def_props ([7aec7f0](https://github.com/frender-rs/frender/commit/7aec7f0e18fd39b6ca509fe3d7018e464c099c47))
* rename Node trait methods ([3ac1012](https://github.com/frender-rs/frender/commit/3ac10129089dcbabc6be9255c460f76639036dba))
* rsx macro ([28d2ce4](https://github.com/frender-rs/frender/commit/28d2ce4f1e0313a9ba5f5a20977ed928de6cc867))
* use_effect_one allow FnOnce ([b86cecb](https://github.com/frender-rs/frender/commit/b86cecb97e44d6f64a263c1cbe9b8baeabb51900))


### BREAKING CHANGES

* Node trait methods are renamed
* many api changed to allow frender::Element to be cloned safely

# [1.0.0-alpha.3](https://github.com/frender-rs/frender/compare/v1.0.0-alpha.2...v1.0.0-alpha.3) (2022-02-03)


### Bug Fixes

* use_memo fails because React.useRef doesn't accept function as initializer ([cd76127](https://github.com/frender-rs/frender/commit/cd76127ba3e96cf72ab7266a1095466df84e3271))

# [1.0.0-alpha.2](https://github.com/frender-rs/frender/compare/v1.0.0-alpha.1...v1.0.0-alpha.2) (2022-01-28)


### Features

* add package metadata ([a67602e](https://github.com/frender-rs/frender/commit/a67602e803bf1a453b728d9260a0da6584bbaefc))

# 1.0.0-alpha.1 (2022-01-28)


### Features

* **react:** Node and Nodes ([6042a12](https://github.com/frender-rs/frender/commit/6042a1206ad5b07fc51563e1ece6dcd072267985))

## [1.2.1](https://github.com/tlibjs/package-template/compare/v1.2.0...v1.2.1) (2021-07-21)

### Bug Fixes

- github release actions should check required env vars before releasing ([f70e902](https://github.com/tlibjs/package-template/commit/f70e902fa60782754b44c1f6b274644b2a05fef9)), closes [#1](https://github.com/tlibjs/package-template/issues/1)

# [1.2.0](https://github.com/tlibjs/package-template/compare/v1.1.0...v1.2.0) (2021-07-20)

### Features

- only keep one tsconfig ([204a972](https://github.com/tlibjs/package-template/commit/204a972af9e205d6313bc4ea556fe42cdb8c4bb3))
- ts-node demo ([78e6738](https://github.com/tlibjs/package-template/commit/78e6738d25b6ab533aa0100d1b68a41ab9fb8180))
- use typescript rollup configs ([c25cfdb](https://github.com/tlibjs/package-template/commit/c25cfdbe7b24ca82fd93b0e686d93f523a30355c))

# [1.1.0](https://github.com/tlibjs/package-template/compare/v1.0.0...v1.1.0) (2021-07-20)

### Bug Fixes

- auto global namespace ([33ecbf5](https://github.com/tlibjs/package-template/commit/33ecbf5165a82e3090bbf1cddc95a178569285a9))
- set rollup output.exports to auto ([a2d4399](https://github.com/tlibjs/package-template/commit/a2d43992dd4ff8f35536e92c867999795a5ce30e))
- should use @rollup/plugin-node-resolve and @rollup/plugin-commonjs for bundle builds ([4d7774a](https://github.com/tlibjs/package-template/commit/4d7774a739ce600213e9f6ac5efd834ea8bf3e80))

### Features

- multiple entry points ([caf97f1](https://github.com/tlibjs/package-template/commit/caf97f12c56e733af6c3364be3f3f95684ec354c))
- support ie 11 ([ffe02d4](https://github.com/tlibjs/package-template/commit/ffe02d4a10a6cac7701507ea3ed9bbbd990b5a20))

# 1.0.0 (2021-07-18)

### Features

- example function hello ([9a4fa99](https://github.com/tlibjs/package-template/commit/9a4fa99575359aeab5748c191ae6b3dbe2d935b0))
