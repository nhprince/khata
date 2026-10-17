//! Preset shapes for Insert › Shapes: the gallery's catalogue and the geometry of each shape.
//!
//! Every shape is drawn here from plain geometry (polygons, arcs and a few parametric curves) in a
//! 100 × 100 space that is then stretched over the shape's frame. A preset the engine has no native
//! kind for becomes a [`Freeform`] with closed paths, so it renders, prints, exports and resizes like
//! any other shape. The gallery paints its thumbnails from the same geometry, so what you see in the
//! menu is what you get on the page.

use std::f32::consts::PI;

use crate::freeform::{FreePath, Freeform};

type Pt = [f32; 2];

/// The gallery's group headings, in order (Lines are handled by the UI: they are connectors).
pub const GROUPS: [&str; 7] = ["Rectangles", "Basic Shapes", "Block Arrows", "Equation Shapes", "Flowchart", "Stars and Banners", "Callouts"];

/// One entry of the gallery.
#[derive(Clone, Copy, Debug)]
pub struct Preset {
    /// The `preset` id (or, for a native shape, its `kind`).
    pub id: &'static str,
    pub label: &'static str,
    pub group: &'static str,
    /// A native kind of the engine (drawn natively, not as a freeform).
    pub native: bool,
    /// Inserted square (stars, faces) instead of wide.
    pub square: bool,
}

const fn p(id: &'static str, label: &'static str, group: &'static str, square: bool) -> Preset {
    Preset { id, label, group, native: false, square }
}
const fn n(id: &'static str, label: &'static str, group: &'static str, square: bool) -> Preset {
    Preset { id, label, group, native: true, square }
}

