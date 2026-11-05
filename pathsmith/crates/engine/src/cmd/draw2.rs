//! Commands backing the drawing tools (pencil, curvature, anchor tools, scissors, knife, eraser,
//! blob brush, smooth, path eraser, join).

use std::collections::BTreeSet;
use std::sync::Arc;

use kurbo::{CubicBez, ParamCurve, ParamCurveNearest};
use serde_json::{Value, json};
use pathsmith_color::{Color, Paint};
use pathsmith_doc::{AngleProfile, Appearance, Document, Node, NodeId, NodeKind, PressureProfile, Selection};
use pathsmith_geom::hit::fill_contains;
use pathsmith_geom::{Anchor, AnchorKind, BezPath, FillRule, PathData, Point, Rect, SubPath, Vec2};
use pathsmith_pathops as po;
use pathsmith_pathops::{BoolOp, Cap, Join, SimplifyOptions};
use pathsmith_tools::draw2::{eraser_area, eraser_tip};

use super::create::{add_look, insert_look};
use super::newart::NewArt;
use super::pathops::{Beside, remove_each};
use super::{
    CommandSpec, Result, Session, bad, check_numbers, cmd, f64_or, has_anchors, has_doc, has_selection, id_param, index, number_or, ok, str_param,
    switch_or,
};
use crate::EngineError;

mod anchors;
mod commands;
mod common;
mod cut;
mod freehand;
mod smooth;

pub use commands::specs;
pub(super) use common::{ids_json, replace_with_pieces, usize_opt};
pub(super) use cut::{components, erase_outline};
