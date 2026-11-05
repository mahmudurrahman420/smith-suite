# Attribution

Every non-code asset in this repository (images, icons, fonts, example parts) is listed here with
its author, source and licence. `cargo xtask assets` (part of `cargo xtask ci`) fails if an asset
file is missing from this table.

**Policy (mandatory):** Solidsmith contains **no Autodesk iconography, images, artwork, fonts,
templates, materials or help text.** Every asset is original work by Solidsmith contributors or
third-party material under an open licence. Screenshots of Autodesk software are never committed.

Editable drawing code, native projects and shipped exports are collected under `assets/`.

## Files

| File | Author | Source | Licence |
|---|---|---|---|
| assets/render/catalog/capabilities.json | Solidsmith contributors | Byte-identical runtime export of docs/render-materials/capabilities.json; generated and checked by tools/render-library/generate.py | MIT OR Apache-2.0 |
| assets/render/catalog/environments.json | Solidsmith contributors | Byte-identical runtime export of docs/render-materials/environments.json; generated and checked by tools/render-library/generate.py | MIT OR Apache-2.0 |
| assets/render/catalog/lighting.json | Solidsmith contributors | Byte-identical runtime export of docs/render-materials/lighting.json; generated and checked by tools/render-library/generate.py | MIT OR Apache-2.0 |
| assets/render/catalog/materials.json | Solidsmith contributors | Byte-identical runtime export of docs/render-materials/materials.json; generated and checked by tools/render-library/generate.py | MIT OR Apache-2.0 |
| assets/render/catalog/quality-presets.json | Solidsmith contributors | Byte-identical runtime export of docs/render-materials/quality-presets.json; generated and checked by tools/render-library/generate.py | MIT OR Apache-2.0 |
| assets/render/catalog/scene-presets.json | Solidsmith contributors | Byte-identical runtime export of docs/render-materials/scene-presets.json; generated and checked by tools/render-library/generate.py | MIT OR Apache-2.0 |
| examples/chebyshev-motion-trace.json | Solidsmith contributors | Original parametric Chebyshev lambda mechanism and associative-trace demo, constructed from analytic circle intersections; classical link proportions documented at https://doi.org/10.1002/rcs.70095; no external CAD geometry or artwork | MIT OR Apache-2.0 |
| examples/material-card.json | Solidsmith contributors | original analytic box and explicitly synthetic engineering card; invented illustrative constants, no external material dataset | MIT OR Apache-2.0 |
| examples/studio/author_sweep.py | Solidsmith contributors | original editable CAD scene and authoring metadata; no external assets | MIT OR Apache-2.0 |
| examples/studio/author_scene.py | Solidsmith contributors | original editable CAD scene and authoring metadata; no external assets | MIT OR Apache-2.0 |
| examples/studio/materials.json | Solidsmith contributors | original editable CAD scene and authoring metadata; no external assets | MIT OR Apache-2.0 |
| examples/studio/coffee-exploded-sweep.json | Solidsmith contributors | original editable CAD scene and authoring metadata; no external assets | MIT OR Apache-2.0 |
| examples/studio/coffee-exploded-render.json | Solidsmith contributors | original editable CAD scene and authoring metadata; no external assets | MIT OR Apache-2.0 |
| examples/studio/coffee-exploded.json | Solidsmith contributors | original editable CAD scene and authoring metadata; no external assets | MIT OR Apache-2.0 |
| examples/studio/stage1-sweep.json | Solidsmith contributors | original tangent circular-arc CAD cyclorama; no external assets | MIT OR Apache-2.0 |
| examples/studio/stage1.json | Solidsmith contributors | original editable CAD studio scene; no imported geometry or external image assets | MIT OR Apache-2.0 |
| examples/studio/stage1-render.json | Solidsmith contributors | original studio camera and lighting parameters | MIT OR Apache-2.0 |
| examples/render-studio.json | Solidsmith contributors | original procedural box/cylinder enclosure script for the integrated Render workspace | MIT OR Apache-2.0 |
| examples/render-opaque-studio.json | Solidsmith contributors | original procedural sphere study varying opaque metal fraction and roughness independently; no external assets or engineering data | MIT OR Apache-2.0 |
| crates/gpu/src/path/denoise.wgsl | Solidsmith contributors | original bounded edge-aware a-trous spatial filter; GPU2 donor d710e31, no external shader source | MIT OR Apache-2.0 |
| crates/gpu/src/path/denoise_guides.wgsl | Solidsmith contributors | original deterministic centre-ray material and geometry guide pass; GPU2 donor d710e31, no external shader source | MIT OR Apache-2.0 |
| crates/gpu/src/path/shadows.wgsl | Solidsmith contributors | original bounded area, sun and environment visibility transport; GPU2 donor d710e31, no external shader source | MIT OR Apache-2.0 |
| crates/gpu/src/path/textures.wgsl | Solidsmith contributors | original planar colour and normal map sampling; GPU2 donor d710e31, no external shader source | MIT OR Apache-2.0 |
| assets/appearances/starter_pack.rs | Solidsmith contributors | original illustrative optical recipes, authored scalar values and future procedural resource descriptors; no measured or external material dataset | MIT OR Apache-2.0 |
| examples/material-finishes.json | Solidsmith contributors | original procedural sphere study of six illustrative opaque finishes with generic material densities; no external assets or measured optical data | MIT OR Apache-2.0 |
| examples/render-path-studio.json | Solidsmith contributors | original procedural opaque enclosure, teal insert and metal fasteners for progressive rendering; no imported models | MIT OR Apache-2.0 |
| crates/gpu/src/path/path.wgsl | Solidsmith contributors | original software BVH traversal and path integration using the existing Solidsmith surface/studio equations; no external shader source | MIT OR Apache-2.0 |
| crates/gpu/src/pbr.rs | Solidsmith contributors | original WGSL translation of Solidsmith's opaque surface and fixed studio reference, embedded as Rust source; no external shader code | MIT OR Apache-2.0 |
| assets/icons/app/macos/source/Solidsmith.icon/icon.json | Solidsmith contributors | approved Icon Composer composition and appearance settings for the original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/app/macos/source/Solidsmith.icon/Assets/02-grid.svg | Solidsmith contributors | original perspective grid with elliptical radial alpha, exported from Pathsmith | MIT OR Apache-2.0 |
| assets/icons/app/macos/source/Solidsmith.icon/Assets/02-grid-dark.svg | Solidsmith contributors | original blue perspective grid with elliptical radial alpha, exported from Pathsmith for the Dark appearance | MIT OR Apache-2.0 |
| assets/icons/app/macos/source/Solidsmith.icon/Assets/03-solid-blue-cutout.svg | Solidsmith contributors | original saturated blue CAD solid with transparent topology cutouts, edited and exported with Pathsmith | MIT OR Apache-2.0 |
| assets/icons/app/macos/exports/Assets.car | Solidsmith contributors | Apple actool compilation of assets/icons/app/macos/source/Solidsmith.icon for macOS; contains original artwork only | MIT OR Apache-2.0 |
| assets/archive/icons/macos-v1/source/icon.pathsmith | Solidsmith contributors | retained editable historical Pathsmith source, with named face and background layers; redundant exports retired | MIT OR Apache-2.0 |
| assets/icons/app/macos/exports/icon.icns | Solidsmith contributors | macOS fallback compiled by Apple's actool from the approved assets/icons/app/macos/source/Solidsmith.icon composition | MIT OR Apache-2.0 |
| assets/models/test_boat.bin | 3DBenchy.com | 3DBenchy.com | Public Domain |
| assets/models/make_test_boat.py | Solidsmith contributors | original script that packs a model into the compact file above | MIT OR Apache-2.0 |
| docs/screenshots/class-a-blend.png | Solidsmith contributors | screenshot of Solidsmith itself: a 24 mm and a 10 mm box bridged by a `surface.blend` sheet (G1 at both edges), rendered by the app's `ui.render` in the dark theme with examples/demo/class_a_shots.py | MIT OR Apache-2.0 |
| docs/screenshots/class-a-crown.png | Solidsmith contributors | screenshot of Solidsmith itself: a patch crowned with `surface.patch crown=0.3` over a skew four-line rim, rendered by the app's `ui.render` in the dark theme with examples/demo/class_a_shots.py | MIT OR Apache-2.0 |
| docs/screenshots/class-a-boundary.png | Solidsmith contributors | screenshot of Solidsmith itself: a surface through two 3D sketch splines built by `surface.boundary`, rendered by the app's `ui.render` in the dark theme with examples/demo/class_a_shots.py | MIT OR Apache-2.0 |
| docs/screenshots/sample-plate.png | Solidsmith contributors | screenshot of Solidsmith 0.22.1 itself (`solidsmith --sample`, default dark theme, 1600x1000) | MIT OR Apache-2.0 |
| assets/samples/exports/0.png | Solidsmith contributors | original Sample Plate from `crates/engine/src/sample.rs`, exported by the Solidsmith thumbnail renderer | MIT OR Apache-2.0 |
| assets/samples/exports/1.png | Solidsmith contributors | original Sheet Metal Bracket from `crates/engine/src/sample.rs`, exported by the Solidsmith thumbnail renderer | MIT OR Apache-2.0 |
| assets/samples/exports/2.png | Solidsmith contributors | original Hinge Assembly from `crates/engine/src/sample.rs`, exported by the Solidsmith thumbnail renderer | MIT OR Apache-2.0 |
| assets/samples/exports/3.png | Solidsmith contributors | original Plastic Enclosure from `crates/engine/src/sample.rs`, exported by the Solidsmith thumbnail renderer | MIT OR Apache-2.0 |
| assets/samples/exports/4.png | Solidsmith contributors | original Bolted Flange from `crates/engine/src/sample.rs`, exported by the Solidsmith thumbnail renderer | MIT OR Apache-2.0 |
| assets/samples/exports/5.png | Solidsmith contributors | original Vent Plate from `crates/engine/src/sample.rs`, exported by the Solidsmith thumbnail renderer | MIT OR Apache-2.0 |
| assets/samples/exports/scripts.json | Solidsmith contributors | SHA-256 fingerprints of the original sample scripts, generated with the shipped thumbnails | MIT OR Apache-2.0 |
| docs/screenshots/demo.gif | Solidsmith contributors | original screen recording of the actual Solidsmith 0.22.1 English desktop UI (default dark theme, software renderer) modelling an original Raspberry Pi 4 case (no logos or branding; the engraved text is set in the system's DejaVu Sans Bold), generated with examples/demo/record_ui.sh and its original animated pointer overlay (6× playback, idle pauses cut) | MIT OR Apache-2.0 |
| docs/screenshots/demo.mp4 | Solidsmith contributors | original screen recording of the actual Solidsmith 0.22.1 English desktop UI (default dark theme, software renderer) modelling an original Raspberry Pi 4 case (no logos or branding; the engraved text is set in the system's DejaVu Sans Bold), generated with examples/demo/record_ui.sh and its original animated pointer overlay (6× playback, idle pauses cut) | MIT OR Apache-2.0 |
| docs/screenshots/rover-build.gif | 0xlillith (model); Solidsmith contributors (recording) | screen recording of the actual Solidsmith 0.22.1 desktop UI (default dark theme, software renderer) replaying the timeline of the SELENE rover (`Rover-recorded.solidsmith` from https://github.com/0xlillith/solidsmith-rover, an original model by 0xlillith built through Solidsmith MCP); the model's lettering follows Ubuntu Light outlines (Canonical Ltd., Ubuntu Font Licence 1.0, as noted in that repository); the emblem is the Solidsmith project icon (15× playback, idle pauses cut, constant-rate camera orbit) | MIT OR Apache-2.0 |
| docs/screenshots/rover-build.mp4 | 0xlillith (model); Solidsmith contributors (recording) | screen recording of the actual Solidsmith 0.22.1 desktop UI (default dark theme, software renderer) replaying the timeline of the SELENE rover (`Rover-recorded.solidsmith` from https://github.com/0xlillith/solidsmith-rover, an original model by 0xlillith built through Solidsmith MCP); the model's lettering follows Ubuntu Light outlines (Canonical Ltd., Ubuntu Font Licence 1.0, as noted in that repository); the emblem is the Solidsmith project icon | MIT OR Apache-2.0 |
| docs/design/ux-redesign/model-floating.png | Solidsmith contributors | original UX mockup (Model tab, floating layout) rendered from an HTML mockup that uses the original Technical Accent icon set; no third-party artwork, and not a screenshot of any other CAD product | MIT OR Apache-2.0 |
| docs/design/ux-redesign/model-compact.png | Solidsmith contributors | original UX mockup (Model tab, compact docked layout) rendered from an HTML mockup that uses the original Technical Accent icon set; no third-party artwork, and not a screenshot of any other CAD product | MIT OR Apache-2.0 |
| docs/design/ux-redesign/sketch.png | Solidsmith contributors | original UX mockup (Sketch tab) rendered from an HTML mockup that uses the original Technical Accent icon set; no third-party artwork, and not a screenshot of any other CAD product | MIT OR Apache-2.0 |
| docs/design/ux-redesign/sheet-metal.png | Solidsmith contributors | original UX mockup (Sheet Metal contextual tab) rendered from an HTML mockup that uses the original Technical Accent icon set; no third-party artwork, and not a screenshot of any other CAD product | MIT OR Apache-2.0 |
| docs/design/ux-redesign/start-page.png | Solidsmith contributors | original UX mockup (start page (File view)) rendered from an HTML mockup that uses the original Technical Accent icon set; no third-party artwork, and not a screenshot of any other CAD product | MIT OR Apache-2.0 |
| docs/design/ux-redesign/workspaces.png | Solidsmith contributors | original UX mockup (workspace switcher) rendered from an HTML mockup that uses the original Technical Accent icon set; no third-party artwork, and not a screenshot of any other CAD product | MIT OR Apache-2.0 |
| docs/design/ux-redesign/command-map.png | Solidsmith contributors | original UX mockup (ribbon command map) rendered from an HTML mockup that uses the original Technical Accent icon set; no third-party artwork, and not a screenshot of any other CAD product | MIT OR Apache-2.0 |
| assets/branding/web/source/og.png | Solidsmith contributors | original 1280 × 640 social preview card (the Solidsmith mark, name and tagline), served by the docs site for link previews | MIT OR Apache-2.0 |
| docs/book/src/generated/sample-plate.png | Solidsmith contributors | copy of docs/screenshots/sample-plate.png made by `cargo xtask book` (gitignored) | MIT OR Apache-2.0 |
| docs/book/src/generated/bracket.png | Solidsmith contributors | rendered by `cargo xtask book` (`solidsmith-cli snapshot examples/bracket.json`; gitignored) | MIT OR Apache-2.0 |
| docs/book/src/generated/enclosure.png | Solidsmith contributors | rendered by `cargo xtask book` (`solidsmith-cli snapshot docs/book/scripts/enclosure.json`; gitignored) | MIT OR Apache-2.0 |
| docs/book/src/generated/revolve.png | Solidsmith contributors | rendered by `cargo xtask book` (`solidsmith-cli snapshot docs/book/scripts/revolve.json`; gitignored) | MIT OR Apache-2.0 |
| docs/book/src/generated/sheet-metal.png | Solidsmith contributors | rendered by `cargo xtask book` (`solidsmith-cli snapshot docs/book/scripts/sheet-metal.json`; gitignored) | MIT OR Apache-2.0 |
| assets/fonts/ibm-plex-mono/IBMPlexMono-Regular.ttf | IBM Corp. (IBM Plex Mono) | https://github.com/IBM/plex (via google/fonts), unmodified; licence text in assets/fonts/ibm-plex-mono/OFL.txt; the interface monospace font | SIL OFL 1.1 |
| assets/fonts/ibm-plex-sans/IBMPlexSans-Variable.ttf | IBM Corp. (IBM Plex Sans, variable wdth and wght) | https://github.com/IBM/plex (via google/fonts), unmodified; licence text in assets/fonts/ibm-plex-sans/OFL.txt; the default interface font | SIL OFL 1.1 |
| docs/book/src/generated/tool-preview.png | Solidsmith contributors | rendered by `cargo xtask book` (`solidsmith-cli snapshot docs/book/scripts/tool-preview.json`; gitignored) | MIT OR Apache-2.0 |
| assets/fonts/liberation-sans/LiberationSans-Regular.ttf | Red Hat, Inc. (Liberation Fonts 2.x; digitized data Google) | https://github.com/liberationfonts (Debian fonts-liberation), unmodified; licence text in assets/fonts/liberation-sans/OFL.txt; used for sketch Text outlines | SIL OFL 1.1 |
| assets/fonts/biz-udp-gothic/BIZUDPGothic-Regular.ttf | The BIZ UDGothic Project Authors | https://github.com/googlefonts/morisawa-biz-ud-gothic (Google Fonts), unmodified; licence text in assets/fonts/biz-udp-gothic/OFL.txt; used for web UI Japanese font | SIL OFL 1.1 |
| assets/fonts/craft-ui-sc/CraftUI-SC.otf | Adobe / Google Inc. (Noto Sans SC); Solidsmith subset | https://github.com/notofonts/noto-cjk Sans/SubsetOTF/SC Regular, glyph-subset for assets/locales/zh.json and renamed Craft UI SC (OFL reserved-name); licence text in assets/fonts/craft-ui-sc/OFL.txt; web UI Simplified Chinese | SIL OFL 1.1 |
| assets/fonts/craft-ui-tc/CraftUI-TC.otf | Adobe / Google Inc. (Noto Sans TC); Solidsmith subset | https://github.com/notofonts/noto-cjk Sans/SubsetOTF/TC Regular, glyph-subset for assets/locales/zh-TW.json and renamed Craft UI TC (OFL reserved-name); licence text in assets/fonts/craft-ui-tc/OFL.txt; web UI Traditional Chinese | SIL OFL 1.1 |
| tests/step-conformance/cone_point.step | step-conformance contributors | generated from formulas by https://github.com/dan335/step-conformance (corpus/generated, 96f30bb); import regression fixture | CC0-1.0 |
| tests/step-conformance/cone_point_vertex.step | step-conformance contributors | generated from formulas by https://github.com/dan335/step-conformance (corpus/generated, 96f30bb); import regression fixture | CC0-1.0 |
| tests/step-conformance/cylinder.step | step-conformance contributors | generated from formulas by https://github.com/dan335/step-conformance (corpus/generated, 96f30bb); import regression fixture | CC0-1.0 |
| tests/step-conformance/cylinder_inch.step | step-conformance contributors | generated from formulas by https://github.com/dan335/step-conformance (corpus/generated, 96f30bb); import regression fixture | CC0-1.0 |
| tests/step-conformance/cylinder_nurbs.step | step-conformance contributors | generated from formulas by https://github.com/dan335/step-conformance (corpus/generated, 96f30bb); import regression fixture | CC0-1.0 |
| tests/step-conformance/cylinder_placed.step | step-conformance contributors | generated from formulas by https://github.com/dan335/step-conformance (corpus/generated, 96f30bb); import regression fixture | CC0-1.0 |
| tests/step-conformance/cylinder_seam.step | step-conformance contributors | generated from formulas by https://github.com/dan335/step-conformance (corpus/generated, 96f30bb); import regression fixture | CC0-1.0 |
| tests/step-conformance/cylinder_twice.step | step-conformance contributors | generated from formulas by https://github.com/dan335/step-conformance (corpus/generated, 96f30bb); import regression fixture | CC0-1.0 |
| tests/step-conformance/dome.step | step-conformance contributors | generated from formulas by https://github.com/dan335/step-conformance (corpus/generated, 96f30bb); import regression fixture | CC0-1.0 |
| tests/step-conformance/dome_nurbs.step | step-conformance contributors | generated from formulas by https://github.com/dan335/step-conformance (corpus/generated, 96f30bb); import regression fixture | CC0-1.0 |
| tests/step-conformance/far_cylinder.step | step-conformance contributors | generated from formulas by https://github.com/dan335/step-conformance (corpus/generated, 96f30bb); import regression fixture | CC0-1.0 |
| tests/step-conformance/grooved_shaft.step | step-conformance contributors | generated from formulas by https://github.com/dan335/step-conformance (corpus/generated, 96f30bb); import regression fixture | CC0-1.0 |
| tests/step-conformance/plate_100_holes.step | step-conformance contributors | generated from formulas by https://github.com/dan335/step-conformance (corpus/generated, 96f30bb); import regression fixture | CC0-1.0 |
| tests/step-conformance/sheet_disc.step | step-conformance contributors | generated from formulas by https://github.com/dan335/step-conformance (corpus/generated, 96f30bb); import regression fixture | CC0-1.0 |
| tests/step-conformance/tiny_pin.step | step-conformance contributors | generated from formulas by https://github.com/dan335/step-conformance (corpus/generated, 96f30bb); import regression fixture | CC0-1.0 |
| tests/step-conformance/torus_ring_nurbs.step | step-conformance contributors | generated from formulas by https://github.com/dan335/step-conformance (corpus/generated, 96f30bb); import regression fixture | CC0-1.0 |
| tests/step-conformance/washer.step | step-conformance contributors | generated from formulas by https://github.com/dan335/step-conformance (corpus/generated, 96f30bb); import regression fixture | CC0-1.0 |
| tests/kernel-cases/import/1721-closed-bspline-tube-face/input.step | step-conformance contributors | copy of tests/step-conformance/cylinder_nurbs.step (generated from formulas by https://github.com/dan335/step-conformance, corpus/generated, 96f30bb); kernel regression case for tangency-labs/solidcraft#1721 | CC0-1.0 |
| tests/kernel-cases/import/2043-outer-bound-opposing-surface-sense/input.step | Solidsmith contributors | synthetic STEP file written by hand from ISO 10303-21 for the kernel regression case of tangency-labs/solidcraft#2043 | MIT OR Apache-2.0 |
| tests/kernel-cases/import/258-cone-point-vertex/input.step | step-conformance contributors | copy of tests/step-conformance/cone_point_vertex.step (generated from formulas by https://github.com/dan335/step-conformance, corpus/generated, 96f30bb); kernel regression case for tangency-labs/solidcraft#258 | CC0-1.0 |
| tests/kernel-cases/import/258-cone-to-point/input.step | step-conformance contributors | copy of tests/step-conformance/cone_point.step (generated from formulas by https://github.com/dan335/step-conformance, corpus/generated, 96f30bb); kernel regression case for tangency-labs/solidcraft#258 | CC0-1.0 |
| tests/kernel-cases/import/258-tiny-pin/input.step | step-conformance contributors | copy of tests/step-conformance/tiny_pin.step (generated from formulas by https://github.com/dan335/step-conformance, corpus/generated, 96f30bb); kernel regression case for tangency-labs/solidcraft#258 | CC0-1.0 |
| tests/kernel-cases/import/3098-bspline-edges-over-tiny-knot-span/input.step | Solidsmith contributors | synthetic STEP file written by hand from ISO 10303-21 for the kernel regression case of tangency-labs/solidcraft#3098 | MIT OR Apache-2.0 |
| tests/kernel-cases/import/3208-extruded-ellipse-face-over-parameter-seam/input.step | Solidsmith contributors | synthetic STEP file written by hand from ISO 10303-21 for the kernel regression case of tangency-labs/solidcraft#3208 | MIT OR Apache-2.0 |
| tests/kernel-cases/import/3352-vertex-off-its-edge-curve/input.step | Solidsmith contributors | synthetic STEP file written by hand from ISO 10303-21 for the kernel regression case of tangency-labs/solidcraft#3352 | MIT OR Apache-2.0 |
| tests/kernel-cases/import/3730-void-oriented-shell-false/input.step | Solidsmith contributors | synthetic STEP file written by hand from ISO 10303-21 for the kernel regression case of tangency-labs/solidcraft#3730 | MIT OR Apache-2.0 |
| tests/kernel-cases/import/3880-cylinder-seam-edge/input.step | step-conformance contributors | copy of tests/step-conformance/cylinder_seam.step (generated from formulas by https://github.com/dan335/step-conformance, corpus/generated, 96f30bb); kernel regression case for tangency-labs/solidcraft#3880 | CC0-1.0 |
| tests/kernel-cases/import/3880-nurbs-hemisphere-dome/input.step | step-conformance contributors | copy of tests/step-conformance/dome_nurbs.step (generated from formulas by https://github.com/dan335/step-conformance, corpus/generated, 96f30bb); kernel regression case for tangency-labs/solidcraft#3880 | CC0-1.0 |
| tests/kernel-cases/import/3963-closed-spline-edge-vertex-past-start/input.step | Solidsmith contributors | synthetic STEP file written by hand from ISO 10303-21 for the kernel regression case of tangency-labs/solidcraft#3963 | MIT OR Apache-2.0 |
| tests/kernel-cases/import/4005-cylinder-written-as-near-zero-cone/input.step | Solidsmith contributors | synthetic STEP file written by hand from ISO 10303-21 for the kernel regression case of tangency-labs/solidcraft#4005 | MIT OR Apache-2.0 |
| tests/kernel-cases/import/4006-plane-written-as-cone-of-nearly-right-angle/input.step | Solidsmith contributors | synthetic STEP file written by hand from ISO 10303-21 for the kernel regression case of tangency-labs/solidcraft#4006 | MIT OR Apache-2.0 |
| tests/ssi-corpus/cases/*.json.gz, tests/ssi-corpus/LICENSE-MIT, tests/ssi-corpus/LICENSE-APACHE | ssi-corpus contributors (jyviko) | 119 surface/surface intersection cases with exact ground truth from https://github.com/jyviko/ssi-corpus (cases/data, b04c8a6), gzip-compressed, unchanged; SSI scorer fixtures (tests/ssi-corpus/README.md) | MIT OR Apache-2.0 |
| tests/ssi-corpus/cases/y23_*.json.gz | Yang, Jia and Yan (paper authors: published surfaces, figures and topology); ssi-corpus contributors (reconstructions and truth) | 25 cases built from Yang, Jia & Yan 2023, "Topology Guaranteed B-Spline Surface/Surface Intersection", ACM Transactions on Graphics 42(6), Article 211; each cites the paper in its `source` field | CC-BY 4.0 (paper); MIT OR Apache-2.0 (case files) |
| tests/step-regressions/rod_doubled_edge.step | Solidsmith contributors | written by Solidsmith from the commands of #4598 (a bored rod moved 100 mm and exported); import regression fixture | MIT OR Apache-2.0 |
| tests/oracle/17-loft/reference.stl | Solidsmith contributors | triangulation of our own recipe (tests/oracle/17-loft/recipe.json) built with FreeCAD as an external tool by tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/23-bracket-multifeature/reference.stl | Solidsmith contributors | triangulation of our own recipe (tests/oracle/23-bracket-multifeature/recipe.json) built with FreeCAD as an external tool by tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/28-blind-pocket-fillets/reference.stl | Solidsmith contributors | triangulation of our own recipe (tests/oracle/28-blind-pocket-fillets/recipe.json) built with FreeCAD as an external tool by tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/30-fillet-after-shell/reference.stl | Solidsmith contributors | triangulation of our own recipe (tests/oracle/30-fillet-after-shell/recipe.json) built with FreeCAD as an external tool by tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/31-fillet-l-all-edges/reference.stl | Solidsmith contributors | triangulation of our own recipe (tests/oracle/31-fillet-l-all-edges/recipe.json) built with FreeCAD as an external tool by tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/34-chamfer-then-fillet/reference.stl | Solidsmith contributors | triangulation of our own recipe (tests/oracle/34-chamfer-then-fillet/recipe.json) built with FreeCAD as an external tool by tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/35-fillet-curved-curved/reference.stl | Solidsmith contributors | triangulation of our own recipe (tests/oracle/35-fillet-curved-curved/recipe.json) built with FreeCAD as an external tool by tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/36-corner-blend-two-radii/reference.stl | Solidsmith contributors | triangulation of our own recipe (tests/oracle/36-corner-blend-two-radii/recipe.json) built with FreeCAD as an external tool by tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/37-pyramid-apex-blend/reference.stl | Solidsmith contributors | triangulation of our own recipe (tests/oracle/37-pyramid-apex-blend/recipe.json) built with FreeCAD as an external tool by tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/39-tee-pipe/reference.stl | Solidsmith contributors | triangulation of our own recipe (tests/oracle/39-tee-pipe/recipe.json) built with FreeCAD as an external tool by tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/41-tube-radial-holes/reference.stl | Solidsmith contributors | triangulation of our own recipe (tests/oracle/41-tube-radial-holes/recipe.json) built with FreeCAD as an external tool by tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/42-overlapping-patterns/reference.stl | Solidsmith contributors | triangulation of our own recipe (tests/oracle/42-overlapping-patterns/recipe.json) built with FreeCAD as an external tool by tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/47-draft-curved-pocket/reference.stl | Solidsmith contributors | triangulation of our own recipe (tests/oracle/47-draft-curved-pocket/recipe.json) built with FreeCAD as an external tool by tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/51-loft-rect-circle-offset/reference.stl | Solidsmith contributors | triangulation of our own recipe (tests/oracle/51-loft-rect-circle-offset/recipe.json) built with FreeCAD as an external tool by tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/56-holes-on-curved-face/reference.stl | Solidsmith contributors | triangulation of our own recipe (tests/oracle/56-holes-on-curved-face/recipe.json) built with FreeCAD as an external tool by tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/58-phone-stand/reference.stl | Solidsmith contributors | triangulation of our own recipe (tests/oracle/58-phone-stand/recipe.json) built with FreeCAD as an external tool by tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/59-enclosure-base/reference.stl | Solidsmith contributors | triangulation of our own recipe (tests/oracle/59-enclosure-base/recipe.json) built with FreeCAD as an external tool by tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/61-shaft-keyway/reference.stl | Solidsmith contributors | triangulation of our own recipe (tests/oracle/61-shaft-keyway/recipe.json) built with FreeCAD as an external tool by tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/63-hinge-knuckle/reference.stl | Solidsmith contributors | triangulation of our own recipe (tests/oracle/63-hinge-knuckle/recipe.json) built with FreeCAD as an external tool by tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/65-sketch-gear-tooth/reference.stl | Solidsmith contributors | triangulation of our own recipe (tests/oracle/65-sketch-gear-tooth/recipe.json) built with FreeCAD as an external tool by tools/freecad-oracle | MIT OR Apache-2.0 |
| assets/appearances/library.rs | Solidsmith contributors | Original Solidsmith resource data or drawing code, collected without redesign | MIT OR Apache-2.0 |
| assets/appearances/opaque_finishes.rs | Solidsmith contributors | original illustrative opaque finish values; no external dataset, textures or measured engineering data | MIT OR Apache-2.0 |
| assets/fonts/biz-udp-gothic/OFL.txt | The BIZ UDGothic Project Authors | Unmodified accompanying font licence | SIL OFL 1.1 |
| assets/fonts/craft-ui-sc/OFL.txt | Adobe / Google Inc. (Noto CJK); Solidsmith header | Accompanying OFL for CraftUI-SC.otf | SIL OFL 1.1 |
| assets/fonts/craft-ui-tc/OFL.txt | Adobe / Google Inc. (Noto CJK); Solidsmith header | Accompanying OFL for CraftUI-TC.otf | SIL OFL 1.1 |
| assets/fonts/ibm-plex-mono/OFL.txt | IBM Corp. | Unmodified accompanying font licence | SIL OFL 1.1 |
| assets/fonts/ibm-plex-sans/OFL.txt | IBM Corp. | Unmodified accompanying font licence | SIL OFL 1.1 |
| assets/fonts/liberation-sans/OFL.txt | Red Hat, Inc.; Liberation Fonts contributors | Unmodified accompanying font licence | SIL OFL 1.1 |
| assets/icons/app/linux/exports/hicolor/128x128/apps/io.tangencylabs.solidsmith.png | Solidsmith contributors | Generated from assets/icons/app/shared/source/app_icon.rs with the original CLI/ICO encoders | MIT OR Apache-2.0 |
| assets/icons/app/linux/exports/hicolor/16x16/apps/io.tangencylabs.solidsmith.png | Solidsmith contributors | Generated from assets/icons/app/shared/source/app_icon.rs with the original CLI/ICO encoders | MIT OR Apache-2.0 |
| assets/icons/app/linux/exports/hicolor/24x24/apps/io.tangencylabs.solidsmith.png | Solidsmith contributors | Generated from assets/icons/app/shared/source/app_icon.rs with the original CLI/ICO encoders | MIT OR Apache-2.0 |
| assets/icons/app/linux/exports/hicolor/256x256/apps/io.tangencylabs.solidsmith.png | Solidsmith contributors | Generated from assets/icons/app/shared/source/app_icon.rs with the original CLI/ICO encoders | MIT OR Apache-2.0 |
| assets/icons/app/linux/exports/hicolor/32x32/apps/io.tangencylabs.solidsmith.png | Solidsmith contributors | Generated from assets/icons/app/shared/source/app_icon.rs with the original CLI/ICO encoders | MIT OR Apache-2.0 |
| assets/icons/app/linux/exports/hicolor/48x48/apps/io.tangencylabs.solidsmith.png | Solidsmith contributors | Generated from assets/icons/app/shared/source/app_icon.rs with the original CLI/ICO encoders | MIT OR Apache-2.0 |
| assets/icons/app/linux/exports/hicolor/512x512/apps/io.tangencylabs.solidsmith.png | Solidsmith contributors | Generated from assets/icons/app/shared/source/app_icon.rs with the original CLI/ICO encoders | MIT OR Apache-2.0 |
| assets/icons/app/linux/exports/hicolor/64x64/apps/io.tangencylabs.solidsmith.png | Solidsmith contributors | Generated from assets/icons/app/shared/source/app_icon.rs with the original CLI/ICO encoders | MIT OR Apache-2.0 |
| assets/icons/app/shared/source/app_icon.rs | Solidsmith contributors | Original Solidsmith drawing code: the borderless mark, drawn from the faces and gradients of assets/icons/app/macos/source/Solidsmith.icon/Assets/03-solid-blue-cutout.svg | MIT OR Apache-2.0 |
| assets/icons/app/shared/source/windows_ico.rs | Solidsmith contributors | Original Solidsmith resource data or drawing code, collected without redesign | MIT OR Apache-2.0 |
| assets/icons/app/windows/exports/solidsmith-executable.ico | Solidsmith contributors | Generated from assets/icons/app/shared/source/app_icon.rs with the original CLI/ICO encoders | MIT OR Apache-2.0 |
| assets/icons/app/windows/exports/solidsmith.ico | Solidsmith contributors | Generated from assets/icons/app/shared/source/app_icon.rs with the original CLI/ICO encoders | MIT OR Apache-2.0 |
| assets/icons/ui/source/glyphs.rs | Solidsmith / Pathsmith contributors | Original glyphs; caption geometry adapted from Pathsmith titlebar.rs (https://github.com/mahmudurrahman420/pathsmith) | MIT OR Apache-2.0 |
| assets/icons/ui/source/markers.rs | Solidsmith contributors | Original Solidsmith resource data or drawing code, collected without redesign | MIT OR Apache-2.0 |
| assets/icons/ui/source/navigation.rs | Solidsmith contributors | Original Solidsmith resource data or drawing code, collected without redesign | MIT OR Apache-2.0 |
| assets/icons/ui/source/pointer.rs | Solidsmith contributors | Original Solidsmith resource data or drawing code, collected without redesign | MIT OR Apache-2.0 |
| assets/locales/cs.json | potuzai | Czech translation of the English catalog, contributed upstream | MIT OR Apache-2.0 |
| assets/locales/da.json | Solidsmith contributors | Original Danish translation of Solidsmith's English catalog; no commercial CAD text used | MIT OR Apache-2.0 |
| assets/locales/de.json | Solidsmith contributors | Original Solidsmith resource data or drawing code, collected without redesign | MIT OR Apache-2.0 |
| assets/locales/en-GB.json | Solidsmith contributors | English (UK): the English catalog's messages whose spelling differs, in UK spelling | MIT OR Apache-2.0 |
| assets/locales/en.json | Solidsmith contributors | Original Solidsmith resource data or drawing code, collected without redesign | MIT OR Apache-2.0 |
| assets/locales/es.json | jawler | Spanish (Spain) translation of the English catalog, contributed upstream | MIT OR Apache-2.0 |
| assets/locales/fr.json | rlartot | French translation of the English catalog, contributed upstream | MIT OR Apache-2.0 |
| assets/locales/gl.json | Pablo López (pablopeza) | Galician translation of the English catalog, contributed upstream | MIT OR Apache-2.0 |
| assets/locales/hu.json | Solidsmith contributors | Original Hungarian translation of Solidsmith's English catalog; no commercial CAD text used | MIT OR Apache-2.0 |
| assets/locales/id.json | Solidsmith contributors | Original Indonesian translation of Solidsmith's English catalog; no commercial CAD text used | MIT OR Apache-2.0 |
| assets/locales/it.json | Solidsmith contributors | Original Italian translation of Solidsmith's English catalog; no commercial CAD text used | MIT OR Apache-2.0 |
| assets/locales/ja.json | Solidsmith contributors | Original Solidsmith resource data or drawing code, collected without redesign | MIT OR Apache-2.0 |
| assets/locales/ko.json | Solidsmith contributors | Original Solidsmith resource data or drawing code, collected without redesign | MIT OR Apache-2.0 |
| assets/locales/ms.json | A.Hakim (a-hakim) | Malay translation of the English message catalog | MIT OR Apache-2.0 |
| assets/locales/nb.json | Solidsmith contributors | Original Norwegian Bokmål translation of Solidsmith's English catalog; no commercial CAD text used | MIT OR Apache-2.0 |
| assets/locales/nl.json | Solidsmith contributors | Original Dutch translation of Solidsmith's English catalog; no commercial CAD text used | MIT OR Apache-2.0 |
| assets/locales/pl.json | cyryllo | Polish translation of the English catalog, contributed upstream | MIT OR Apache-2.0 |
| assets/locales/ru.json | Alex Pro (del000) | Russian translation of the English catalog, contributed upstream | MIT OR Apache-2.0 |
| assets/locales/sv.json | Solidsmith contributors | Original Swedish translation of Solidsmith's English catalog; no commercial CAD text used | MIT OR Apache-2.0 |
| assets/locales/pt-BR.json | DuBearte | Brazilian Portuguese translation of the English catalog, contributed upstream | MIT OR Apache-2.0 |
| assets/locales/ro.json | Solidsmith contributors | Original Romanian translation of Solidsmith's English catalog; no commercial CAD text used | MIT OR Apache-2.0 |
| assets/locales/tr.json | macrulz54 | Turkish translation of the English catalog, contributed upstream | MIT OR Apache-2.0 |
| assets/locales/uk.json | Igor Bronovskyi (BrunIF) | Ukrainian translation of the English catalog, contributed upstream | MIT OR Apache-2.0 |
| assets/locales/vi.json | vandang890615 | Vietnamese translation of the English catalog, contributed upstream | MIT OR Apache-2.0 |
| assets/locales/zh.json | Solidsmith contributors | Original Solidsmith resource data or drawing code, collected without redesign | MIT OR Apache-2.0 |
| assets/locales/zh-TW.json | Solidsmith contributors | Original Traditional Chinese (Taiwan) translation of Solidsmith's English catalog; no commercial CAD text used | MIT OR Apache-2.0 |
| assets/materials/library.rs | Solidsmith contributors | Original Solidsmith resource data or drawing code, collected without redesign | MIT OR Apache-2.0 |
| assets/platforms/linux/io.tangencylabs.solidsmith.desktop | Pixelsmith / Solidsmith contributors | Pixelsmith packaging templates (https://github.com/mahmudurrahman420/pixelsmith), adapted for Solidsmith | Apache-2.0 |
| assets/platforms/linux/io.tangencylabs.solidsmith.metainfo.xml.in | Pixelsmith / Solidsmith contributors | Pixelsmith packaging templates (https://github.com/mahmudurrahman420/pixelsmith), adapted for Solidsmith | Apache-2.0 |
| assets/platforms/linux/io.tangencylabs.solidsmith.mime.xml | Pixelsmith / Solidsmith contributors | Pixelsmith packaging templates (https://github.com/mahmudurrahman420/pixelsmith), adapted for Solidsmith | Apache-2.0 |
| assets/platforms/macos/dmg/background.tiff | Solidsmith contributors | original disk image window background (blueprint sheet: grid, double border, circles, centre lines, leader arrow, title block) drawn by assets/platforms/macos/dmg/make_background.py in IBM Plex Sans/Mono; 1x and 2x representations | MIT OR Apache-2.0 |
| assets/platforms/macos/dmg/make_background.py | Solidsmith contributors | authoring script that draws the disk image background (Pillow); not part of any build | MIT OR Apache-2.0 |
| assets/platforms/macos/dmg/layout.json | Solidsmith contributors | disk image window size, icon size and icon positions shared by the background script and settings.py | MIT OR Apache-2.0 |
| assets/platforms/macos/dmg/settings.py | Solidsmith contributors | dmgbuild settings (MIT, https://github.com/dmgbuild/dmgbuild) for the Solidsmith disk image; original settings, no dmgbuild code copied | MIT OR Apache-2.0 |
| assets/platforms/macos/Info.plist.in | Pixelsmith / Solidsmith contributors | Pixelsmith packaging templates (https://github.com/mahmudurrahman420/pixelsmith), adapted for Solidsmith | Apache-2.0 |
| assets/platforms/macos/README.txt | Pixelsmith / Solidsmith contributors | Pixelsmith packaging templates (https://github.com/mahmudurrahman420/pixelsmith), adapted for Solidsmith | Apache-2.0 |
| assets/platforms/windows/solidsmith.en-us.wxl | Pixelsmith / Solidsmith contributors | Pixelsmith packaging templates (https://github.com/mahmudurrahman420/pixelsmith), adapted for Solidsmith | Apache-2.0 |
| assets/platforms/windows/solidsmith.wxs | Pixelsmith / Solidsmith contributors | Pixelsmith packaging templates (https://github.com/mahmudurrahman420/pixelsmith), adapted for Solidsmith | Apache-2.0 |
| assets/shaders/viewport.wgsl | Solidsmith contributors | Original Solidsmith resource data or drawing code, collected without redesign | MIT OR Apache-2.0 |
| assets/styles/ui/palettes.rs | Solidsmith contributors | Original Solidsmith resource data or drawing code, collected without redesign | MIT OR Apache-2.0 |
| assets/styles/ui/themes/gruvbox-dark.json | Solidsmith contributors | original Solidsmith theme file mapping Solidsmith's UI colours to the Gruvbox palette's colour values (Gruvbox by Pavel Pertsev, https://github.com/morhetz/gruvbox, MIT) | MIT OR Apache-2.0 |
| assets/styles/ui/themes/gruvbox-light.json | Solidsmith contributors | original Solidsmith theme file mapping Solidsmith's UI colours to the Gruvbox palette's colour values (Gruvbox by Pavel Pertsev, https://github.com/morhetz/gruvbox, MIT) | MIT OR Apache-2.0 |
| assets/styles/ui/themes/solidsmith-95.json | Solidsmith contributors | original Solidsmith theme file | MIT OR Apache-2.0 |
| assets/styles/ui/themes/platinum-9.json | Solidsmith contributors | original Solidsmith theme file | MIT OR Apache-2.0 |
| assets/styles/ui/themes/cupertino.json | Solidsmith contributors | original Solidsmith theme file | MIT OR Apache-2.0 |
| assets/styles/ui/themes/green-phosphor.json | Solidsmith contributors | original Solidsmith theme file | MIT OR Apache-2.0 |
| assets/styles/ui/themes/amber-phosphor.json | Solidsmith contributors | original Solidsmith theme file | MIT OR Apache-2.0 |
| assets/styles/ui/themes/blueprint.json | Solidsmith contributors | original Solidsmith theme file | MIT OR Apache-2.0 |
| assets/styles/ui/themes/one-dark.json | Solidsmith contributors | original Solidsmith theme file with colours adapted from the One Dark palette (Atom One Dark by GitHub Atom, https://github.com/atom/atom, MIT) | MIT OR Apache-2.0 |
| assets/styles/ui/themes/tokyo-night.json | Solidsmith contributors | original Solidsmith theme file adapting the colours of a published palette (Tokyo Night by enkia, https://github.com/enkia/tokyo-night-vscode-theme, MIT) | MIT OR Apache-2.0 |
| assets/styles/ui/themes/solarized-light.json | Solidsmith contributors | original Solidsmith theme file adapting the colours of a published palette (Solarized by Ethan Schoonover, https://github.com/altercation/solarized, MIT) | MIT OR Apache-2.0 |
| assets/styles/ui/themes/nord.json | Solidsmith contributors | original Solidsmith theme file adapting the colours of a published palette (Nord by Sven Greb and the Nord contributors, https://github.com/nordtheme/nord, MIT) | MIT OR Apache-2.0 |
| assets/styles/ui/themes/high-contrast.json | Solidsmith contributors | original Solidsmith theme file | MIT OR Apache-2.0 |
| assets/styles/ui/themes/graphite.json | Solidsmith contributors | original Solidsmith theme file | MIT OR Apache-2.0 |
| assets/styles/ui/themes/nosferatu.json | Solidsmith contributors | original Solidsmith theme file adapting the colours of a published palette (Dracula by Zeno Rocha and the Dracula Theme contributors, https://github.com/dracula/dracula-theme, MIT) | MIT OR Apache-2.0 |
| assets/styles/ui/themes/catppuccin-mocha.json | Solidsmith contributors | original Solidsmith theme file adapting the colours of a published palette (Catppuccin by the Catppuccin organisation, https://github.com/catppuccin/catppuccin, MIT) | MIT OR Apache-2.0 |
| assets/styles/ui/themes/okibi.json | Solidsmith contributors (Kai Evans) | original Solidsmith theme file mapping Solidsmith's UI colours to the Okibi palette (dark) of the Okibi design system by Kai Evans (https://kaievns.github.io/okibi-akiri-design-system-spec/) | MIT OR Apache-2.0 |
| assets/styles/ui/themes/akari.json | Solidsmith contributors (Kai Evans) | original Solidsmith theme file mapping Solidsmith's UI colours to the Akari palette (light) of the Okibi design system by Kai Evans (https://kaievns.github.io/okibi-akiri-design-system-spec/) | MIT OR Apache-2.0 |
| assets/styles/web/loading-logo.svg | Solidsmith contributors | Animation of the Solidsmith mark, built from the paths and gradients of assets/icons/app/macos/source/Solidsmith.icon/Assets/03-solid-blue-cutout.svg; original animation | MIT OR Apache-2.0 |
| assets/styles/web/loading.css | Solidsmith contributors | Original Solidsmith resource data or drawing code, collected without redesign | MIT OR Apache-2.0 |
| crates/io/src/native_cad/fixtures/block.FCStd | Solidsmith contributors | original 10 × 20 × 30 mm test box saved by FreeCAD 1.1.4 run as an external program (`make_block_fcstd.py` beside it); no third-party geometry | MIT OR Apache-2.0 |
| crates/io/src/native_cad/fixtures/block.3dm | Solidsmith contributors | original 10 × 20 × 30 mm test box written with McNeel's MIT-licensed rhino3dm Python package (`make_block_3dm.py` beside it); no third-party geometry | MIT OR Apache-2.0 |
| assets/icons/ui/design/aliases.toml | Solidsmith contributors | icon ids that share a master, original Solidsmith data | MIT OR Apache-2.0 |
| assets/icons/ui/design/palette.toml | Solidsmith contributors | icon colour roles (light and dark) and accents, original Solidsmith data | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/bug_report.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/chevron.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/chevron_right.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/close.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/disclosure_closed.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/disclosure_open.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/fit.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/folder.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/help.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/home.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/lookat.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/moon.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/nav_settings.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/orbit.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/plus.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/pan.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/perspective.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/prefs_appearance.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/prefs_assistant.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/prefs_design.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/prefs_general.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/prefs_graphics.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/prefs_layout.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/prefs_mouse3d.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/prefs_navigation.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/prefs_plugins.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/prefs_shortcuts.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/prefs_sketch.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/prefs_tools.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/redo.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/save.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/settings.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/settings_keyboard.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/settings_monitor.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/settings_mouse.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/settings_palette.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/settings_plug.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/spin.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/style.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/undo.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/zoom.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/accessibility.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/activate.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/align.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/appearance.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/arc3.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/arc_center.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/arc_slot.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/arc_tangent.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/as_built.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/auto_constrain.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/axis.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/bendtable.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/blend.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/blend_surface.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/body.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/boss.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/boundary.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/boundary_fill.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/box.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/break.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/c_coincident.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/c_collinear.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/c_concentric.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/c_equal.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/c_fix.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/c_hv.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/c_midpoint.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/c_parallel.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/c_perpendicular.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/c_polygon.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/c_smooth.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/c_symmetry.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/c_tangent.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/camera.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/canvas.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/capture.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/center_of_mass.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/centerline.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/chain.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/chamfer.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/checkpoint.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/circle.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/circle2.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/circle3.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/circle_tt.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/circle_ttt.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/coil.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/colored_components.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/combine.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/component.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |

### Render workspace icon drafts

Original Solidsmith drawings, Open for owner review; no runtime assignments.

| File | Author | Source | Licence |
|---|---|---|---|
| assets/icons/ui/design/svg/model/render_all_shots.svg | Solidsmith contributors | original Camera icon master (64 grid): stacked camera bodies with a lower-right play badge | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_appearance.svg | Solidsmith contributors | original Looks icon master (64 grid): shaded look sphere on the top corner of an accent-shaded cube | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_aspect_frame.svg | Solidsmith contributors | original Camera icon master (64 grid): wide rounded frame with two opposite corner brackets | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_backdrop.svg | Solidsmith contributors | original Studio icon master (64 grid): grey cube in front of a tall upright accent panel | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_cancel.svg | Solidsmith contributors | original Output icon master (64 grid): picture with a lower-right round stop badge | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_check.svg | Solidsmith contributors | original Output icon master (64 grid): picture with a lower-right round check badge | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_compare.svg | Solidsmith contributors | original Output icon master (64 grid): two grey pictures split by a vertical slider with opposed arrows | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_copy_look.svg | Solidsmith contributors | original Looks icon master (64 grid): look sphere in a square frame offset from a second frame outline | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_execute.svg | Solidsmith contributors | original Output icon master (64 grid): picture with a lower-right round play badge | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_export_image.svg | Solidsmith contributors | original Output icon master (64 grid): picture with a large outward diagonal arrow at the upper right | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_exposure.svg | Solidsmith contributors | original Camera icon master (64 grid): split light and accent disc with plus and minus marks | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_fit_product.svg | Solidsmith contributors | original Camera icon master (64 grid): grey cube inside four corner brackets | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_frame_guides.svg | Solidsmith contributors | original Camera icon master (64 grid): rounded frame with a rule-of-thirds grid and four accent nodes | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_from_material.svg | Solidsmith contributors | original Looks icon master (64 grid): look sphere struck through above a plain grey slab | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_ground.svg | Solidsmith contributors | original Studio icon master (64 grid): grey cube on a slab with an accent contact patch | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_lens.svg | Solidsmith contributors | original Camera icon master (64 grid): concentric lens rings around an accent centre | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_library.svg | Solidsmith contributors | original Studio icon master (64 grid): row of three look spheres, middle one accent, on a shelf bar | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_light_area.svg | Solidsmith contributors | original Lights icon master (64 grid): slanted flat panel above a dotted accent stem | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_light_by_highlight.svg | Solidsmith contributors | original Lights icon master (64 grid): shaded sphere with a ringed highlight target and dotted stem arriving from above | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_light_point.svg | Solidsmith contributors | original Lights icon master (64 grid): round source disc with centre dot above a dotted accent stem | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_light_spot.svg | Solidsmith contributors | original Lights icon master (64 grid): cone source with curved rim above a dotted accent stem | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_lighting_presets.svg | Solidsmith contributors | original Studio icon master (64 grid): two stacked cards, the front one holding a light bulb | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_output_size.svg | Solidsmith contributors | original Output icon master (64 grid): picture frame with a diagonal double-headed dimension arrow | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_paint.svg | Solidsmith contributors | original Looks icon master (64 grid): flat brush with an accent tip, light ferrule and slate handle | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_pick_up.svg | Solidsmith contributors | original Looks icon master (64 grid): eyedropper with a grey bulb, slate crossbar and accent sampling tip | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_quality.svg | Solidsmith contributors | original Output icon master (64 grid): three rising bars, the tallest in accent | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_replace_all.svg | Solidsmith contributors | original Looks icon master (64 grid): grey sample sphere with an arrow to an accent sphere beside a second one | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_results.svg | Solidsmith contributors | original Output icon master (64 grid): stack of two pictures, the front one showing a landscape | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_same_look.svg | Solidsmith contributors | original Looks icon master (64 grid): look sphere and three cubes, the two matching cubes ringed | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_shot.svg | Solidsmith contributors | original Camera icon master (64 grid): camera body with a lower-right plus badge | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_shots_strip.svg | Solidsmith contributors | original Camera icon master (64 grid): strip of three small frames, the middle one in accent | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_solo_light.svg | Solidsmith contributors | original Lights icon master (64 grid): point source and dotted stem inside four corner focus brackets | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_sun.svg | Solidsmith contributors | original Lights icon master (64 grid): source disc with eight short radial rays | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_transparent_background.svg | Solidsmith contributors | original Output icon master (64 grid): grey cube on a checker tile | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_update_shot.svg | Solidsmith contributors | original Camera icon master (64 grid): camera body with a lower-right refresh badge | MIT OR Apache-2.0 |

### UI icon masters (continued)

| File | Author | Source | Licence |
|---|---|---|---|
| assets/icons/ui/design/svg/model/compute.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/configuration_table.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/conic.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/constrainer.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/construction.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/convert_sheet.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/copy.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/curvature_comb.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/curvature_map.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/cylinder.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/decal.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/delete.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/dimension.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/dots.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/draft.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/draft_analysis.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/drive.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/dxf.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/edge_continuity.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/edit.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/ellipse.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/emboss.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/environment_map.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/explode.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/export.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/extend.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/extrude.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/eye.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/fastener.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/fillet.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/filter.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/finish.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/fit_section.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/flange.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/flat.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/fold.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/folder.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/graph.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/hem.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/hole.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/import.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/include.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/inspect.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/interference.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/intersect.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/intersection_curve.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/iso_analysis.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/iso_curve.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/joint.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/joint_origin.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/library.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/line.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/line_mid.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/lip.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/list.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/loft.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/material.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/measure.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/min_radius.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/mirror.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/motion_link.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/move.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/new.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/offset.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/offset_face.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/offset_surface.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/open.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/origin.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/params.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/paste.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/patch.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/pattern.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/pattern_circ.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/pattern_path.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/pattern_rect.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/perforate.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/pipe.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/silhouette.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/assistant.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/agent_link.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/offset_two_sides.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/offset_symmetric.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/import_tray.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/export_tray.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/send.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/stop.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/shield.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/status_recovery.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/frame_member.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_environment.svg | Solidsmith contributors | original Studio icon master (64 grid): dome with a grey cube inside | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/style_shaded.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/style_wireframe.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/style_hidden_edges.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/style_translucent.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/select_faces_edges.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/select_bodies.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/select_features.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/select_parts.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/select_components.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/select_sketch.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/display.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/grid.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/ground_shadow.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/ambient_occlusion.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/placeholder.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/plane.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/plane_angle.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/plastic_assign.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/plastic_rule.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/play.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/point.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/polygon.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/presspull.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/project.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/project_surface.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/rect.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/rect3.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/rect_center.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/rect_rounded.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/refold.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/rename.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_image.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/render_scene.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/reorder.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/rest.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/restore.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/reverse_normal.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/revert.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/revolve.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/rib.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/rigid_group.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/rollback.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/ruled.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/scale.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/section.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/select.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/sheet.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/sheet_rules.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/shell.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/sketch.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/sketch_chamfer.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/sketch_fillet.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/sketch_mirror.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/slot.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/snap.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/sphere.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/spline.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/spline_cv.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/split_body.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/split_face.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/spun.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/star.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/status_error.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/status_warning.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/stitch.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/suppress.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/surface.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/svg.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/sweep.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/text.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/thicken.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/thread.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/torus.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/trim.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/unfold.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/unstitch.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/untrim.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/update.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/zebra.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/small/as_built.svg | Solidsmith contributors | small-slot icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/small/component.svg | Solidsmith contributors | small-slot icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/small/curvature_comb.svg | Solidsmith contributors | small-slot icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/small/interference.svg | Solidsmith contributors | small-slot icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/small/iso_analysis.svg | Solidsmith contributors | small-slot icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/small/joint.svg | Solidsmith contributors | small-slot icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/small/pattern.svg | Solidsmith contributors | small-slot icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/small/section.svg | Solidsmith contributors | small-slot icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/workspace/ws_animate.svg | Solidsmith contributors | workspace icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/polygon_circumscribed.svg | Solidsmith contributors | original sketch variant icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/polygon_edge.svg | Solidsmith contributors | original sketch variant icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/slot_overall.svg | Solidsmith contributors | original sketch variant icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/slot_center.svg | Solidsmith contributors | original sketch variant icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/arc_slot_center.svg | Solidsmith contributors | original sketch variant icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/sketch_chamfer_angle.svg | Solidsmith contributors | original sketch variant icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/sketch_chamfer_two_distances.svg | Solidsmith contributors | original sketch variant icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/gear.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/rack_pinion.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/screw.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/cam.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/trace.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/agent_chat.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/normal.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/infinite.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/pattern_flip.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/pattern_center.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/reach.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/cut.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/compare.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/web.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/replace_face.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/bounding_solid.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/delete_face.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/snap_fit.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/contact_sets.svg | Solidsmith contributors | original contact icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/contact_all.svg | Solidsmith contributors | original contact icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/contact_off.svg | Solidsmith contributors | original contact icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/contact_new.svg | Solidsmith contributors | original contact icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/axis_through_cylinder.svg | Solidsmith contributors | original construction icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/axis_normal_to_face.svg | Solidsmith contributors | original construction icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/axis_two_planes.svg | Solidsmith contributors | original construction icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/axis_two_points.svg | Solidsmith contributors | original construction icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/axis_edge.svg | Solidsmith contributors | original construction icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/point_vertex.svg | Solidsmith contributors | original construction icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/point_two_edges.svg | Solidsmith contributors | original construction icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/point_three_planes.svg | Solidsmith contributors | original construction icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/point_center.svg | Solidsmith contributors | original construction icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/point_edge_plane.svg | Solidsmith contributors | original construction icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/point_along_path.svg | Solidsmith contributors | original construction icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/plane_midplane.svg | Solidsmith contributors | original construction icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/plane_two_edges.svg | Solidsmith contributors | original construction icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/plane_three_points.svg | Solidsmith contributors | original construction icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/plane_tangent.svg | Solidsmith contributors | original construction icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/plane_along_path.svg | Solidsmith contributors | original construction icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/plane_perpendicular.svg | Solidsmith contributors | original construction icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/workspace/ws_design.svg | Solidsmith contributors | workspace icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/workspace/ws_drawing.svg | Solidsmith contributors | workspace icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/workspace/ws_render.svg | Solidsmith contributors | workspace icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/references.toml | Solidsmith contributors | original icon authoring or review tool resource | MIT OR Apache-2.0 |
| assets/icons/ui/design/tools/author/bases.py | Solidsmith contributors | original icon authoring or review tool resource | MIT OR Apache-2.0 |
| assets/icons/ui/design/tools/author/new_icon.py | Solidsmith contributors | original icon authoring or review tool resource | MIT OR Apache-2.0 |
| assets/icons/ui/design/tools/review/index.html | Solidsmith contributors | original icon authoring or review tool resource | MIT OR Apache-2.0 |
| assets/icons/ui/design/tools/review/guide.html | Solidsmith contributors | original icon authoring or review tool resource | MIT OR Apache-2.0 |
| assets/icons/ui/design/tools/review/review.css | Solidsmith contributors | original icon authoring or review tool resource | MIT OR Apache-2.0 |
| assets/icons/ui/design/tools/review/review.js | Solidsmith contributors | original icon authoring or review tool resource | MIT OR Apache-2.0 |
| assets/icons/ui/design/tools/icon_palette.py | Solidsmith contributors | Original shared semantic icon palette resolver for repository tools | MIT OR Apache-2.0 |
| assets/icons/ui/design/tools/icon_status.py | Solidsmith contributors | icon lint, status page or preview tool (standard-library Python), original Solidsmith code | MIT OR Apache-2.0 |
| assets/icons/ui/design/tools/lint_icons.py | Solidsmith contributors | icon lint, status page or preview tool (standard-library Python), original Solidsmith code | MIT OR Apache-2.0 |
| assets/icons/ui/design/tools/preview.py | Solidsmith contributors | icon lint, status page or preview tool (standard-library Python), original Solidsmith code | MIT OR Apache-2.0 |
| assets/icons/ui/design/tools/review_state.py | Solidsmith contributors | icon review state helper (standard-library Python), original Solidsmith code | MIT OR Apache-2.0 |
| assets/icons/ui/design/requested_icons.txt | Solidsmith contributors | Runtime icon requests collected by the headless menu audit, original Solidsmith data | MIT OR Apache-2.0 |
| assets/icons/ui/design/review.json | Solidsmith contributors | icon review state (accepted icons and notes), original Solidsmith data | MIT OR Apache-2.0 |
| assets/icons/ui/design/tools/author/lib.py | Solidsmith contributors | icon authoring script (standard-library Python), original Solidsmith code | MIT OR Apache-2.0 |

| docs/book/src/og.png | Solidsmith contributors | copy of assets/branding/web/source/og.png staged by cargo xtask book (gitignored) | MIT OR Apache-2.0 |
| docs/sponsors/blacksmith-ci-powered.png | Blacksmith (blacksmith.sh) | "CI powered by Blacksmith" sponsor banner supplied by Blacksmith for the sponsorship credit; used only to credit the sponsor, not part of the application | © Blacksmith, used with permission |
| assets/icons/ui/design/svg/model/path_flange.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| docs/book/src/generated/hem.png | Solidsmith contributors | rendered by `cargo xtask book` (`solidsmith-cli snapshot docs/book/scripts/hem.json`; gitignored), original sheet metal demonstration | MIT OR Apache-2.0 |
| docs/book/scripts/hem.json | Solidsmith contributors | original parametric recipe for an 80 × 50 mm sheet with an 8 mm returned edge, authored for the command guide | MIT OR Apache-2.0 |
| assets/styles/ui/themes/clay.json | Solidsmith contributors | original Solidsmith theme file (Clay, light): warm paper palette chosen for Solidsmith | MIT OR Apache-2.0 |
| assets/styles/ui/themes/clay-dark.json | Solidsmith contributors | original Solidsmith theme file (Clay Dark): warm charcoal palette chosen for Solidsmith | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/cone.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/locales/ar.json | Abdullah (A-AL-ANAZI) | Original Arabic translation of Solidsmith's English catalog; no commercial CAD text used | MIT OR Apache-2.0 |
| assets/locales/he.json | Solidsmith contributors | Original Hebrew translation of Solidsmith's English catalog; no commercial CAD text used | MIT OR Apache-2.0 |
| assets/styles/ui/themes/cga.json | Solidsmith contributors | original Solidsmith theme file using the 16 colours of the IBM Color Graphics Adapter (black, cyan, magenta, and white of its high-intensity palette 1 for the interface) | MIT OR Apache-2.0 |
| assets/styles/ui/themes/1-bit.json | Solidsmith contributors | original Solidsmith theme file | MIT OR Apache-2.0 |
| assets/styles/ui/themes/1-bit-dark.json | Solidsmith contributors | original Solidsmith theme file | MIT OR Apache-2.0 |
| `assets/locales/pending/navbar-customisation.json` | Solidsmith contributors | Original navigation bar editor translations | MIT OR Apache-2.0 |
| `assets/locales/pending/navbar-slots.json` | Solidsmith contributors | Original navigation slot translations | MIT OR Apache-2.0 |
| assets/locales/bs.json | Tarik Sijercic (tsijercic1) | Original Bosnian translation of Solidsmith's English catalog; no commercial CAD text used | MIT OR Apache-2.0 |
| assets/locales/hr.json | Tarik Sijercic (tsijercic1) | Original Croatian translation of Solidsmith's English catalog; no commercial CAD text used | MIT OR Apache-2.0 |
| assets/locales/sr.json | Tarik Sijercic (tsijercic1) | Original Serbian (ekavian, Cyrillic) translation of Solidsmith's English catalog; no commercial CAD text used | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/finish_edit.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| `assets/locales/pending/sketch-tracking-guides.json` | Solidsmith contributors | Original alignment-guide preference labels and help text | MIT OR Apache-2.0 |
| assets/locales/pending/point-clouds.json | Solidsmith contributors | Original point-cloud UI strings and translations | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/feature_group.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/mesh_body.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/timeline_play.svg | Solidsmith contributors | Original history playback icon; canonical grey_cube base and original action cue | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/timeline_stop.svg | Solidsmith contributors | Original history playback icon; canonical grey_cube base and original action cue | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/status_pin.svg | Solidsmith contributors | Original analytic pin icon for persistent viewport results | MIT OR Apache-2.0 |
| assets/locales/pending/render-wear.json | Solidsmith contributors | Original UI labels and translations | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/equation_curve.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/locales/pending/sketch-recent-snap.json | Solidsmith contributors | Original preference messages for the recent centre and midpoint snap | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/construction_line.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/construction_line_horizontal.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/construction_line_vertical.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/construction_line_angle.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/construction_line_parallel.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/construction_line_perpendicular.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/construction_line_tangent_external.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/construction_line_tangent_at.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/construction_line_tangent_two.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/construction_line_bisector.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| docs/book/src/generated/equation-circle.png | Solidsmith contributors | rendered by `cargo xtask book` (`solidsmith-cli snapshot docs/book/scripts/equation-circle.json`; gitignored) | MIT OR Apache-2.0 |
| docs/book/src/generated/equation-flower.png | Solidsmith contributors | rendered by `cargo xtask book` (`solidsmith-cli snapshot docs/book/scripts/equation-flower.json`; gitignored) | MIT OR Apache-2.0 |
| docs/book/src/generated/equation-involute.png | Solidsmith contributors | rendered by `cargo xtask book` (`solidsmith-cli snapshot docs/book/scripts/equation-involute.json`; gitignored) | MIT OR Apache-2.0 |
| docs/book/src/generated/equation-line.png | Solidsmith contributors | rendered by `cargo xtask book` (`solidsmith-cli snapshot docs/book/scripts/equation-line.json`; gitignored) | MIT OR Apache-2.0 |
| docs/book/src/generated/equation-parabola.png | Solidsmith contributors | rendered by `cargo xtask book` (`solidsmith-cli snapshot docs/book/scripts/equation-parabola.json`; gitignored) | MIT OR Apache-2.0 |
| docs/book/src/generated/equation-spiral.png | Solidsmith contributors | rendered by `cargo xtask book` (`solidsmith-cli snapshot docs/book/scripts/equation-spiral.json`; gitignored) | MIT OR Apache-2.0 |
| docs/book/src/generated/equation-wave.png | Solidsmith contributors | rendered by `cargo xtask book` (`solidsmith-cli snapshot docs/book/scripts/equation-wave.json`; gitignored) | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/print_face.svg | Solidsmith contributors | two-tone modelling icon master (64 grid), original Solidsmith artwork | MIT OR Apache-2.0 |
| assets/locales/fi.json | Solidsmith contributors | Original Finnish translation of Solidsmith's English catalog, AI-assisted (Claude Code) clean-room from the English and not yet reviewed line by line by a native speaker; no commercial CAD text used | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/model/plane_axis_point.svg | Solidsmith contributors | original icon master (64 grid) | MIT OR Apache-2.0 |
| docs/design/preferences/figures/aa-2x.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/aa-2x.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/aa-4x.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/aa-4x.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/aa-8x.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/aa-8x.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/aa-off.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/aa-off.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/agent-pointer-speed.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/agent-pointer-speed.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/ao-on.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/ao-on.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/assembly-move.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/assembly-move.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/browser-card.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/browser-card.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/browser-column.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/browser-column.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/colour-vision-axes.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/colour-vision-axes.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/colour-vision-bends.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/colour-vision-bends.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/colour-vision-draft.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/colour-vision-draft.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/colour-vision-failed.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/colour-vision-failed.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/colour-vision-fixed.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/colour-vision-fixed.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/colour-vision-projected.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/colour-vision-projected.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/direct-interaction.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/direct-interaction.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/frame-rate.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/frame-rate.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/history-bottom.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/history-bottom.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/history-in-browser.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/history-in-browser.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/history-left.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/history-left.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/history-under-browser.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/history-under-browser.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/layout-schematic.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/layout-schematic.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/measure-axis-distances.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/measure-axis-distances.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/mouse-mapping.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/mouse-mapping.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/navigator-axes.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/navigator-axes.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/navigator-cube.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/navigator-cube.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/orbit-centre.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/orbit-centre.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/orbit-cursor.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/orbit-cursor.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/orbit-origin.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/orbit-origin.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/orientation-y-up.dark.svg | Solidsmith contributors | original Solidsmith artwork: the Orientation figure copied from the app and extracted from the Preferences redesign board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/orientation-y-up.light.svg | Solidsmith contributors | original Solidsmith artwork: the Orientation figure copied from the app and extracted from the Preferences redesign board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/orientation-z-up.dark.svg | Solidsmith contributors | original Solidsmith artwork: the Orientation figure copied from the app and extracted from the Preferences redesign board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/orientation-z-up.light.svg | Solidsmith contributors | original Solidsmith artwork: the Orientation figure copied from the app and extracted from the Preferences redesign board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/quick-measure.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/quick-measure.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/select-bodies.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/select-bodies.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/select-faces-edges.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/select-faces-edges.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/sketch-conflict.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/sketch-conflict.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/sketch-exit-keep.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/sketch-exit-keep.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/sketch-exit-previous.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/sketch-exit-previous.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/sketch-exit-unless-turned.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/sketch-exit-unless-turned.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/sketch-guides.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/sketch-guides.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/sketch-hide-glyphs.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/sketch-hide-glyphs.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/sketch-state-icons.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/sketch-state-icons.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/style-shaded-edges.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/style-shaded-edges.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/tess-coarse.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/tess-coarse.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/tess-fine.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/tess-fine.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/tess-normal.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/tess-normal.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/tess-ultra.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/tess-ultra.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/theme-dark.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/theme-dark.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/theme-gruvbox.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/theme-gruvbox.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/theme-imported.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/theme-imported.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/theme-light.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/theme-light.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/theme-system-dark-half.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/theme-system-dark-half.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/theme-system-light-half.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/theme-system-light-half.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/tool-cursor.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/tool-cursor.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/tool-extrude.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/tool-extrude.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/tool-hole.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/tool-hole.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/tool-placement.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/tool-placement.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/vertex-markers.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/vertex-markers.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/view-angles.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/view-angles.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/view-grid.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/view-grid.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/view-perspective.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/view-perspective.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/view-shadow.dark.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| docs/design/preferences/figures/view-shadow.light.svg | Solidsmith contributors | original Solidsmith artwork drawn for the Preferences redesign (tracker #9863), extracted from the design board as a standalone SVG | MIT OR Apache-2.0 |
| `assets/locales/pending/sketch-offset-instances.json` | Solidsmith contributors | Original offset instance-count label and translations | MIT OR Apache-2.0 |
| assets/icons/ui/design/svg/flat/settings_profile.svg | Solidsmith contributors | single-colour application/navigation icon master (64 grid), original analytic head and shoulder geometry for settings profiles | MIT OR Apache-2.0 |
| tests/kernel-cases/import/4561-convert-box-mesh-with-a-zero-area-triangle/box-degenerate.stl | Solidsmith contributors | synthetic ASCII STL written by hand: a box with one zero-area triangle, for the kernel regression case of tangency-labs/solidcraft#4561 | MIT OR Apache-2.0 |
| assets/locales/pending/theme-blur-ui.json | Solidsmith contributors | Original blur controls, notices and translations | MIT OR Apache-2.0 |
| crates/ui-egui/tests/fixtures/theme-blur.json | Solidsmith contributors | Original theme for blur option scenarios and manager review | MIT OR Apache-2.0 |
| assets/locales/ka.json | David Machakhelidze (devius) | Original Georgian translation of Solidsmith's English catalog, written with Claude Code from the meaning of each English string; terminology and doubtful strings decided by a native Georgian speaker; no commercial CAD text used | MIT OR Apache-2.0 |
| assets/branding/logo/solidsmith-mark.svg | Solidsmith contributors | static Solidsmith mark: the five faces and gradients of assets/icons/app/macos/source/Solidsmith.icon/Assets/03-solid-blue-cutout.svg (the final frame of assets/styles/web/loading-logo.svg), cropped to the mark's bounds; original artwork | MIT OR Apache-2.0 |
| assets/branding/logo/solidsmith-mark-mono-black.svg | Solidsmith contributors | one-colour (black) version of assets/branding/logo/solidsmith-mark.svg, same paths; original artwork | MIT OR Apache-2.0 |
| assets/branding/logo/solidsmith-mark-mono-white.svg | Solidsmith contributors | one-colour (white) version of assets/branding/logo/solidsmith-mark.svg, same paths; original artwork | MIT OR Apache-2.0 |
| assets/branding/logo/solidsmith-mark-512.png | Solidsmith contributors | transparent 512 px render of assets/branding/logo/solidsmith-mark.svg (cairosvg); original artwork | MIT OR Apache-2.0 |
| assets/branding/logo/solidsmith-mark-1024.png | Solidsmith contributors | transparent 1024 px render of assets/branding/logo/solidsmith-mark.svg (cairosvg); original artwork | MIT OR Apache-2.0 |
| assets/branding/logo/solidsmith-mark-2048.png | Solidsmith contributors | transparent 2048 px render of assets/branding/logo/solidsmith-mark.svg (cairosvg); original artwork | MIT OR Apache-2.0 |
| assets/branding/logo/solidsmith-wordmark.svg | Solidsmith contributors | the name Solidsmith set like the loading logo's wordmark (bold, +0.02 em tracking, #15499e) in IBM Plex Sans Bold from assets/fonts/ibm-plex-sans (SIL OFL 1.1, IBM Corp.), outlined to paths; no font file is embedded | MIT OR Apache-2.0 |
| assets/branding/logo/solidsmith-lockup-horizontal.svg | Solidsmith contributors | assets/branding/logo/solidsmith-mark.svg beside assets/branding/logo/solidsmith-wordmark.svg, in the proportions of assets/branding/web/source/og.png; original artwork | MIT OR Apache-2.0 |
| assets/styles/ui/themes/onyx.json | Solidsmith contributors | original Onyx stone-grey palette, option overlays and semantic icon-role mapping; existing Solidsmith artwork is unchanged | MIT OR Apache-2.0 |
| docs/book/src/media/ui-tour/window.png | Solidsmith contributors | screenshot of Solidsmith 0.29.0 itself (`solidsmith --sample`, default dark theme, software renderer, 1280x800) with original numbered callouts (digits set in DejaVu Sans Bold), captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| docs/book/src/media/ui-tour/navigate.mp4 | Solidsmith contributors | original screen recording of the actual Solidsmith 0.29.0 English desktop UI (`--sample`, default dark theme, software renderer) orbiting, panning and zooming, with the original drawn pointer of examples/demo/overlay.py and labels set in DejaVu Sans Bold, captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| assets/locales/pending/sketch-tool-hints.json | carlymx | Original sketch tool hints and engine error translations | MIT OR Apache-2.0 |
| assets/locales/pending/primitive-pick-field-tips.json | Solidsmith contributors | Original primitive pick-field tooltip translations | MIT OR Apache-2.0 |
| assets/locales/pending/ribbon-panel-hover.json | Solidsmith contributors | Original ribbon panel menu preference labels and translations | MIT OR Apache-2.0 |
| docs/screenshots/box-insert-designer.gif | Shawn Marx | original recording made with BoxInsertDesigner's own Motion export (its three.js viewer) of its example game `examples/ccg-example` (https://github.com/ShawnMarx/BoxInsertDesigner): a card game box insert whose trays were built by Solidsmith through `solidsmith-cli`; the published card, board and box art in it is pixelated beyond recognition, and no logos or game names are shown (2.5× playback, 800 px wide) | MIT OR Apache-2.0 |
| docs/screenshots/box-insert-designer.mp4 | Shawn Marx | original recording made with BoxInsertDesigner's own Motion export (its three.js viewer) of its example game `examples/ccg-example` (https://github.com/ShawnMarx/BoxInsertDesigner): a card game box insert whose trays were built by Solidsmith through `solidsmith-cli`; the published card, board and box art in it is pixelated beyond recognition, and no logos or game names are shown (1280×720) | MIT OR Apache-2.0 |
| assets/locales/pending/agent-http-consent.json | Solidsmith contributors | Original endpoint-specific HTTP consent messages and translations | MIT OR Apache-2.0 |
| assets/locales/pending/ground-shadow-placement.json | Solidsmith contributors | Original ground-shadow placement preference messages and translations | MIT OR Apache-2.0 |
| docs/book/src/media/assemblies/components.mp4 | Solidsmith contributors | original screen recording of the actual Solidsmith 0.29.0 English desktop UI (default dark theme, software renderer) making components from two original hinge-leaf bodies, with the original drawn pointer of examples/demo/overlay.py and labels set in DejaVu Sans Bold, captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| docs/book/src/media/assemblies/joint.mp4 | Solidsmith contributors | original screen recording of the actual Solidsmith 0.29.0 English desktop UI (default dark theme, software renderer) creating a revolute joint between two hinge leaves, with the original drawn pointer of examples/demo/overlay.py and labels set in DejaVu Sans Bold, captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| docs/book/src/media/assemblies/drag.mp4 | Solidsmith contributors | original screen recording of the actual Solidsmith 0.29.0 English desktop UI (default dark theme, software renderer) dragging a jointed hinge leaf, with the original drawn pointer of examples/demo/overlay.py and labels set in DejaVu Sans Bold, captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| docs/book/src/media/assemblies/drive.mp4 | Solidsmith contributors | original screen recording of the actual Solidsmith 0.29.0 English desktop UI (default dark theme, software renderer) driving a revolute joint between two hinge leaves, with the original drawn pointer of examples/demo/overlay.py and labels set in DejaVu Sans Bold, captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| docs/book/src/media/features/extrude.mp4 | Solidsmith contributors | original screen recording of the actual Solidsmith 0.29.0 English desktop UI (default dark theme, software renderer) extruding a sketch's profile, with the original drawn pointer of examples/demo/overlay.py and labels set in DejaVu Sans Bold, captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| docs/book/src/media/features/shell.mp4 | Solidsmith contributors | original screen recording of the actual Solidsmith 0.29.0 English desktop UI (default dark theme, software renderer) shelling a block, with the original drawn pointer of examples/demo/overlay.py and labels set in DejaVu Sans Bold, captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| docs/book/src/media/features/fillet.mp4 | Solidsmith contributors | original screen recording of the actual Solidsmith 0.29.0 English desktop UI (default dark theme, software renderer) filleting four edges, with the original drawn pointer of examples/demo/overlay.py and labels set in DejaVu Sans Bold, captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| docs/book/src/media/features/edit.mp4 | Solidsmith contributors | original screen recording of the actual Solidsmith 0.29.0 English desktop UI (default dark theme, software renderer) editing a feature from the timeline, with the original drawn pointer of examples/demo/overlay.py and labels set in DejaVu Sans Bold, captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| docs/book/src/media/features/hole.mp4 | Solidsmith contributors | original screen recording of the actual Solidsmith 0.29.0 English desktop UI (default dark theme, software renderer) placing a hole, with the original drawn pointer of examples/demo/overlay.py and labels set in DejaVu Sans Bold, captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| docs/book/src/media/features/pattern.mp4 | Solidsmith contributors | original screen recording of the actual Solidsmith 0.29.0 English desktop UI (default dark theme, software renderer) patterning a hole around an axis, with the original drawn pointer of examples/demo/overlay.py and labels set in DejaVu Sans Bold, captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| docs/book/src/media/parameters/dialog.png | Solidsmith contributors | screenshot of Solidsmith 0.29.0 itself (default dark theme, software renderer) showing the Parameters dialog, with original numbered callouts (digits set in DejaVu Sans Bold), captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| docs/book/src/media/files/menu.png | Solidsmith contributors | screenshot of Solidsmith 0.29.0 itself (`--sample`, default dark theme, software renderer) showing the File menu, with original numbered callouts (digits set in DejaVu Sans Bold), captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| docs/book/src/media/getting-started/start.png | Solidsmith contributors | screenshot of Solidsmith 0.29.0 itself (default dark theme, software renderer, 1280x800) showing the start page with its built-in sample thumbnails, with original numbered callouts (digits set in DejaVu Sans Bold), captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| docs/book/src/media/getting-started/sample.mp4 | Solidsmith contributors | original screen recording of the actual Solidsmith 0.29.0 English desktop UI (default dark theme, software renderer) opening the built-in Sample Plate from the start page, with the original drawn pointer of examples/demo/overlay.py and labels set in DejaVu Sans Bold, captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| docs/book/src/media/sheet-metal/base.mp4 | Solidsmith contributors | original screen recording of the actual Solidsmith 0.29.0 English desktop UI (default dark theme, software renderer) making a sheet metal base flange, with the original drawn pointer of examples/demo/overlay.py and labels set in DejaVu Sans Bold, captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| docs/book/src/media/sheet-metal/flange.mp4 | Solidsmith contributors | original screen recording of the actual Solidsmith 0.29.0 English desktop UI (default dark theme, software renderer) adding sheet metal edge flanges, with the original drawn pointer of examples/demo/overlay.py and labels set in DejaVu Sans Bold, captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| docs/book/src/media/sheet-metal/flat.mp4 | Solidsmith contributors | original screen recording of the actual Solidsmith 0.29.0 English desktop UI (default dark theme, software renderer) showing a sheet metal flat pattern, with the original drawn pointer of examples/demo/overlay.py and labels set in DejaVu Sans Bold, captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| docs/book/src/media/sketching/create.mp4 | Solidsmith contributors | original screen recording of the actual Solidsmith 0.29.0 English desktop UI (default dark theme, software renderer) starting a sketch on the Top plane, with the original drawn pointer of examples/demo/overlay.py and labels set in DejaVu Sans Bold, captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| docs/book/src/media/sketching/draw.mp4 | Solidsmith contributors | original screen recording of the actual Solidsmith 0.29.0 English desktop UI (default dark theme, software renderer) drawing a rectangle and a circle with typed values, with the original drawn pointer of examples/demo/overlay.py and labels set in DejaVu Sans Bold, captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| docs/book/src/media/sketching/dimension.mp4 | Solidsmith contributors | original screen recording of the actual Solidsmith 0.29.0 English desktop UI (default dark theme, software renderer) dimensioning a sketch until it is fully constrained, with the original drawn pointer of examples/demo/overlay.py and labels set in DejaVu Sans Bold, captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| docs/book/src/media/sketching/parts.png | Solidsmith contributors | screenshot of Solidsmith 0.29.0 itself (default dark theme, software renderer, 1280x800) showing a fully constrained sketch, with original numbered callouts (digits set in DejaVu Sans Bold), captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| docs/book/src/media/sketching/offset.mp4 | Solidsmith contributors | original screen recording of the actual Solidsmith 0.29.0 English desktop UI (default dark theme, software renderer) offsetting a sketch outline, with the original drawn pointer of examples/demo/overlay.py and labels set in DejaVu Sans Bold, captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |
| docs/book/src/media/sketching/trim.mp4 | Solidsmith contributors | original screen recording of the actual Solidsmith 0.29.0 English desktop UI (default dark theme, software renderer) trimming a notch in a sketch, with the original drawn pointer of examples/demo/overlay.py and labels set in DejaVu Sans Bold, captured with examples/demo/doc_shots.py | MIT OR Apache-2.0 |

## Code adapted from sibling projects

| Solidsmith file | Adapted from | Licence |
|---|---|---|
| xtask/src/layers.rs | CADsmith `xtask/src/layers.rs` (dependency layering rules and tests) | MIT OR Apache-2.0 |
| apps/solidsmith/src/control_server.rs | CADsmith `apps/cadsmith/src/control_server.rs` (loopback JSON-lines server) | MIT OR Apache-2.0 |
| crates/ui-egui/src/lib.rs (screenshot queue, synthetic input) | CADsmith `crates/ui-egui/src/lib.rs` | MIT OR Apache-2.0 |
| crates/ui-egui/src/control.rs (method dispatch and screenshot saving) | CADsmith `crates/ui-egui/src/control.rs` | MIT OR Apache-2.0 |
| crates/ui-egui/src/gpu.rs (paint-callback structure) | CADsmith `crates/ui-egui/src/gpu.rs` | MIT OR Apache-2.0 |
| crates/mcp/src/server.rs, crates/mcp/src/backend.rs (JSON-RPC framing, MCP lifecycle, control-port bridge) | CADsmith `crates/mcp/src/{server,backend}.rs` | MIT OR Apache-2.0 |
| crates/mcp/src/tools.rs (tool schema helpers and argument checking) | CADsmith `crates/mcp/src/tools.rs`, Cellsmith `crates/mcp/src/tools.rs` | MIT OR Apache-2.0 |
| crates/io/src/threemf.rs (bounded zip part reading and writing) | Cellsmith `crates/xlsx/src/package.rs` | MIT OR Apache-2.0 |
| crates/ui-egui/src/titlebar.rs (title bar drag, caption buttons, resize zones, their tests) and the window setup in apps/solidsmith/src/main.rs | Pathsmith `crates/ui-egui/src/titlebar.rs`, `apps/pathsmith/src/main.rs` | MIT OR Apache-2.0 |
| apps/solidsmith/src/open_documents.rs (macOS: files opened from Finder and the Dock, its test) | Pathsmith `apps/pathsmith/src/open_documents.rs` (https://github.com/mahmudurrahman420/pathsmith) | MIT OR Apache-2.0 |
| LICENSE-MIT, LICENSE-APACHE (licence text) | CADsmith | — |
| .github/workflows/release.yml, .github/workflows/packaging-lint.yml and packaging/ (release pipeline: Linux AppImage/deb/rpm/Flatpak/tar.gz, FreeBSD, macOS dmg, Windows MSI/portable zip, web zip; AppRun, nfpm, Flatpak, WiX and Info.plist templates) | Pixelsmith (https://github.com/mahmudurrahman420/pixelsmith) `.github/workflows/{release,packaging-lint}.yml`, `packaging/` | Apache-2.0 |

## Incremental HDR codec adapter

`crates/render/src/lighting/hdr.rs` adapts the scanline/component/old-RLE decoding
state machines from the Radiance decoder in `image` 0.25.10
([source](https://docs.rs/crate/image/0.25.10/source/src/codecs/hdr/decoder.rs)),
MIT OR Apache-2.0. Metadata parsing still uses that library. The adapter makes row
work cooperative and adds checked run lengths, zero-run rejection, width validation,
header/input/radiance bounds and cancellation. Original licence texts are retained
in THIRD-PARTY-LICENSES.txt; no images or artwork from that project are copied.

## File-format crates

| Crate | Used for | Licence |
|---|---|---|
| gltf 1.4.1, gltf-json 1.4.1, gltf-derive 1.4.1 (https://github.com/gltf-rs/gltf) | glTF 2.0 import (crates/io/src/gltf_in.rs): the JSON document model and its reference checks; the GLB container, buffers and accessors are read by Solidsmith's own code | MIT OR Apache-2.0 |
| inflections 1.1.1 (build-time, through gltf-derive) | case conversion in gltf's derive macros | MIT |
| lazy_static 1.5.1 (through gltf-json) | | MIT OR Apache-2.0 |

The glTF test files are built by the tests themselves (crates/io/src/gltf_in_tests.rs,
crates/engine/src/tests/gltf_import.rs); no third-party models are committed.

## Geometry kernel crates

| Crate | Used for | Licence |
|---|---|---|
| solidcore (https://github.com/tangency-labs/solidcore, pinned by rev in Cargo.toml) | the Rust port of NVIDIA's SMLib: the B-rep kernel behind crates/smlib | Apache-2.0 (its NOTICE credits NVIDIA's USD BRep) |

Linked as a crate; no code from it is copied into Solidsmith. Its licence text and NOTICE go
into THIRD-PARTY-LICENSES.txt when `main` regenerates it.

## FreeCAD and Rhino import library

FreeCAD `.FCStd` and Rhino `.3dm` files are read with [cadmpeg](https://github.com/cadmpeg/cadmpeg)
0.6.7 (Apache-2.0), linked as the crates `cadmpeg-core`, `cadmpeg-ir`, `cadmpeg-registry`,
`cadmpeg-container`, `cadmpeg-codec-freecad`, `cadmpeg-codec-rhino` and `cadmpeg-codec-step`
in native and browser builds (`crates/io/src/native_cad.rs`). Only those open-format codecs are built: no
default features, so cadmpeg's codecs for commercial CAD formats and its ShapeManager/ACIS
codec are not compiled in. Its licence text is in THIRD-PARTY-LICENSES.txt; no code from it is
copied into Solidsmith.

## Recovered contributions

| Solidsmith file | Original contribution | Licence |
|---|---|---|
| crates/ui-egui/src/viewport.rs (`nearest_quarter_turn`, `axis_view_orbit`) | N3MI-DG's [PR #101](https://github.com/bherbruck/solidsmith/pull/101): [minimal TOP/BOTTOM turns](https://github.com/bherbruck/solidsmith/commit/c28e780f52490784ef5886f094b9d3f6d10c3e3f) and [drag-to-orbit](https://github.com/bherbruck/solidsmith/commit/f30b30be6ecc4e2e6544a851243137c01a354934), adapted to the axis gizmo | MIT OR Apache-2.0 |
| crates/smlib/src/core/offset.rs (`draft`: the tilted planes and `SmBrep::LocalOperation`, the tail of the capi's `draft_faces`) | ablasik's `draft` in `crates/smlib/src/ffi/rs_ops2.rs` on [ablasik/solidsmith `app/solidcore-backend-on-develop`](https://github.com/ablasik/solidsmith/blob/9c7e8e31565e1823f0591b58e2967351c32af13f/crates/smlib/src/ffi/rs_ops2.rs), adapted to `core::offset`'s errors and checks | MIT OR Apache-2.0 |

## Vendored third-party code

| Directory | Source | Licence | Changes |
|---|---|---|---|
| vendor/accesskit_macos | accesskit_macos 0.26.3, https://github.com/AccessKit/accesskit (The AccessKit contributors) | MIT OR Apache-2.0 | the subclassing adapter and its tests from 0.27.2: dropping the adapter keeps AppKit's key-value observers (vendor/README.md) |

## Vendored renderer compatibility fix

`vendor/egui-wgpu` is egui-wgpu 0.36.2 from https://github.com/emilk/egui
(MIT OR Apache-2.0; both original licence texts are included). Rust sources and
its original `egui.wgsl` shader are retained; no images or other artwork are copied.
Solidsmith's narrow patch selects a safe 1x MSAA path on the chosen OpenGL adapter
and shares that effective count with the surface attachments and CAD callbacks.
Vulkan, DirectX 12, Metal and WebGPU retain their requested multisampling.
A frame whose meshes the GPU refuses to stage (device lost or out of memory) is
skipped with a warning instead of panicking (#1469).
Paint callbacks are prepared only after a drawable is acquired, so a skipped
surface frame cannot strand cached meshes without their GPU initialization.

## Runtime studio lighting mathematics

`crates/gpu/src/path/path.wgsl` implements the power-heuristic estimator from the
mathematical description in [PBRT fourth edition, section 13.4](https://pbr-book.org/4ed/Light_Transport_I_Surface_Reflection/A_Better_Path_Tracer).
No PBRT code or assets were copied. `crates/render/src/job/lighting.rs` evaluates
[Krzysztof Narkowicz's published 2016 ACES rational fit](https://knarkowicz.wordpress.com/2016/01/06/aces-filmic-tone-mapping-curve/)
using its published coefficients (author licenses the fit under CC0/MIT).
These original implementations are under Solidsmith's MIT OR Apache-2.0 licence.

## Original coffee framing variant

| File | Author | Provenance | Licence |
|---|---|---|---|
| examples/studio/author_coffee_framing.py | Solidsmith contributors | original editable composition, camera and documentation; no external assets | MIT OR Apache-2.0 |
| examples/studio/coffee-framing.json | Solidsmith contributors | original editable composition, camera and documentation; no external assets | MIT OR Apache-2.0 |
| examples/studio/coffee-framing-sweep.json | Solidsmith contributors | original editable composition, camera and documentation; no external assets | MIT OR Apache-2.0 |
| examples/studio/coffee-framing-render.json | Solidsmith contributors | original editable composition, camera and documentation; no external assets | MIT OR Apache-2.0 |
| examples/studio/coffee-framing.md | Solidsmith contributors | original editable composition, camera and documentation; no external assets | MIT OR Apache-2.0 |

## Original cylindrical coffee housing variant

| File | Author | Provenance | Licence |
|---|---|---|---|
| examples/studio/author_coffee_cylinder.py | Solidsmith contributors | original parametric cylindrical CAD construction and editable command recipes; no external assets | MIT OR Apache-2.0 |
| examples/studio/coffee-cylinder.json | Solidsmith contributors | original cylindrical housing on the original exploded coffee composition; no external assets | MIT OR Apache-2.0 |
| examples/studio/coffee-cylinder-sweep.json | Solidsmith contributors | original cylindrical housing and continuous CAD cyclorama; no external assets | MIT OR Apache-2.0 |
| examples/studio/coffee-cylinder-render.json | Solidsmith contributors | original camera, studio lighting and optical settings retained from the framing variant | MIT OR Apache-2.0 |
| examples/studio/coffee-cylinder.md | Solidsmith contributors | original dimensional recipe, provenance and validation limitations | MIT OR Apache-2.0 |

## Runtime environment sampling

`crates/render/src/lighting/environment.rs` and the environment/sun portions of
`crates/gpu/src/path/path.wgsl` are original Solidsmith implementations of
solid-angle importance sampling and MIS, informed by the mathematical description
in [PBRT fourth edition, sections 12.5](https://pbr-book.org/4ed/Light_Sources/Infinite_Area_Lights)
and [13.4](https://pbr-book.org/4ed/Light_Transport_I_Surface_Reflection/A_Better_Path_Tracer).
The procedural gradient and inline RGBE test fixtures are original mathematical
colours, not downloaded imagery. No third-party environment images or shader code
are included. Licence: MIT OR Apache-2.0.

## Original tumbler showcase assets

The CAD room, tumblers, window opening, wood/weave patterns and RGBE sky below
are original Solidsmith contributor work. No third-party scene, photograph, HDRI,
texture, or AI-generated artwork was used. Python authoring sources are included.
All listed files use the repository MIT OR Apache-2.0 licence.

| File | Source / author | Use | License |
| --- | --- | --- | --- |
| examples/studio/author_showcase.py | Original Solidsmith contributors | Editable scene / runtime resource / reproducible authoring metadata | MIT OR Apache-2.0 |
| examples/studio/author_tumblers.py | Original Solidsmith contributors | Editable scene / runtime resource / reproducible authoring metadata | MIT OR Apache-2.0 |
| examples/studio/textures/ATTRIBUTION.md | Original Solidsmith contributors | Editable scene / runtime resource / reproducible authoring metadata | MIT OR Apache-2.0 |
| examples/studio/textures/author_textures.py | Original Solidsmith contributors | Editable scene / runtime resource / reproducible authoring metadata | MIT OR Apache-2.0 |
| examples/studio/textures/environment-manifest.json | Original Solidsmith contributors | Editable scene / runtime resource / reproducible authoring metadata | MIT OR Apache-2.0 |
| examples/studio/textures/manifest.json | Original Solidsmith contributors | Editable scene / runtime resource / reproducible authoring metadata | MIT OR Apache-2.0 |
| examples/studio/tumblers-hdr-textures-blocked-render.json | Original Solidsmith contributors | Editable scene / runtime resource / reproducible authoring metadata | MIT OR Apache-2.0 |
| examples/studio/tumblers-hdr-textures-render.json | Original Solidsmith contributors | Editable scene / runtime resource / reproducible authoring metadata | MIT OR Apache-2.0 |
| examples/studio/tumblers-materials.json | Original Solidsmith contributors | Editable scene / runtime resource / reproducible authoring metadata | MIT OR Apache-2.0 |
| examples/studio/tumblers-render.json | Original Solidsmith contributors | Editable scene / runtime resource / reproducible authoring metadata | MIT OR Apache-2.0 |
| examples/studio/tumblers-sun-render.json | Original Solidsmith contributors | Editable scene / runtime resource / reproducible authoring metadata | MIT OR Apache-2.0 |
| examples/studio/tumblers-sun-textures-blocked-render.json | Original Solidsmith contributors | Editable scene / runtime resource / reproducible authoring metadata | MIT OR Apache-2.0 |
| examples/studio/tumblers-sun-textures-render.json | Original Solidsmith contributors | Editable scene / runtime resource / reproducible authoring metadata | MIT OR Apache-2.0 |
| examples/studio/tumblers-sun.json | Original Solidsmith contributors | Editable scene / runtime resource / reproducible authoring metadata | MIT OR Apache-2.0 |
| examples/studio/tumblers-textures-blocked-render.json | Original Solidsmith contributors | Editable scene / runtime resource / reproducible authoring metadata | MIT OR Apache-2.0 |
| examples/studio/tumblers-textures-render.json | Original Solidsmith contributors | Editable scene / runtime resource / reproducible authoring metadata | MIT OR Apache-2.0 |
| examples/studio/tumblers.json | Original Solidsmith contributors | Editable scene / runtime resource / reproducible authoring metadata | MIT OR Apache-2.0 |

The original studio generators produce local ignored PNG maps and an RGBE gradient.
Generated image bytes are not source assets; their authorship, licences and expected
hashes are documented under `examples/studio/textures`. Rendered evidence belongs
on the results branch. No photographs, scans or external image assets are included.

## Additional render and showcase resources

| File | Author | Source | Licence |
|---|---|---|---|
| assets/render/starter/tools/check_all.py | Solidsmith contributors | original read-only authored/native resource validation orchestration and evidence receipts | MIT OR Apache-2.0 |
| assets/render/starter/tools/test_check_all.py | Solidsmith contributors | original validation-runner regression fixtures and subprocess tests | MIT OR Apache-2.0 |
| assets/render/starter/tools/requirements.txt | Solidsmith contributors | pinned offline authoring tool requirements; no application dependency changes | MIT OR Apache-2.0 |
| crates/render/examples/validate_starter.rs | Solidsmith contributors | original native resource conformance checker using public Solidsmith decoding, normalization, sampling and mip APIs | MIT OR Apache-2.0 |
| .gitattributes | Solidsmith contributors | original canonical line-ending policy for the existing runtime icon-discovery manifest; no artwork or icon-assignment changes | MIT OR Apache-2.0 |
| assets/render/.gitattributes | Solidsmith contributors | original canonical line-ending policy for SHA-256-pinned renderer authoring resources | MIT OR Apache-2.0 |
| docs/render-materials/.gitattributes | Solidsmith contributors | original canonical line-ending policy for SHA-256-pinned renderer authoring resources | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/assembly-bench-soft.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/automotive-turntable.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/black-metal-strips.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/cnc-overhead.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/esd-bench-neutral.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/fdm-neutral-inspection.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/garage-door.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/hangar-skylight.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/home-workbench.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/makerspace-worklights.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/mechanical-bay.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/neutral-comparison.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/neutral-side-softbox.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/paint-inspection-bands.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/printed-part-photo.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/rework-offset-strips.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/robot-led-editorial.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/showroom-ribbons.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/spacecraft-clean-bay.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/sun-free-overcast.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/vacuum-black.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/warm-table-window.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/white-enveloping-studio.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/exports/woodshop-window.hdr | Solidsmith contributors | original analytic relative-radiance HDR export; recipe, linear-sRGB primaries and SHA-256 in assets/render/starter/environments/manifest.json; not measured photometry | MIT OR Apache-2.0 |
| assets/render/starter/environments/manifest.json | Solidsmith contributors | original clean-room renderer resource recipe or manifest; authored/unbound scope and source receipts in assets/render/starter/README.md | MIT OR Apache-2.0 |
| assets/render/starter/environments/source/recipes.json | Solidsmith contributors | original clean-room renderer resource recipe or manifest; authored/unbound scope and source receipts in assets/render/starter/README.md | MIT OR Apache-2.0 |
| assets/render/starter/lighting/exports/rigs.json | Solidsmith contributors | original clean-room renderer resource recipe or manifest; authored/unbound scope and source receipts in assets/render/starter/README.md | MIT OR Apache-2.0 |
| assets/render/starter/lighting/recipe.json | Solidsmith contributors | original clean-room renderer resource recipe or manifest; authored/unbound scope and source receipts in assets/render/starter/README.md | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-001/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-001/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-001/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-001/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-001/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-002/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-002/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-002/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-002/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-002/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-003/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-003/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-003/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-003/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-003/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-004/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-004/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-004/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-004/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-004/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-005/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-005/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-005/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-005/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-005/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-006/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-006/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-006/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-006/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-006/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-007/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-007/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-007/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-007/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-007/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-008/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-008/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-008/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-008/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-008/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-009/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-009/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-009/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-009/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-009/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-010/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-010/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-010/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-010/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-010/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-011/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-011/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-011/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-011/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-011/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-012/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-012/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-012/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-012/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-012/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-013/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-013/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-013/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-013/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-013/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-014/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-014/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-014/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-014/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-014/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-015/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-015/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-015/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-015/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-015/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-016/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-016/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-016/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-016/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-016/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-017/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-017/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-017/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-017/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-017/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-018/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-018/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-018/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-018/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-018/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-019/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-019/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-019/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-019/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-019/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-020/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-020/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-020/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-020/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-020/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-021/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-021/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-021/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-021/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-021/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-022/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-022/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-022/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-022/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-022/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-023/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-023/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-023/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-023/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-023/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-024/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-024/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-024/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-024/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-024/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-025/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-025/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-025/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-025/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-025/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-026/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-026/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-026/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-026/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-026/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-027/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-027/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-027/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-027/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-027/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-028/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-028/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-028/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-028/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-028/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-029/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-029/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-029/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-029/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-029/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-030/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-030/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-030/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-030/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-030/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-031/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-031/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-031/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-031/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-031/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-032/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-032/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-032/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-032/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-032/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-033/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-033/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-033/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-033/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-033/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-034/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-034/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-034/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-034/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-034/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-035/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-035/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-035/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-035/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-035/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-036/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-036/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-036/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-036/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-036/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-037/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-037/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-037/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-037/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-037/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-038/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-038/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-038/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-038/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-038/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-039/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-039/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-039/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-039/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-039/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-040/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-040/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-040/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-040/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-040/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-041/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-041/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-041/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-041/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-041/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-042/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-042/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-042/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-042/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-042/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-043/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-043/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-043/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-043/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-043/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-044/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-044/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-044/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-044/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-044/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-045/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-045/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-045/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-045/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-045/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-046/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-046/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-046/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-046/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-046/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-047/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-047/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-047/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-047/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-047/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-048/base_color.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-048/height.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-048/metallic.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-048/normal_open_gl.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/exports/SURF-048/roughness.png | Solidsmith contributors | original deterministic periodic PBR map export; recipe, role, scale and SHA-256 in assets/render/starter/materials/manifest.json; not measured material data | MIT OR Apache-2.0 |
| assets/render/starter/materials/manifest.json | Solidsmith contributors | original clean-room renderer resource recipe or manifest; authored/unbound scope and source receipts in assets/render/starter/README.md | MIT OR Apache-2.0 |
| assets/render/starter/materials/source/recipes.json | Solidsmith contributors | original clean-room renderer resource recipe or manifest; authored/unbound scope and source receipts in assets/render/starter/README.md | MIT OR Apache-2.0 |
| assets/render/starter/pack.json | Solidsmith contributors | original clean-room renderer resource recipe or manifest; authored/unbound scope and source receipts in assets/render/starter/README.md | MIT OR Apache-2.0 |
| assets/render/starter/tools/environments.py | Solidsmith contributors | original clean-room offline resource authoring, numerical verification or validation tool; no application runtime dependency | MIT OR Apache-2.0 |
| assets/render/starter/tools/lighting.py | Solidsmith contributors | original clean-room offline resource authoring, numerical verification or validation tool; no application runtime dependency | MIT OR Apache-2.0 |
| assets/render/starter/tools/materials.py | Solidsmith contributors | original clean-room offline resource authoring, numerical verification or validation tool; no application runtime dependency | MIT OR Apache-2.0 |
| assets/render/starter/tools/test_environments.py | Solidsmith contributors | original clean-room offline resource authoring, numerical verification or validation tool; no application runtime dependency | MIT OR Apache-2.0 |
| assets/render/starter/tools/test_lighting.py | Solidsmith contributors | original clean-room offline resource authoring, numerical verification or validation tool; no application runtime dependency | MIT OR Apache-2.0 |
| assets/render/starter/tools/test_materials.py | Solidsmith contributors | original clean-room offline resource authoring, numerical verification or validation tool; no application runtime dependency | MIT OR Apache-2.0 |
| assets/render/starter/tools/validate_pack.py | Solidsmith contributors | original clean-room offline resource authoring, numerical verification or validation tool; no application runtime dependency | MIT OR Apache-2.0 |
| assets/render/selene/licenses/UFL.txt | Canonical Ltd. | exact Ubuntu Font Licence 1.0 notice retained from https://github.com/0xlillith/solidsmith-rover/blob/b32437915372d32b71419dc8f5143fe22ff1e869/UFL.txt for the modelled Ubuntu Light lettering | Ubuntu Font Licence 1.0 |
| assets/render/selene/materials/appearance-pack.json | Solidsmith contributors | original clean-room English appearance authoring data: 48 illustrative opaque PBR projections with separate advanced target specifications; no measured dataset or external assets | MIT OR Apache-2.0 |
| assets/render/selene/materials/appearance-swatches.json | Solidsmith contributors | original analytic command script for 48 opaque box swatches and a support base, derived only from the adjacent original appearance pack; no external artwork or engineering properties | MIT OR Apache-2.0 |
| assets/render/selene/models/SELENE-functional.solidsmith | Solidsmith contributors (native enhancements); 0xlillith (original rover); SychPL / Mateusz Sych (supplied showcase assembly); Canonical Ltd. (lettering outlines) | native-command derivative of the unchanged author-supplied source, with original bearing, suspension, gripper, electronics and presentation work; source and final hashes in assets/render/selene/SOURCE.md and VALIDATION.md | Model and showcase contributions: MIT OR Apache-2.0 (explicit SychPL grant https://github.com/tangency-labs/solidcraft/pull/5494#issuecomment-6375788360); lettering: Ubuntu Font Licence 1.0, notice retained in assets/render/selene/licenses/UFL.txt |
| assets/render/selene/motion/interference-pairs.json | Solidsmith contributors | original authored exact-body pair selection for bounded native clearance checks on the functional SELENE derivative; not a generated collision result | MIT OR Apache-2.0 |
| assets/render/selene/motion/mechanism-acceptance.json | Solidsmith contributors | original kinematic acceptance criteria, inspected source-container bindings and analytic planning checks from cited public Solidsmith documentation and the admitted native source; derivative runtime acceptance is recorded separately | MIT OR Apache-2.0 |
| assets/render/selene/motion/preflight-commands.json | Solidsmith contributors | original read-only inventory script using existing project commands; no source-model selectors or state mutations | MIT OR Apache-2.0 |
| assets/render/selene/motion/sample-sequence.json | Solidsmith contributors | original future lunar sampling and sealed-vial dynamics state specification; no simulation or physical calibration result | MIT OR Apache-2.0 |
| assets/render/selene/scenes/roles.json | Solidsmith contributors | original semantic role and candidate-material authoring specification for the prepared SELENE demo; no native geometry or external assets bundled | MIT OR Apache-2.0 |
| assets/render/selene/scenes/shots.json | Solidsmith contributors | original shot, framing and staged-sequence authoring specification for the prepared SELENE demo; no render or simulation result claimed | MIT OR Apache-2.0 |
| assets/render/selene/scenes/stages.json | Solidsmith contributors | original stage composition and light-ownership authoring specification derived from the project render catalog; scientific context linked in scenes/README.md | MIT OR Apache-2.0 |
| assets/render/selene/source-lock.json | Solidsmith contributors | original source-admission metadata and public-reference inventory pinning the supplied improved SELENE native file; original-model and font notices preserved in SOURCE.md | MIT OR Apache-2.0 |
| assets/render/selene/source/SELENE-showcase.solidsmith | 0xlillith (original rover); SychPL / Mateusz Sych (showcase assembly and helpers); Solidsmith contributors (emblem); Canonical Ltd. (lettering outlines) | unchanged author-published native file from https://github.com/SychPL/solidsmith/blob/32ec1720447e3fcc6bb6cc53a63fb5b39cfa21b9/SELENE-showcase.solidsmith; source hash and license evidence in assets/render/selene/SOURCE.md | Model and showcase contributions: MIT OR Apache-2.0 (explicit SychPL grant https://github.com/tangency-labs/solidcraft/pull/5494#issuecomment-6375788360); lettering: Ubuntu Font Licence 1.0, notice retained in assets/render/selene/licenses/UFL.txt |
| assets/render/selene/tools/bearings.py | Solidsmith contributors | original native-command authoring of grooved bearings, rolling-element kinematics, clamp hubs and arm/head clearances; analytic geometry, no copied CAD source | MIT OR Apache-2.0 |
| assets/render/selene/tools/build.py | Solidsmith contributors | original Python development tool for pinned-source native command replay, staged checkpoints, command-trace and geometry health checks; no application runtime dependency | MIT OR Apache-2.0 |
| assets/render/selene/tools/electronics.py | Solidsmith contributors | original native-command Raspberry Pi 5 visual reconstruction using public manufacturer mechanical dimensions; original package envelopes and connector internals, no manufacturer CAD or artwork copied | MIT OR Apache-2.0 |
| assets/render/selene/tools/export_views.py | Solidsmith contributors | original native MCP preview export tool for three reusable showcase views; changes only transient visibility and cameras and verifies the source master hash | MIT OR Apache-2.0 |
| assets/render/selene/tools/finishes.py | Solidsmith contributors | original native physical-material proxy and opaque PBR assignment generator preserving the supplied rover palette | MIT OR Apache-2.0 |
| assets/render/selene/tools/mechanisms.py | Solidsmith contributors | original native-command gripper sleeves, pads, wheel ownership, joint limits and inspection studies with independent analytic suspension relation | MIT OR Apache-2.0 |
| assets/render/selene/tools/prepare.py | Solidsmith contributors | original Python development tool for bounded native-container inspection and reviewed opaque body-assignment preparation; no application runtime dependency | MIT OR Apache-2.0 |
| assets/render/selene/tools/presentation.py | Solidsmith contributors | original native-command browser organization, camera/render setups and closed sample-vial glass preparation | MIT OR Apache-2.0 |
| assets/render/selene/tools/suspension.py | Solidsmith contributors | original native-command trailing-arm suspension, telescoping coilover loops, plain bushings, carrier gussets and local motion clearances derived from the admitted rover geometry; independent analytic pin-distance relation, no copied external CAD | MIT OR Apache-2.0 |
| assets/render/selene/tools/validate_native.py | Solidsmith contributors | original bounded native MCP verification tool for sampled SELENE kinematics, independent analytic follower and world-coordinate checks, optional exact-name interference pairs and unchanged input hashes; no application runtime dependency | MIT OR Apache-2.0 |
| assets/render/selene/tools/validate_prepared.py | Solidsmith contributors | original Python development validator for prepared SELENE resources, source status, references, hashes and command projections; no application runtime dependency | MIT OR Apache-2.0 |
| assets/render/selene/tools/yaw_supports.py | Solidsmith contributors | original native-command plain yaw journals, flanged brass thrust bushings, recessed axial retaining pins and mounting-head clearances derived from the admitted rover geometry; no copied external CAD | MIT OR Apache-2.0 |
| assets/render/selene/ui/demo-flow.json | Solidsmith contributors | original progressive-disclosure UI integration specification for the existing Render workspace; no copied UI artwork or runtime registration | MIT OR Apache-2.0 |
| docs/render-materials/baseline-setup-commands.json | Solidsmith contributors | original clean-room English catalog, preset or analytic planning data; external references documented in docs/render-materials/SOURCES.md; no external assets bundled | MIT OR Apache-2.0 |
| docs/render-materials/capabilities.json | Solidsmith contributors | original clean-room English catalog, preset or analytic planning data; external references documented in docs/render-materials/SOURCES.md; no external assets bundled | MIT OR Apache-2.0 |
| docs/render-materials/environment-presets.json | Solidsmith contributors | original clean-room English catalog, preset or analytic planning data; external references documented in docs/render-materials/SOURCES.md; no external assets bundled | MIT OR Apache-2.0 |
| docs/render-materials/environments.json | Solidsmith contributors | original clean-room English catalog, preset or analytic planning data; external references documented in docs/render-materials/SOURCES.md; no external assets bundled | MIT OR Apache-2.0 |
| docs/render-materials/lighting.json | Solidsmith contributors | original clean-room English catalog, preset or analytic planning data; external references documented in docs/render-materials/SOURCES.md; no external assets bundled | MIT OR Apache-2.0 |
| docs/render-materials/lunar-physics-fixtures.json | Solidsmith contributors | original clean-room English catalog, preset or analytic planning data; external references documented in docs/render-materials/SOURCES.md; no external assets bundled | MIT OR Apache-2.0 |
| docs/render-materials/materials.json | Solidsmith contributors | original clean-room English catalog, preset or analytic planning data; external references documented in docs/render-materials/SOURCES.md; no external assets bundled | MIT OR Apache-2.0 |
| docs/render-materials/provenance.json | Solidsmith contributors | original clean-room English catalog, preset or analytic planning data; external references documented in docs/render-materials/SOURCES.md; no external assets bundled | MIT OR Apache-2.0 |
| docs/render-materials/quality-presets.json | Solidsmith contributors | original clean-room English catalog, preset or analytic planning data; external references documented in docs/render-materials/SOURCES.md; no external assets bundled | MIT OR Apache-2.0 |
| docs/render-materials/roadmap.json | Solidsmith contributors | original clean-room English catalog, preset or analytic planning data; external references documented in docs/render-materials/SOURCES.md; no external assets bundled | MIT OR Apache-2.0 |
| docs/render-materials/scene-presets.json | Solidsmith contributors | original clean-room English catalog, preset or analytic planning data; external references documented in docs/render-materials/SOURCES.md; no external assets bundled | MIT OR Apache-2.0 |
| docs/render-materials/selene-bindings.json | Solidsmith contributors | original clean-room English catalog, preset or analytic planning data; external references documented in docs/render-materials/SOURCES.md; no external assets bundled | MIT OR Apache-2.0 |
| docs/render-materials/simulation-requirements.json | Solidsmith contributors | original clean-room English catalog, preset or analytic planning data; external references documented in docs/render-materials/SOURCES.md; no external assets bundled | MIT OR Apache-2.0 |
| docs/render-materials/studio-presets.json | Solidsmith contributors | original clean-room English catalog, preset or analytic planning data; external references documented in docs/render-materials/SOURCES.md; no external assets bundled | MIT OR Apache-2.0 |
| docs/render-materials/ui-integration.json | Solidsmith contributors | original English UI integration specification and exact source-revision mappings from Solidsmith's own implemented shell, UI kit and accepted icons; no copied artwork or runtime registration | MIT OR Apache-2.0 |
| docs/render-materials/ux.json | Solidsmith contributors | original clean-room English catalog, preset or analytic planning data; external references documented in docs/render-materials/SOURCES.md; no external assets bundled | MIT OR Apache-2.0 |
| docs/render-materials/validate_catalog.py | Solidsmith contributors | original Python development validator for catalog references, preset composition and analytic specification fixtures; no application runtime dependency | MIT OR Apache-2.0 |

### Built-in render resource adapters (personal integration batch, 2026-10-22)

The original adapters in `crates/engine/src/cmd/render/library.rs`, the generated lookup `library_data.rs` and its generator `tools/render-library/generate.py` project the already attributed starter and SELENE resources without replacing their bytes or authoring manifests. The added scalar texture channels, explicit color/V conventions and immutable-source/GPU-payload sharing are original contributor work under MIT OR Apache-2.0. No external code, commercial CAD data or new third-party imagery was imported. Source metadata and the original authoring attribution remain authoritative; pending runtime acceptance is not a photometric or engineering qualification.

## Physical reference resource additions

| File | Author | Source | Licence |
|---|---|---|---|
| assets/render/physical/bindings.json | Solidsmith contributors | original physical-reference curation, associations or derived schema-1 export; per-fact public sources retained without relicensing their publications | MIT OR Apache-2.0 |
| assets/render/physical/exports/coverage.json | Solidsmith contributors | original physical-reference curation, associations or derived schema-1 export; per-fact public sources retained without relicensing their publications | MIT OR Apache-2.0 |
| assets/render/physical/exports/native-cards.json | Solidsmith contributors | original physical-reference curation, associations or derived schema-1 export; per-fact public sources retained without relicensing their publications | MIT OR Apache-2.0 |
| assets/render/physical/profiles.json | Solidsmith contributors | original physical-reference curation, associations or derived schema-1 export; per-fact public sources retained without relicensing their publications | MIT OR Apache-2.0 |
| assets/render/physical/quantities.json | Solidsmith contributors | original physical-reference curation, associations or derived schema-1 export; per-fact public sources retained without relicensing their publications | MIT OR Apache-2.0 |
| assets/render/physical/sources.json | Solidsmith contributors | original physical-reference curation, associations or derived schema-1 export; per-fact public sources retained without relicensing their publications | MIT OR Apache-2.0 |
| assets/render/physical/tools/check_physical.py | Solidsmith contributors | original offline physical-reference validation and projection tool | MIT OR Apache-2.0 |
| assets/render/physical/tools/physical.py | Solidsmith contributors | original offline physical-reference validation and projection tool | MIT OR Apache-2.0 |
| assets/render/physical/tools/test_physical.py | Solidsmith contributors | original offline physical-reference validation and projection tool | MIT OR Apache-2.0 |
| assets/render/physical/tools/export_library.py | Solidsmith contributors | original explicit reference-library planner and native no-loss export verifier; no external code | MIT OR Apache-2.0 |
| assets/render/physical/tools/test_export_library.py | Solidsmith contributors | original regressions for native capability refusal, source identity and physical-card preservation | MIT OR Apache-2.0 |

| tools/render-library/selene_studio.py | Solidsmith contributors | original bounded native MCP studio orchestration with pinned editable master, explicit GPU policy and request/export receipts; no generated model or image substitutes | MIT OR Apache-2.0 |

| tools/render-library/selene_recipe.py | Solidsmith contributors | original full-detail SELENE material/light composition and analytic camera fit; unchanged pinned native master, no external imagery or measured optical dataset | MIT OR Apache-2.0 |
| tools/render-library/test_selene_recipe.py | Solidsmith contributors | original independent projection, scaling, nonmutation and input-refusal fixtures for the SELENE studio recipe | MIT OR Apache-2.0 |

| assets/render/selene/scenes/studio-hero.json | Solidsmith contributors | original executable render intent and verified provenance for the selected native SELENE studio showcase; authored RGB appearance and relative light units, not measured optics or simulation results | MIT OR Apache-2.0 |
| tools/render-library/publish_selene.py | Solidsmith contributors | original bounded native-frame publication checks for explicit visual review, source identity, resource hashes, PNG CRCs and scanline limits; copies completed native bytes without image synthesis or resampling | MIT OR Apache-2.0 |
| tools/render-library/test_publish_selene.py | Solidsmith contributors | original tiny analytic PNG fixtures for source-size, CRC, expansion, dimensions and explicit-publication-consent regressions | MIT OR Apache-2.0 |

| tools/render-library/package_selene.py | Solidsmith contributors | original byte-preserving packaging of the reviewed native SELENE PNG, public provenance and checksums; bounded resource/PNG verification, no fonts, executables or new render execution | MIT OR Apache-2.0 |
| .github/workflows/selene-showcase.yml | Solidsmith contributors | original read-only artifact-delivery workflow for the reviewed native SELENE export; existing pinned repository actions, no compilation, package installation, secret use or renderer claims | MIT OR Apache-2.0 |

| tools/render-library/verify_linear_master.py | Solidsmith contributors | original independent validation driver for native linear EXR output using the official OpenEXR Python reader as an external test oracle; no codec implementation copied or production dependency added | MIT OR Apache-2.0 |

## Aging, effects and backgrounds showcase, 2026-10-23

| File | Author | Source | Licence |
|---|---|---|---|
| tools/render-showcase/instrument.py | Solidsmith contributors | original native-scene authoring, checksum-verified editorial assembly or bounded delivery tool; no third-party source or font binary copied | MIT OR Apache-2.0 |
| tools/render-showcase/compose.py | Solidsmith contributors | original native-scene authoring, checksum-verified editorial assembly or bounded delivery tool; no third-party source or font binary copied | MIT OR Apache-2.0 |
| tools/render-showcase/package.py | Solidsmith contributors | original native-scene authoring, checksum-verified editorial assembly or bounded delivery tool; no third-party source or font binary copied | MIT OR Apache-2.0 |

| distribution/Solidsmith-Render-Showcase.zip | Solidsmith contributors and the explicitly credited original model/UI/resource authors | byte-verified selected collaborator package of reviewed native media, editable sample, recipes and public provenance; contained attribution and existing SELENE lettering notices retained; no fonts or executable binaries | MIT OR Apache-2.0 for new contributions; referenced original notices retained |
| distribution/Solidsmith-Render-Showcase.json | Solidsmith contributors | original verified delivery archive manifest with exact SHA-256, member count and bounded scope | MIT OR Apache-2.0 |
