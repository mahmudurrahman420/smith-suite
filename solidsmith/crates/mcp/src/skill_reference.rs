//! Read-only access to the skill documents shipped with Solidsmith.

use serde_json::{Map, Value};

struct Skill {
    name: &'static str,
    files: &'static [(&'static str, &'static str)],
}

// Exact names only: these are embedded documents, never filesystem paths to open.
const SKILLS: &[Skill] = &[
    Skill {
        name: "solidsmith-checking",
        files: &[
            ("SKILL.md", include_str!("../../../skills/solidsmith-checking/SKILL.md")),
            ("reference/porting.md", include_str!("../../../skills/solidsmith-checking/reference/porting.md")),
        ],
    },
    Skill { name: "solidsmith-editing", files: &[("SKILL.md", include_str!("../../../skills/solidsmith-editing/SKILL.md"))] },
    Skill {
        name: "solidsmith-mechanisms",
        files: &[
            ("SKILL.md", include_str!("../../../skills/solidsmith-mechanisms/SKILL.md")),
            ("reference.md", include_str!("../../../skills/solidsmith-mechanisms/reference.md")),
        ],
    },
    Skill {
        name: "solidsmith-modeling",
        files: &[
            ("SKILL.md", include_str!("../../../skills/solidsmith-modeling/SKILL.md")),
            ("reference.md", include_str!("../../../skills/solidsmith-modeling/reference.md")),
        ],
    },
];

pub(crate) fn read(args: &Map<String, Value>) -> Result<Value, String> {
    let name = args.get("skill").and_then(Value::as_str).ok_or("`skill` is required and must be a string")?;
    let skill = SKILLS
        .iter()
        .find(|s| s.name == name)
        .ok_or_else(|| format!("Unknown skill. Available skills: {}", SKILLS.iter().map(|s| s.name).collect::<Vec<_>>().join(", ")))?;
    let file = match args.get("file") {
        None | Some(Value::Null) => "reference.md",
        Some(Value::String(file)) => file,
        Some(_) => return Err("`file` must be a string".into()),
    };
    skill.files.iter().find(|(name, _)| *name == file).map(|(_, text)| Value::String((*text).into())).ok_or_else(|| {
        format!("Unknown file for {}. Available files: {}", skill.name, skill.files.iter().map(|(name, _)| *name).collect::<Vec<_>>().join(", "))
    })
}