pub const PRESETS: &[Preset] = &[
    // Rectangles.
    n("rectangle", "Rectangle", "Rectangles", false),
    n("roundedRectangle", "Rounded Rectangle", "Rectangles", false),
    p("snip1Rect", "Snip Single Corner Rectangle", "Rectangles", false),
    p("snip2SameRect", "Snip Same Side Corner Rectangle", "Rectangles", false),
    p("round1Rect", "Round Single Corner Rectangle", "Rectangles", false),
    p("round2SameRect", "Round Same Side Corner Rectangle", "Rectangles", false),
    // Basic shapes.
    n("ellipse", "Oval", "Basic Shapes", false),
    n("triangle", "Isosceles Triangle", "Basic Shapes", false),
    n("rightTriangle", "Right Triangle", "Basic Shapes", false),
    n("diamond", "Diamond", "Basic Shapes", false),
    p("parallelogram", "Parallelogram", "Basic Shapes", false),
    p("trapezoid", "Trapezoid", "Basic Shapes", false),
    p("pentagon", "Regular Pentagon", "Basic Shapes", true),
    p("hexagon", "Hexagon", "Basic Shapes", false),
    p("heptagon", "Heptagon", "Basic Shapes", true),
    p("octagon", "Octagon", "Basic Shapes", true),
    p("decagon", "Decagon", "Basic Shapes", true),
    p("plus", "Cross", "Basic Shapes", true),
    p("lShape", "L-Shape", "Basic Shapes", true),
    p("teardrop", "Teardrop", "Basic Shapes", true),
    p("moon", "Moon", "Basic Shapes", true),
    p("sun", "Sun", "Basic Shapes", true),
    p("smiley", "Smiley Face", "Basic Shapes", true),
    p("lightning", "Lightning Bolt", "Basic Shapes", true),
    n("heart", "Heart", "Basic Shapes", true),
    p("cloud", "Cloud", "Basic Shapes", false),
    p("cube", "Cube", "Basic Shapes", true),
    p("can", "Cylinder", "Basic Shapes", true),
    p("bevel", "Bevel", "Basic Shapes", true),
    p("blockArc", "Block Arc", "Basic Shapes", false),
    p("arc", "Arc", "Basic Shapes", true),
    p("leftBracket", "Left Bracket", "Basic Shapes", false),
    p("rightBracket", "Right Bracket", "Basic Shapes", false),
    // Block arrows.
    p("rightArrow", "Arrow: Right", "Block Arrows", false),
    p("leftArrow", "Arrow: Left", "Block Arrows", false),
    p("upArrow", "Arrow: Up", "Block Arrows", true),
    p("downArrow", "Arrow: Down", "Block Arrows", true),
    p("leftRightArrow", "Arrow: Left-Right", "Block Arrows", false),
    p("upDownArrow", "Arrow: Up-Down", "Block Arrows", true),
    p("quadArrow", "Arrow: Quad", "Block Arrows", true),
    p("notchedRightArrow", "Arrow: Notched Right", "Block Arrows", false),
    p("homePlate", "Arrow: Pentagon", "Block Arrows", false),
    p("chevron", "Arrow: Chevron", "Block Arrows", false),
    // Equation shapes.
    p("mathPlus", "Plus Sign", "Equation Shapes", true),
    p("mathMinus", "Minus Sign", "Equation Shapes", false),
    p("mathMultiply", "Multiplication Sign", "Equation Shapes", true),
    p("mathDivide", "Division Sign", "Equation Shapes", true),
    p("mathEqual", "Equal Sign", "Equation Shapes", true),
    p("mathNotEqual", "Not Equal Sign", "Equation Shapes", true),
    // Flowchart.
    p("flowProcess", "Flowchart: Process", "Flowchart", false),
    p("flowAlternateProcess", "Flowchart: Alternate Process", "Flowchart", false),
    p("flowDecision", "Flowchart: Decision", "Flowchart", false),
    p("flowData", "Flowchart: Data", "Flowchart", false),
    p("flowPredefined", "Flowchart: Predefined Process", "Flowchart", false),
    p("flowDocument", "Flowchart: Document", "Flowchart", false),
    p("flowTerminator", "Flowchart: Terminator", "Flowchart", false),
    p("flowPreparation", "Flowchart: Preparation", "Flowchart", false),
    p("flowManualInput", "Flowchart: Manual Input", "Flowchart", false),
    p("flowConnector", "Flowchart: Connector", "Flowchart", true),
    p("flowOffpage", "Flowchart: Off-page Connector", "Flowchart", true),
    p("flowDelay", "Flowchart: Delay", "Flowchart", false),
    p("flowDisk", "Flowchart: Magnetic Disk", "Flowchart", true),
    p("flowMerge", "Flowchart: Merge", "Flowchart", true),
    p("flowSumming", "Flowchart: Summing Junction", "Flowchart", true),
    n("flowchartOr", "Flowchart: Or", "Flowchart", true),
    // Stars and banners.
    p("star4", "Star: 4 Points", "Stars and Banners", true),
    n("star", "Star: 5 Points", "Stars and Banners", true),
    p("star6", "Star: 6 Points", "Stars and Banners", true),
    p("star7", "Star: 7 Points", "Stars and Banners", true),
    p("star8", "Star: 8 Points", "Stars and Banners", true),
    p("star12", "Star: 12 Points", "Stars and Banners", true),
    p("star24", "Star: 24 Points", "Stars and Banners", true),
    p("burst", "Explosion", "Stars and Banners", true),
    p("wave", "Wave", "Stars and Banners", false),
    // Callouts.
    p("calloutRect", "Speech Bubble: Rectangle", "Callouts", false),
    p("calloutRound", "Speech Bubble: Rectangle with Corners Rounded", "Callouts", false),
    p("calloutOval", "Speech Bubble: Oval", "Callouts", false),
    p("calloutCloud", "Thought Bubble: Cloud", "Callouts", false),
    p("calloutLine", "Callout: Line", "Callouts", false),
];

/// The entry with this id.
pub fn find(id: &str) -> Option<&'static Preset> {
    PRESETS.iter().find(|p| p.id == id)
}

