//! Insert › Shapes: a gallery of drawn thumbnails grouped under headings (Recently Used Shapes,
//! Lines, Rectangles, Basic Shapes, Block Arrows, Equation Shapes, Flowchart, Stars and Banners,
//! Callouts), instead of a plain text list. The thumbnails are painted from the same geometry the
//! shapes are inserted with (`khata_doc::shape_presets`), so the menu shows what the page gets.

use egui::{Rect, Sense, Shape, Stroke, Ui, pos2, vec2};
use khata_doc::shape_presets as presets;
use serde_json::json;

use crate::WordApp;
use crate::theme::{Tokens, semibold};

const CELL: f32 = 30.0;
const COLUMNS: usize = 10;
/// How many recently used shapes the gallery remembers.
pub const RECENT_MAX: usize = 10;

/// What choosing an entry does.
#[derive(Clone, Copy)]
enum Action {
    /// A native shape kind of the engine.
    Native(&'static str),
    /// A preset the engine draws as a freeform.
    Preset(&'static str),
    /// A line or connector: its kind and arrowheads.
    Line(&'static str, &'static str),
}

#[derive(Clone, Copy)]
struct Entry {
    id: &'static str,
    label: &'static str,
    action: Action,
}

/// The lines and connectors (the same commands the old text menu had).
const LINES: [Entry; 7] = [
    Entry { id: "line", label: "Line", action: Action::Line("line", "none") },
    Entry { id: "straightConnector", label: "Straight Connector", action: Action::Line("straightConnector", "none") },
    Entry { id: "straightArrowConnector", label: "Straight Arrow Connector", action: Action::Line("straightConnector", "end") },
    Entry { id: "elbowConnector", label: "Elbow Connector", action: Action::Line("elbowConnector", "none") },
    Entry { id: "elbowArrowConnector", label: "Elbow Arrow Connector", action: Action::Line("elbowConnector", "end") },
    Entry { id: "curvedConnector", label: "Curved Connector", action: Action::Line("curvedConnector", "none") },
    Entry { id: "curvedArrowConnector", label: "Curved Arrow Connector", action: Action::Line("curvedConnector", "end") },
];

fn preset_entry(p: &'static presets::Preset) -> Entry {
    Entry { id: p.id, label: p.label, action: if p.native { Action::Native(p.id) } else { Action::Preset(p.id) } }
}

fn find(id: &str) -> Option<Entry> {
    LINES.iter().copied().find(|e| e.id == id).or_else(|| presets::find(id).map(preset_entry))
}

/// A thumbnail's geometry in the 0–100 box: (points, closed).
fn thumbnail(entry: &Entry) -> Vec<(Vec<[f32; 2]>, bool)> {
    match entry.action {
        Action::Line(kind, arrow) => {
            let (mut paths, tip, back): (Vec<Vec<[f32; 2]>>, [f32; 2], [f32; 2]) = match kind {
                "elbowConnector" => (vec![vec![[4.0, 96.0], [4.0, 50.0], [96.0, 50.0], [96.0, 10.0]]], [96.0, 4.0], [96.0, 24.0]),
                "curvedConnector" => {
                    let pts = (0..=24)
                        .map(|i| {
                            let t = i as f32 / 24.0;
                            let u = 1.0 - t;
                            let (p0, p1, p2, p3) = ([4.0_f32, 96.0_f32], [4.0_f32, 36.0_f32], [96.0_f32, 64.0_f32], [96.0_f32, 6.0_f32]);
                            [
                                u * u * u * p0[0] + 3.0 * u * u * t * p1[0] + 3.0 * u * t * t * p2[0] + t * t * t * p3[0],
                                u * u * u * p0[1] + 3.0 * u * u * t * p1[1] + 3.0 * u * t * t * p2[1] + t * t * t * p3[1],
                            ]
                        })
                        .collect();
                    (vec![pts], [96.0, 4.0], [96.0, 26.0])
                }
                _ => (vec![vec![[4.0, 96.0], [96.0, 4.0]]], [96.0, 4.0], [76.0, 14.0]),
            };
            let mut out: Vec<(Vec<[f32; 2]>, bool)> = paths.drain(..).map(|p| (p, false)).collect();
            if arrow == "end" {
                // A small arrowhead at the tip, pointing along the line's last direction.
                let (dx, dy) = (tip[0] - back[0], tip[1] - back[1]);
                let len = (dx * dx + dy * dy).sqrt().max(1.0);
                let (ux, uy) = (dx / len, dy / len);
                let base = [tip[0] - ux * 16.0, tip[1] - uy * 16.0];
                let (nx, ny) = (-uy * 8.0, ux * 8.0);
                out.push((vec![tip, [base[0] + nx, base[1] + ny], [base[0] - nx, base[1] - ny]], true));
            }
            out
        }
        _ => presets::paths(entry.id).unwrap_or_default().into_iter().map(|p| (p.pts, p.closed)).collect(),
    }
}

fn paint_thumbnail(ui: &Ui, rect: Rect, entry: &Entry, color: egui::Color32) {
    let pad = 6.0;
    let inner = rect.shrink(pad);
    let to = |p: [f32; 2]| pos2(inner.min.x + p[0] / 100.0 * inner.width(), inner.min.y + p[1] / 100.0 * inner.height());
    let stroke = Stroke::new(1.3, color);
    for (pts, is_closed) in thumbnail(entry) {
        let pts: Vec<_> = pts.into_iter().map(to).collect();
        if pts.len() < 2 {
            continue;
        }
        ui.painter().add(if is_closed { Shape::closed_line(pts, stroke) } else { Shape::line(pts, stroke) });
    }
}

fn heading(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(4.0);
    ui.label(egui::RichText::new(tl!(text)).font(semibold(12.5)).color(t.text));
    ui.add_space(2.0);
}

/// One grid of entries. Returns the entry the person chose.
fn grid(ui: &mut Ui, entries: &[Entry]) -> Option<Entry> {
    let t = Tokens::get(ui.ctx());
    let mut chosen = None;
    ui.spacing_mut().item_spacing = vec2(2.0, 2.0);
    for row in entries.chunks(COLUMNS) {
        ui.horizontal(|ui| {
            for entry in row {
                let (r, resp) = ui.allocate_exact_size(vec2(CELL, CELL), Sense::click());
                let resp = resp.on_hover_text(tl!(entry.label));
                if resp.hovered() {
                    ui.painter().rect(r, 5.0, t.hover, Stroke::new(1.5, t.ink), egui::StrokeKind::Inside);
                }
                paint_thumbnail(ui, r, entry, t.icon);
                if resp.clicked() {
                    chosen = Some(*entry);
                }
            }
        });
    }
    chosen
}

/// The Shapes menu's contents.
pub fn menu(ui: &mut Ui, app: &mut WordApp) {
    ui.set_min_width(COLUMNS as f32 * (CELL + 2.0) + 12.0);
    let mut chosen: Option<Entry> = None;
    egui::ScrollArea::vertical().max_height(440.0).auto_shrink([false, true]).show(ui, |ui| {
        let recent: Vec<Entry> = app.ui.recent_shapes.iter().filter_map(|id| find(id)).collect();
        if !recent.is_empty() {
            heading(ui, "Recently Used Shapes");
            chosen = chosen.or(grid(ui, &recent));
        }
        heading(ui, "Lines");
        chosen = chosen.or(grid(ui, &LINES));
        for group in presets::GROUPS {
            let entries: Vec<Entry> = presets::PRESETS.iter().filter(|p| p.group == group).map(preset_entry).collect();
            if entries.is_empty() {
                continue;
            }
            heading(ui, group);
            chosen = chosen.or(grid(ui, &entries));
        }
    });
    ui.separator();
    if ui.button(tl!("New Drawing Canvas")).clicked() {
        let _ = app.run("insert.canvas", json!({}));
        ui.close();
    }
    if let Some(entry) = chosen {
        let args = match entry.action {
            Action::Native(kind) => json!({"kind": kind}),
            Action::Preset(id) => json!({"preset": id}),
            Action::Line(kind, arrow) => json!({"kind": kind, "arrow": arrow}),
        };
        if app.run("insert.shape", args).is_ok() {
            app.ui.recent_shapes.retain(|id| id != entry.id);
            app.ui.recent_shapes.insert(0, entry.id.to_string());
            app.ui.recent_shapes.truncate(RECENT_MAX);
        }
        ui.close();
    }
}
