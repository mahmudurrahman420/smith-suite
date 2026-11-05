# Agent skills

Skills for contributing to Solidsmith and modelling through its [MCP server](../docs/mcp.md). Each folder is
one skill in the [Agent Skills](https://agentskills.io) format: a `SKILL.md` with a name and a
description that tell the agent when to load it, and an optional supporting reference file.

The modelling documents are built into Solidsmith: its MCP server's `skill_reference` tool returns them, so an
agent without file access (such as the one in Solidsmith's own chat) can still read a skill's reference.
The contributor skills (everything except modeling, editing, checking and mechanisms) are for repository work and are not embedded in that tool. `AGENTS.md` routes to them; `cargo xtask skills` fails when its table and this folder disagree or a file, `cargo xtask` command or skill they name is gone.

| Skill | Loads when the agent |
|---|---|
| [`solidsmith-contributing`](solidsmith-contributing/SKILL.md) | helps someone set up a fork, choose and claim an issue, and submit a contribution for review; the rules every PR follows |
| [`solidsmith-ground-rules`](solidsmith-ground-rules/SKILL.md) | adds a dependency, asset or wording: independence, licences, attribution, the product name |
| [`solidsmith-crash-safety`](solidsmith-crash-safety/SKILL.md) | writes non-test Rust, handles hostile input or investigates a crash report |
| [`solidsmith-commands`](solidsmith-commands/SKILL.md) | adds a command or decides which crate code belongs in |
| [`solidsmith-ui`](solidsmith-ui/SKILL.md) | adds a dialog, panel, camera input, icon or colour |
| [`solidsmith-ui-text`](solidsmith-ui-text/SKILL.md) | adds a label, message or translation |
| [`solidsmith-file-formats`](solidsmith-file-formats/SKILL.md) | changes what a design saves, or touches import and export |
| [`solidsmith-preferences`](solidsmith-preferences/SKILL.md) | adds a setting or changes a default |
| [`solidsmith-tests`](solidsmith-tests/SKILL.md) | places or runs tests |
| [`solidsmith-performance`](solidsmith-performance/SKILL.md) | works on slow code, the window thread or the wasm build |
| [`solidsmith-packaging`](solidsmith-packaging/SKILL.md) | prepares a release or changes packages |
| [`solidsmith-agent-tools`](solidsmith-agent-tools/SKILL.md) | runs the app, drives the control channel or changes MCP tools |
| [`solidsmith-modeling`](solidsmith-modeling/SKILL.md) | builds a new part (enclosure, bracket, plate, flange, …) |
| [`solidsmith-editing`](solidsmith-editing/SKILL.md) | changes, fixes or extends an existing design |
| [`solidsmith-checking`](solidsmith-checking/SKILL.md) | builds, changes or ports a design, next to the two above: parameter ranges, every allowed value, exact STEP comparison, regression tests, designs kept in git |
| [`solidsmith-mechanisms`](solidsmith-mechanisms/SKILL.md) | builds, drives or animates an assembly: components, joints, motion links, closed loops such as actuators, contact sets, motion studies |
| [`solidsmith-reporting`](solidsmith-reporting/SKILL.md) | hears that something went wrong (a crash, a failed command, a wrong shape or number, a hang): reproduces it in a fresh design, shrinks it to a minimal script and writes the bug report |

The modelling skills teach the agent to:
- plan the part as numbers
- build it in a few `batch` calls with parametric positions
- verify every stage with `measure` against values it computed itself
- skip the large replies it doesn't need, such as the unfiltered command catalog and full topology dumps

They also list the traps that otherwise cost failed calls. Examples are the sketch axes of the XZ plane, edge picks that snap to the nearest edge, and cuts that miss but still report success.

The modelling skills are written for Solidsmith 0.8 and later; `solidsmith-mechanisms` for `main` after 0.11.1 (the next release), which counts the degrees of freedom of closed loops; `solidsmith-reporting` for 0.18 and later.

## Install

The modelling skills, for Claude Code in all projects:

```sh
git clone --depth 1 https://github.com/mahmudurrahman420/solidsmith
cp -r solidsmith/skills/solidsmith-{modeling,editing,checking,mechanisms} ~/.claude/skills/
```

Windows (PowerShell):

```powershell
git clone --depth 1 https://github.com/mahmudurrahman420/solidsmith
foreach ($s in 'modeling','editing','checking','mechanisms') { Copy-Item -Recurse solidsmith\skills\solidsmith-$s $HOME\.claude\skills\ }
```

- **One project only:** copy the folders into that project's `.claude/skills/` instead.
- **Other agents:** for agents that read the Agent Skills format, copy the folders into the skills directory their documentation names.
- **Updating:** copy the folders again after you update Solidsmith.
- **Contributing:** the contributor skills need no install. Tell your assistant: "I want to contribute to Solidsmith. Read https://github.com/mahmudurrahman420/solidsmith/blob/main/skills/solidsmith-contributing/SKILL.md and help me get started."

## Maintaining

- The worked example in `solidsmith-modeling/reference.md` runs in CI. `skill_reference_example_builds_as_documented` in `crates/mcp/src/tests.rs` builds it, checks the documented volume, changes a parameter and checks it again.
- The sketch-plane table is covered by `sketch_plane_axes_match_the_skill_reference` in the same file.
- The worked example in `solidsmith-mechanisms/reference.md` runs in CI too: `mechanisms_skill_example_moves_as_documented` builds it, drives the hinge and checks the actuator stroke, the linked panel and the stop against their analytic values.
- `skills_name_only_real_tools_and_commands` checks that every tool and command the skills call exists.
- `solidsmith-checking/scripts/tests` checks that skill's helpers, its traps and conventions, and the facts behind the corrections in `solidsmith-modeling`: `PYTHONDONTWRITEBYTECODE=1 uv run --no-project --with pytest --with build123d pytest -p no:cacheprovider skills/solidsmith-checking/scripts/tests`. The tests run the `solidsmith-cli` on `PATH`, not the workspace build, and CI does not run them.
- When a command, a parameter name or a behaviour that these files describe changes, update them in the same PR.
- Facts that describe current limitations, such as edge picks without a distance limit, should go once the limitation is fixed.