/// The size (points) a new preset shape gets.
pub fn default_size(id: &str) -> (f32, f32) {
    if find(id).is_some_and(|p| p.square) { (72.0, 72.0) } else { (108.0, 72.0) }
}

/// The preset as a freeform, ready to insert (`None` for an unknown id or a native kind).
pub fn freeform(id: &str) -> Option<Freeform> {
    if find(id).is_some_and(|p| p.native) {
        return None;
    }
    let paths = paths(id)?;
    Some(Freeform { w: 100.0, h: 100.0, paths, ..Default::default() }.sanitized())
}

/// The geometry of a preset (native kinds included, for the gallery's thumbnails), fitted to fill
/// the 100 × 100 space.
pub fn paths(id: &str) -> Option<Vec<FreePath>> {
    let mut v = raw(id)?;
    fit(&mut v);
    Some(v)
}

// ---------------------------------------------------------------------------------------------
// Geometry helpers.
// ---------------------------------------------------------------------------------------------

fn closed(pts: Vec<Pt>) -> FreePath {
    FreePath { pts, closed: true }
}
fn open(pts: Vec<Pt>) -> FreePath {
    FreePath { pts, closed: false }
}
fn poly(pts: &[Pt]) -> FreePath {
    closed(pts.to_vec())
}

/// Points along an arc of an ellipse; angles in degrees, 0° to the right, increasing clockwise on
/// the page. `a1` may be below `a0` to go the other way.
fn arc(cx: f32, cy: f32, rx: f32, ry: f32, a0: f32, a1: f32, steps: usize) -> Vec<Pt> {
    (0..=steps)
        .map(|i| {
            let a = (a0 + (a1 - a0) * i as f32 / steps as f32).to_radians();
            [cx + rx * a.cos(), cy + ry * a.sin()]
        })
        .collect()
}

fn circle(cx: f32, cy: f32, rx: f32, ry: f32) -> Vec<Pt> {
    let mut v = arc(cx, cy, rx, ry, 0.0, 360.0, 48);
    v.pop();
    v
}

/// A regular polygon in the 100 × 100 square; `rot` turns it (degrees from a point at the top).
fn regular(sides: usize, rot: f32) -> Vec<Pt> {
    (0..sides)
        .map(|i| {
            let a = (-90.0 + rot + 360.0 * i as f32 / sides as f32).to_radians();
            [50.0 + 50.0 * a.cos(), 50.0 + 50.0 * a.sin()]
        })
        .collect()
}

/// A star with `points` points whose inner radius is `ratio` of the outer.
fn star(points: usize, ratio: f32) -> Vec<Pt> {
    (0..points * 2)
        .map(|i| {
            let a = (-90.0 + 180.0 * i as f32 / points as f32).to_radians();
            let r = if i % 2 == 0 { 50.0 } else { 50.0 * ratio };
            [50.0 + r * a.cos(), 50.0 + r * a.sin()]
        })
        .collect()
}

/// The four corner arcs of a rounded rectangle, in order (top left, top right, bottom right, bottom left).
fn corners(x0: f32, y0: f32, x1: f32, y1: f32, r: f32) -> [Vec<Pt>; 4] {
    [
        arc(x0 + r, y0 + r, r, r, 180.0, 270.0, 6),
        arc(x1 - r, y0 + r, r, r, 270.0, 360.0, 6),
        arc(x1 - r, y1 - r, r, r, 0.0, 90.0, 6),
        arc(x0 + r, y1 - r, r, r, 90.0, 180.0, 6),
    ]
}

fn rrect(x0: f32, y0: f32, x1: f32, y1: f32, r: f32) -> Vec<Pt> {
    corners(x0, y0, x1, y1, r).into_iter().flatten().collect()
}

fn map(pts: &[Pt], f: impl Fn(Pt) -> Pt) -> Vec<Pt> {
    pts.iter().map(|&q| f(q)).collect()
}

