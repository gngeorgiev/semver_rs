# Changelog

## 0.2.0 (22/11/2021)

### Special thanks to @mt-caret and @ansrivas for contributing 🎉

- [Small refactoring of `comparator.rs` and `version.rs`](https://github.com/gngeorgiev/semver_rs/commit/24f2438781a5ab3ed01f16e868b7c7f8b3ec2ff7) - @gngeorgiev
- [Add criterion bench](https://github.com/gngeorgiev/semver_rs/commit/6349cb021c5abb20d821a0f72b2ee9c904109b91) - @gngeorgiev
- [Fix `IntoOptionsMaybe` usage in `builder.rs`](https://github.com/gngeorgiev/semver_rs/commit/105bf0fadd1dcf730aad3c841708aa2b11be7211) - @gngeorgiev
- [Update bench dependencies and add alloc param](https://github.com/gngeorgiev/semver_rs/commit/7e4b966979ff84fd11ec9ccd2d3bd89ba00b2278) - @gngeorgiev
- [Don't install node_modules every time for bench. Update bench times](https://github.com/gngeorgiev/semver_rs/commit/72e803429aa85aa22b2710c427f3b0cf70c88f12) - @gngeorgiev
- [Optimize `comparator.rs`](https://github.com/gngeorgiev/semver_rs/commit/512ae726ef8ea4417c75c24baf2402592d34f7fe) - @gngeorgiev
- [Refactor and optimize `range.rs`](https://github.com/gngeorgiev/semver_rs/commit/e95b492c6cf649b226119c57f6082846c6f61439) - @gngeorgiev
- [Rename `match_at_index` in `util.rs`](https://github.com/gngeorgiev/semver_rs/commit/1cf78e2b8e7c95ae0451dcd6d8e2cfd2ddd53537) - @gngeorgiev
- [Improve tests assertion logging in `lib.rs`](https://github.com/gngeorgiev/semver_rs/commit/0294e2da27f3cb0ee187feaebfc70e679bacee91) - @gngeorgiev
- [fix bacon clippy](https://github.com/gngeorgiev/semver_rs/commit/baa78f8599930c4aa81d67939c8d3c32e4210768) - @gngeorgiev
- [Optimize and refactor `version.rs`](https://github.com/gngeorgiev/semver_rs/commit/508b2c06c4869811100553204bd96ea355b951aa) - @gngeorgiev
- [Remove `with_options_maybe` and add blanked `IntoOptionsMaybe` trait impl for `with_options`](https://github.com/gngeorgiev/semver_rs/commit/8919a7a3e9f40b35798b41e13e034b176756f4ae) - @gngeorgiev
- [Add back docs and crates.io badges](https://github.com/gngeorgiev/semver_rs/commit/341c30a9711db9b9bb888962b3f82d6d1e59363d) - @gngeorgiev
- [Fix CI badge](https://github.com/gngeorgiev/semver_rs/commit/a5c0cba35b77bf37a2d2f3ab7fc4403e194ba734) - @gngeorgiev
- [Add `just ci` target](https://github.com/gngeorgiev/semver_rs/commit/0bab2831772d949105a04c8b5e9638fc7c6c391a) - @gngeorgiev
- [migrate to github actions from travis](https://github.com/gngeorgiev/semver_rs/commit/a432ebc0e025bd44b8d34a711fad493a1b270f51) - @gngeorgiev
- [Use `thiserror` instead of manually implementing `Error`](https://github.com/gngeorgiev/semver_rs/commit/cedfba24d94d0b56e331b067f9f847ba5732c239) - @gngeorgiev
- [add just bench target](https://github.com/gngeorgiev/semver_rs/commit/2cd801215600e27fade811ca115a57cca67ca7e0) - @gngeorgiev
- [Remove pointless README subtext](https://github.com/gngeorgiev/semver_rs/commit/e744ba7e89b13e6776eb493bb3d18c009a8fa5ad) - @gngeorgiev
- [Improve benchmarking instructions and setup. Update benchmark results](https://github.com/gngeorgiev/semver_rs/commit/bfb8ccb5430c751d6d6503c481d6c3736d3cb69e) - @gngeorgiev
- [Add just and bacon setup and instructions](https://github.com/gngeorgiev/semver_rs/commit/8dcb7513bad3ba121df6ab4e3846ba88830b3607) - @gngeorgiev
- [Add serde examples in tests and README](https://github.com/gngeorgiev/semver_rs/commit/8a3cc76c5a1d912bdc2485632a10de2303029ab4) - @gngeorgiev
- [Add compare and serde tests](https://github.com/gngeorgiev/semver_rs/commit/7bab2badd81e7dca14a74bda439ffa8532d07260) - @gngeorgiev
- [Leave only license-file in Cargo.toml](https://github.com/gngeorgiev/semver_rs/commit/3f834d761d151746b4bac73bcef018e80d45e222) - @gngeorgiev
- [Remove clone calls for Options](https://github.com/gngeorgiev/semver_rs/commit/fee5f45d7536fc0e8bce24b05c27c2b4cff12818) - @gngeorgiev
- [Derive Copy for Options](https://github.com/gngeorgiev/semver_rs/commit/391cc5ecebd7b036ba76da87a8ef1435a3b54d8d) - @gngeorgiev
- [fix README markdown warnings](https://github.com/gngeorgiev/semver_rs/commit/37c604d46fa72a7f6f4e41c77a02af5a52c798d5) - @gngeorgiev
- [Upgrade Cargo.toml dependencies](https://github.com/gngeorgiev/semver_rs/commit/7faf5c0ab4314318c468974defc2dc9de83bd86d) - @gngeorgiev
- [Refactor more clippy warnings and further refactor](https://github.com/gngeorgiev/semver_rs/commit/59e709920c69952a221c9c3181ce08bcccd5fdaf) - @gngeorgiev
- [cargo clippy --fix first pass](https://github.com/gngeorgiev/semver_rs/commit/331418541f31fe1306a2d02a85b647c74507da3a) - @gngeorgiev
- [Implement Ord for sort operation on `Vec<Version>`](https://github.com/gngeorgiev/semver_rs/commit/dd15c55398053eddf6761f6506091f4c8c9216ea) - @ansrivas
- [Add derive Hash](https://github.com/gngeorgiev/semver_rs/commit/6785d56f84601e47b7d63e8d9d41a517fc3bc5be) - @mt-caret
- [Use i64 instead of i32 to avoid size issues](https://github.com/gngeorgiev/semver_rs/commit/3aed23b6d46ee5d44fa77ea344ad2a7c5d56a1c4) - @mt-caret
