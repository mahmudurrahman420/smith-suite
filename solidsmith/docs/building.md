# Building and running

```sh
cargo run --release -p solidsmith -- --sample          # desktop app with a sample part
cargo run --release -p solidsmith-cli -- commands      # the command registry
cargo run --release -p solidsmith-cli -- run examples/bracket.json --out bracket.step
cargo run --release -p solidsmith-cli -- snapshot examples/bracket.json --out bracket.png
cargo run --release -p solidsmith-cli -- snapshot part.sc --out preview.png --tool Pin --section z=12
cargo run --release -p solidsmith-cli -- render examples/render-studio.json --out product.png
cargo run --release -p solidsmith -- part.step          # open a STEP file as a new design
cargo run --release -p solidsmith-cli -- eval part.step # measure an imported STEP file
cargo xtask ci                                          # fmt, clippy, tests, icons, assets, layers, wasm
```

UI artwork changes follow the [icon specification](../assets/icons/ui/STYLE_GUIDE.md).
The [included review tool](../assets/icons/ui/README.md) discovers new icons and keeps
unaccepted artwork Open; run `cargo xtask icons` to check the collection.

Windows builds: `cargo xwin build --release --target x86_64-pc-windows-msvc -p solidsmith`.


In the browser: `cd apps/solidsmith-web && trunk build --release --public-url ./` (WebGPU, or
WebGL2 with `?webgl`; `?sample` opens the sample part).

Agents can control the document open in a browser tab through `window.solidsmith.execute(...)`
and `window.solidsmith.request(...)`; see the [browser command API](browser-api.md).

See [CONTRIBUTING](../CONTRIBUTING.md) for the test and review workflow.