/// A cloud outline: an ellipse with scalloped edges.
fn cloud_outline(cx: f32, cy: f32, rx: f32, ry: f32) -> Vec<Pt> {
    (0..96)
        .map(|i| {
            let a = 2.0 * PI * i as f32 / 96.0;
            let k = 0.84 + 0.16 * (3.0 * a).sin().abs();
            [cx + rx * k * a.cos(), cy + ry * k * a.sin()]
        })
        .collect()
}

const RIGHT_ARROW: [Pt; 7] = [[0.0, 30.0], [60.0, 30.0], [60.0, 0.0], [100.0, 50.0], [60.0, 100.0], [60.0, 70.0], [0.0, 70.0]];
const LEFT_RIGHT: [Pt; 10] =
    [[0.0, 50.0], [25.0, 0.0], [25.0, 30.0], [75.0, 30.0], [75.0, 0.0], [100.0, 50.0], [75.0, 100.0], [75.0, 70.0], [25.0, 70.0], [25.0, 100.0]];

/// A can: body with a curved bottom, plus the rim's ellipse.
fn can() -> Vec<FreePath> {
    let mut body = vec![[0.0, 15.0]];
    body.extend(arc(50.0, 85.0, 50.0, 15.0, 180.0, 0.0, 24));
    body.push([100.0, 15.0]);
    body.extend(arc(50.0, 15.0, 50.0, 15.0, 0.0, -180.0, 24));
    vec![closed(body), closed(circle(50.0, 15.0, 50.0, 15.0))]
}

fn raw(id: &str) -> Option<Vec<FreePath>> {
    Some(match id {
        // Rectangles.
        "rectangle" => vec![poly(&[[0.0, 0.0], [100.0, 0.0], [100.0, 100.0], [0.0, 100.0]])],
        "roundedRectangle" => vec![closed(rrect(0.0, 0.0, 100.0, 100.0, 18.0))],
        "snip1Rect" => vec![poly(&[[0.0, 0.0], [70.0, 0.0], [100.0, 30.0], [100.0, 100.0], [0.0, 100.0]])],
        "snip2SameRect" => vec![poly(&[[30.0, 0.0], [70.0, 0.0], [100.0, 30.0], [100.0, 100.0], [0.0, 100.0], [0.0, 30.0]])],
        "round1Rect" => {
            let mut v = vec![[0.0, 0.0], [72.0, 0.0]];
            v.extend(arc(72.0, 28.0, 28.0, 28.0, 270.0, 360.0, 8));
            v.extend([[100.0, 100.0], [0.0, 100.0]]);
            vec![closed(v)]
        }
        "round2SameRect" => {
            let mut v = arc(28.0, 28.0, 28.0, 28.0, 180.0, 270.0, 8);
            v.extend(arc(72.0, 28.0, 28.0, 28.0, 270.0, 360.0, 8));
            v.extend([[100.0, 100.0], [0.0, 100.0]]);
            vec![closed(v)]
        }
        // Basic shapes.
        "ellipse" => vec![closed(circle(50.0, 50.0, 50.0, 50.0))],
        "triangle" => vec![poly(&[[50.0, 0.0], [100.0, 100.0], [0.0, 100.0]])],
        "rightTriangle" => vec![poly(&[[0.0, 0.0], [100.0, 100.0], [0.0, 100.0]])],
        "diamond" => vec![poly(&[[50.0, 0.0], [100.0, 50.0], [50.0, 100.0], [0.0, 50.0]])],
        "parallelogram" | "flowData" => vec![poly(&[[25.0, 0.0], [100.0, 0.0], [75.0, 100.0], [0.0, 100.0]])],
        "trapezoid" => vec![poly(&[[25.0, 0.0], [75.0, 0.0], [100.0, 100.0], [0.0, 100.0]])],
        "pentagon" => vec![closed(regular(5, 0.0))],
        "hexagon" | "flowPreparation" => vec![poly(&[[25.0, 0.0], [75.0, 0.0], [100.0, 50.0], [75.0, 100.0], [25.0, 100.0], [0.0, 50.0]])],
        "heptagon" => vec![closed(regular(7, 0.0))],
        "octagon" => vec![closed(regular(8, 22.5))],
        "decagon" => vec![closed(regular(10, 0.0))],
        "plus" => vec![poly(&[
            [33.0, 0.0],
            [67.0, 0.0],
            [67.0, 33.0],
            [100.0, 33.0],
            [100.0, 67.0],
            [67.0, 67.0],
            [67.0, 100.0],
            [33.0, 100.0],
            [33.0, 67.0],
            [0.0, 67.0],
            [0.0, 33.0],
            [33.0, 33.0],
        ])],
        "lShape" => vec![poly(&[[0.0, 0.0], [40.0, 0.0], [40.0, 60.0], [100.0, 60.0], [100.0, 100.0], [0.0, 100.0]])],
        "teardrop" => {
            let mut v = vec![[100.0, 0.0]];
            v.extend(arc(50.0, 50.0, 50.0, 50.0, 0.0, 270.0, 36));
            vec![closed(v)]
        }
        "moon" => {
            let mut v = arc(70.0, 50.0, 50.0, 50.0, 280.0, 80.0, 40);
            v.extend(arc(100.0, 50.0, 53.7, 53.7, 113.4, 246.6, 30));
            vec![closed(v)]
        }
        "sun" => {
            let mut v = vec![closed(circle(50.0, 50.0, 27.0, 27.0))];
            for k in 0..8 {
                let a = (45.0 * k as f32).to_radians();
                let side = 10.0_f32.to_radians();
                let at = |ang: f32, r: f32| [50.0 + r * ang.cos(), 50.0 + r * ang.sin()];
                v.push(poly(&[at(a - side, 34.0), at(a, 50.0), at(a + side, 34.0)]));
            }
            v
        }
        "smiley" => vec![
            closed(circle(50.0, 50.0, 50.0, 50.0)),
            closed(circle(34.0, 38.0, 5.0, 7.0)),
            closed(circle(66.0, 38.0, 5.0, 7.0)),
            open(arc(50.0, 48.0, 30.0, 28.0, 25.0, 155.0, 24)),
        ],
        "lightning" => vec![poly(&[[55.0, 0.0], [15.0, 55.0], [45.0, 55.0], [30.0, 100.0], [85.0, 40.0], [55.0, 40.0], [75.0, 0.0]])],
        "heart" => vec![closed(
            (0..72)
                .map(|i| {
                    let t = 2.0 * PI * i as f32 / 72.0;
                    [16.0 * t.sin().powi(3), -(13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos())]
                })
                .collect(),
        )],
        "cloud" => vec![closed(cloud_outline(50.0, 50.0, 50.0, 34.0))],
        "cube" => vec![
            poly(&[[0.0, 25.0], [75.0, 25.0], [75.0, 100.0], [0.0, 100.0]]),
            poly(&[[0.0, 25.0], [25.0, 0.0], [100.0, 0.0], [75.0, 25.0]]),
            poly(&[[75.0, 25.0], [100.0, 0.0], [100.0, 75.0], [75.0, 100.0]]),
        ],
        "can" | "flowDisk" => can(),
        "bevel" => vec![
            poly(&[[0.0, 0.0], [100.0, 0.0], [100.0, 100.0], [0.0, 100.0]]),
            poly(&[[18.0, 18.0], [82.0, 18.0], [82.0, 82.0], [18.0, 82.0]]),
            open(vec![[0.0, 0.0], [18.0, 18.0]]),
            open(vec![[100.0, 0.0], [82.0, 18.0]]),
            open(vec![[100.0, 100.0], [82.0, 82.0]]),
            open(vec![[0.0, 100.0], [18.0, 82.0]]),
        ],
        "blockArc" => {
            let mut v = arc(50.0, 100.0, 50.0, 100.0, 180.0, 360.0, 36);
            v.extend(arc(50.0, 100.0, 26.0, 52.0, 360.0, 180.0, 28));
            vec![closed(v)]
        }
        "arc" => vec![open(arc(0.0, 100.0, 100.0, 100.0, 270.0, 360.0, 30))],
        "leftBracket" => vec![open(vec![[100.0, 0.0], [0.0, 0.0], [0.0, 100.0], [100.0, 100.0]])],
        "rightBracket" => vec![open(vec![[0.0, 0.0], [100.0, 0.0], [100.0, 100.0], [0.0, 100.0]])],
        // Block arrows.
        "rightArrow" => vec![poly(&RIGHT_ARROW)],
        "leftArrow" => vec![closed(map(&RIGHT_ARROW, |[x, y]| [100.0 - x, y]))],
        "upArrow" => vec![closed(map(&RIGHT_ARROW, |[x, y]| [y, 100.0 - x]))],
        "downArrow" => vec![closed(map(&RIGHT_ARROW, |[x, y]| [100.0 - y, x]))],
        "leftRightArrow" => vec![poly(&LEFT_RIGHT)],
        "upDownArrow" => vec![closed(map(&LEFT_RIGHT, |[x, y]| [y, x]))],
        "quadArrow" => vec![poly(&[
            [50.0, 0.0],
            [68.0, 20.0],
            [58.0, 20.0],
            [58.0, 42.0],
            [80.0, 42.0],
            [80.0, 32.0],
            [100.0, 50.0],
            [80.0, 68.0],
            [80.0, 58.0],
            [58.0, 58.0],
            [58.0, 80.0],
            [68.0, 80.0],
            [50.0, 100.0],
            [32.0, 80.0],
            [42.0, 80.0],
            [42.0, 58.0],
            [20.0, 58.0],
            [20.0, 68.0],
            [0.0, 50.0],
            [20.0, 32.0],
            [20.0, 42.0],
            [42.0, 42.0],
            [42.0, 20.0],
            [32.0, 20.0],
        ])],
        "notchedRightArrow" => {
            vec![poly(&[[0.0, 30.0], [60.0, 30.0], [60.0, 0.0], [100.0, 50.0], [60.0, 100.0], [60.0, 70.0], [0.0, 70.0], [15.0, 50.0]])]
        }
        "homePlate" => vec![poly(&[[0.0, 0.0], [70.0, 0.0], [100.0, 50.0], [70.0, 100.0], [0.0, 100.0]])],
        "chevron" => vec![poly(&[[0.0, 0.0], [70.0, 0.0], [100.0, 50.0], [70.0, 100.0], [0.0, 100.0], [30.0, 50.0]])],
        // Equation shapes.
        "mathPlus" => vec![poly(&[
            [38.0, 0.0],
            [62.0, 0.0],
            [62.0, 38.0],
            [100.0, 38.0],
            [100.0, 62.0],
            [62.0, 62.0],
            [62.0, 100.0],
            [38.0, 100.0],
            [38.0, 62.0],
            [0.0, 62.0],
            [0.0, 38.0],
            [38.0, 38.0],
        ])],
        "mathMinus" => vec![poly(&[[0.0, 35.0], [100.0, 35.0], [100.0, 65.0], [0.0, 65.0]])],
        "mathMultiply" => vec![poly(&[
            [15.0, 0.0],
            [50.0, 35.0],
            [85.0, 0.0],
            [100.0, 15.0],
            [65.0, 50.0],
            [100.0, 85.0],
            [85.0, 100.0],
            [50.0, 65.0],
            [15.0, 100.0],
            [0.0, 85.0],
            [35.0, 50.0],
            [0.0, 15.0],
        ])],
        "mathDivide" => vec![
            poly(&[[0.0, 40.0], [100.0, 40.0], [100.0, 60.0], [0.0, 60.0]]),
            closed(circle(50.0, 15.0, 11.0, 11.0)),
            closed(circle(50.0, 85.0, 11.0, 11.0)),
        ],
        "mathEqual" => {
            vec![poly(&[[0.0, 15.0], [100.0, 15.0], [100.0, 40.0], [0.0, 40.0]]), poly(&[[0.0, 60.0], [100.0, 60.0], [100.0, 85.0], [0.0, 85.0]])]
        }
        "mathNotEqual" => vec![
            poly(&[[0.0, 15.0], [100.0, 15.0], [100.0, 40.0], [0.0, 40.0]]),
            poly(&[[0.0, 60.0], [100.0, 60.0], [100.0, 85.0], [0.0, 85.0]]),
            poly(&[[62.0, 0.0], [82.0, 0.0], [38.0, 100.0], [18.0, 100.0]]),
        ],
        // Flowchart.
        "flowProcess" => vec![poly(&[[0.0, 0.0], [100.0, 0.0], [100.0, 100.0], [0.0, 100.0]])],
        "flowAlternateProcess" => vec![closed(rrect(0.0, 0.0, 100.0, 100.0, 20.0))],
        "flowDecision" => vec![poly(&[[50.0, 0.0], [100.0, 50.0], [50.0, 100.0], [0.0, 50.0]])],
        "flowPredefined" => vec![
            poly(&[[0.0, 0.0], [100.0, 0.0], [100.0, 100.0], [0.0, 100.0]]),
            open(vec![[12.0, 0.0], [12.0, 100.0]]),
            open(vec![[88.0, 0.0], [88.0, 100.0]]),
        ],
        "flowDocument" => {
            let mut v = vec![[0.0, 0.0], [100.0, 0.0]];
            v.extend((0..=24).map(|i| {
                let t = i as f32 / 24.0;
                [100.0 - 100.0 * t, 86.0 + 10.0 * (2.0 * PI * t).sin()]
            }));
            vec![closed(v)]
        }
        "flowTerminator" => vec![closed(rrect(0.0, 0.0, 100.0, 100.0, 50.0))],
        "flowManualInput" => vec![poly(&[[0.0, 20.0], [100.0, 0.0], [100.0, 100.0], [0.0, 100.0]])],
        "flowConnector" => vec![closed(circle(50.0, 50.0, 50.0, 50.0))],
        "flowOffpage" => vec![poly(&[[0.0, 0.0], [100.0, 0.0], [100.0, 65.0], [50.0, 100.0], [0.0, 65.0]])],
        "flowDelay" => {
            let mut v = vec![[0.0, 0.0], [50.0, 0.0]];
            v.extend(arc(50.0, 50.0, 50.0, 50.0, -90.0, 90.0, 24));
            v.push([0.0, 100.0]);
            vec![closed(v)]
        }
        "flowMerge" => vec![poly(&[[0.0, 0.0], [100.0, 0.0], [50.0, 100.0]])],
        "flowSumming" => vec![closed(circle(50.0, 50.0, 50.0, 50.0)), open(vec![[14.6, 14.6], [85.4, 85.4]]), open(vec![[85.4, 14.6], [14.6, 85.4]])],
        "flowchartOr" => vec![closed(circle(50.0, 50.0, 50.0, 50.0)), open(vec![[0.0, 50.0], [100.0, 50.0]]), open(vec![[50.0, 0.0], [50.0, 100.0]])],
        // Stars and banners.
        "star4" => vec![closed(star(4, 0.38))],
        "star" => vec![closed(star(5, 0.382))],
        "star6" => vec![closed(star(6, 0.58))],
        "star7" => vec![closed(star(7, 0.5))],
        "star8" => vec![closed(star(8, 0.62))],
        "star12" => vec![closed(star(12, 0.78))],
        "star24" => vec![closed(star(24, 0.88))],
        "burst" => vec![closed(
            (0..24)
                .map(|i| {
                    let a = (-90.0 + 15.0 * i as f32).to_radians();
                    let r = if i % 2 == 0 { 50.0 * (0.78 + 0.22 * (((i / 2) * 7) % 5) as f32 / 4.0) } else { 26.0 };
                    [50.0 + r * a.cos(), 50.0 + r * a.sin()]
                })
                .collect(),
        )],
        "wave" => {
            let mut v: Vec<Pt> = (0..=24)
                .map(|i| {
                    let t = i as f32 / 24.0;
                    [100.0 * t, 14.0 + 10.0 * (2.0 * PI * t).sin()]
                })
                .collect();
            v.extend((0..=24).rev().map(|i| {
                let t = i as f32 / 24.0;
                [100.0 * t, 86.0 + 10.0 * (2.0 * PI * t).sin()]
            }));
            vec![closed(v)]
        }
        // Callouts.
        "calloutRect" => vec![poly(&[[0.0, 0.0], [100.0, 0.0], [100.0, 75.0], [48.0, 75.0], [20.0, 100.0], [26.0, 75.0], [0.0, 75.0]])],
        "calloutRound" => {
            let [tl, tr, br, bl] = corners(0.0, 0.0, 100.0, 75.0, 12.0);
            let mut v = tl;
            v.extend(tr);
            v.extend(br);
            v.extend([[52.0, 75.0], [20.0, 100.0], [28.0, 75.0]]);
            v.extend(bl);
            vec![closed(v)]
        }
        "calloutOval" => {
            let mut v = arc(50.0, 40.0, 50.0, 40.0, 130.0, 455.0, 44);
            v.push([22.0, 100.0]);
            vec![closed(v)]
        }
        "calloutCloud" => {
            vec![closed(cloud_outline(50.0, 38.0, 50.0, 34.0)), closed(circle(20.0, 86.0, 7.0, 6.0)), closed(circle(8.0, 98.0, 4.0, 3.5))]
        }
        "calloutLine" => vec![poly(&[[30.0, 0.0], [100.0, 0.0], [100.0, 60.0], [30.0, 60.0]]), open(vec![[30.0, 45.0], [0.0, 100.0]])],
        _ => return None,
    })
}

/// Stretches the geometry to fill 0–100 on both axes (an axis with no extent is left alone).
fn fit(paths: &mut [FreePath]) {
    let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    for q in paths.iter().flat_map(|p| p.pts.iter()) {
        x0 = x0.min(q[0]);
        y0 = y0.min(q[1]);
        x1 = x1.max(q[0]);
        y1 = y1.max(q[1]);
    }
    let (sx, sy) = (if x1 - x0 > 1e-3 { 100.0 / (x1 - x0) } else { 1.0 }, if y1 - y0 > 1e-3 { 100.0 / (y1 - y0) } else { 1.0 });
    for p in paths.iter_mut() {
        for q in &mut p.pts {
            *q = [(q[0] - x0) * sx, (q[1] - y0) * sy];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_catalogue_entry_has_geometry_in_the_unit_box() {
        for p in PRESETS {
            let paths = paths(p.id).unwrap_or_else(|| panic!("{} has no geometry", p.id));
            assert!(!paths.is_empty(), "{}", p.id);
            for q in paths.iter().flat_map(|path| path.pts.iter()) {
                assert!(q[0] >= -0.01 && q[0] <= 100.01 && q[1] >= -0.01 && q[1] <= 100.01, "{}: {q:?}", p.id);
            }
        }
    }

    #[test]
    fn ids_are_unique_and_groups_are_known() {
        for (i, p) in PRESETS.iter().enumerate() {
            assert!(PRESETS[..i].iter().all(|o| o.id != p.id), "duplicate {}", p.id);
            assert!(GROUPS.contains(&p.group), "{}", p.group);
        }
    }

    #[test]
    fn native_kinds_are_not_freeforms_and_presets_are() {
        assert!(freeform("rectangle").is_none());
        let f = freeform("star6").expect("star6");
        assert!(f.paths.iter().any(|p| p.closed));
        assert!(freeform("nope").is_none());
    }
}
