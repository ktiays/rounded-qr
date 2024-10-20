use core::f32;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use bitflags::bitflags;
use itertools::iproduct;
use qrcodegen::QrCode;

use crate::rendering::display_list::DisplayListRecorder;
use crate::rendering::geometry::{Point, Size};

type Id = usize;

/// A type that represents the edges of a block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Unit {
    x: i32,
    y: i32,
    value: UnitEdge,
    union_id: Id,
}

impl Unit {
    fn left(&self) -> Segment {
        Segment {
            start: (self.x, self.y + 1),
            end: (self.x, self.y),
        }
    }

    fn top(&self) -> Segment {
        Segment {
            start: (self.x, self.y),
            end: (self.x + 1, self.y),
        }
    }

    fn right(&self) -> Segment {
        Segment {
            start: (self.x + 1, self.y),
            end: (self.x + 1, self.y + 1),
        }
    }

    fn bottom(&self) -> Segment {
        Segment {
            start: (self.x + 1, self.y + 1),
            end: (self.x, self.y + 1),
        }
    }

    fn segment(&self, edge: UnitEdge) -> Segment {
        if edge == UnitEdge::LEFT {
            self.left()
        } else if edge == UnitEdge::TOP {
            self.top()
        } else if edge == UnitEdge::RIGHT {
            self.right()
        } else if edge == UnitEdge::BOTTOM {
            self.bottom()
        } else {
            panic!("Invalid parameter: {:?}", edge);
        }
    }

    /// Returns a boolean value that indicates whether the edges
    /// of this block contain the specified point.
    fn contains(&self, point: (i32, i32)) -> bool {
        if point == (self.x, self.y) {
            self.value.contains(UnitEdge::TOP) || self.value.contains(UnitEdge::LEFT)
        } else if point == (self.x + 1, self.y) {
            self.value.contains(UnitEdge::TOP) || self.value.contains(UnitEdge::RIGHT)
        } else if point == (self.x, self.y + 1) {
            self.value.contains(UnitEdge::BOTTOM) || self.value.contains(UnitEdge::LEFT)
        } else if point == (self.x + 1, self.y + 1) {
            self.value.contains(UnitEdge::BOTTOM) || self.value.contains(UnitEdge::RIGHT)
        } else {
            false
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct UnitEdge: u8 {
        const TOP = 1;
        const RIGHT = 1 << 1;
        const BOTTOM = 1 << 2;
        const LEFT = 1 << 3;
    }
}

struct EdgeIterator {
    start: u8,
    count: u8,
}

impl EdgeIterator {
    pub fn start_with(edge: UnitEdge) -> Self {
        Self {
            start: edge.bits(),
            count: 0,
        }
    }
}

impl Iterator for EdgeIterator {
    type Item = UnitEdge;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 4 {
            return None;
        }
        self.count += 1;

        let edge = UnitEdge::from_bits(self.start);
        let mut next = self.start << 1;
        if next >= 0x10 {
            next = 1;
        }
        self.start = next;
        edge
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Segment {
    pub start: (i32, i32),
    pub end: (i32, i32),
}

impl Segment {
    pub fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    pub fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    pub fn is_collinear(&self, other: &Self) -> bool {
        if self.is_horizontal() && other.is_horizontal() {
            self.start.1 == other.start.1
        } else if self.is_vertical() && other.is_vertical() {
            self.start.0 == other.start.0
        } else {
            false
        }
    }

    pub fn is_clockwise(a: &Segment, b: &Segment) -> bool {
        if a.start.0 == a.end.0 {
            // Vertical
            (b.end.0 - b.start.0) * (a.end.1 - a.start.1).signum() < 0
        } else {
            // Horizontal
            (b.end.1 - b.start.1) * (a.end.0 - a.start.0).signum() > 0
        }
    }
}

/// Draws the given QR Code.
pub(crate) fn draw(code: QrCode, size: Size, recorder: &mut DisplayListRecorder) {
    let mut unit_map = HashMap::new();
    let mut unions = HashMap::new();

    for (x, y) in iproduct!(0..code.size(), 0..code.size()) {
        if !code.get_module(x, y) {
            continue;
        }

        let union_id = unit_map.len();
        let unit = Unit {
            x,
            y,
            value: UnitEdge::all(),
            union_id,
        };
        unit_map.insert((x, y), unit);
        unions.insert(union_id, vec![(x, y)].into_iter().collect::<HashSet<_>>());
    }

    for (x, y) in iproduct!(0..code.size(), 0..code.size()) {
        let Some(current) = unit_map.get(&(x, y)).cloned() else {
            continue;
        };
        let current_union_id = current.union_id;

        let mut removed_edge_for_current = UnitEdge::empty();
        let mut unify_unit = |unit_id| {
            let unit = unit_map.get_mut(&unit_id);
            if let Some(unit) = unit {
                if unit_id.0 > current.x {
                    unit.value.remove(UnitEdge::LEFT);
                    removed_edge_for_current = removed_edge_for_current.union(UnitEdge::RIGHT);
                } else {
                    unit.value.remove(UnitEdge::TOP);
                    removed_edge_for_current = removed_edge_for_current.union(UnitEdge::BOTTOM);
                }

                let near_union_id = unit.union_id;
                if current_union_id == near_union_id {
                    return;
                }

                let right_units = unions
                    .get(&near_union_id)
                    .expect("should get units")
                    .clone();
                unions.remove(&near_union_id);
                unions
                    .get_mut(&current_union_id)
                    .expect("should get units")
                    .extend(right_units.iter());

                for id in right_units {
                    let Some(u) = unit_map.get_mut(&id) else {
                        continue;
                    };
                    u.union_id = current_union_id;
                }
            }
        };

        unify_unit((x + 1, y));
        unify_unit((x, y + 1));

        let current = unit_map.get_mut(&(x, y)).expect("should get the unit");
        current.value.remove(removed_edge_for_current);
    }
    unions.iter_mut().for_each(|(_, v)| {
        v.retain(|v| {
            let unit = unit_map.get(v).expect("should get corresponding unit");
            !unit.value.is_empty()
        });
    });

    for (_, unit_ids) in unions {
        let mut complete = HashSet::new();
        let mut path: Vec<Segment> = vec![];
        let min_x = *unit_ids
            .iter()
            .min_by_key(|v| v.0)
            .expect("should have values");
        let mut next = *unit_ids
            .iter()
            .filter(|v| v.0 == min_x.0)
            .min_by_key(|v| v.1)
            .expect("should have value");
        let mut start_edge = UnitEdge::TOP;

        loop {
            let unit = unit_map.get_mut(&next).expect("should get unit");

            let mut last = path.last().cloned();

            if let Some(last) = last {
                let point = last.end;
                start_edge = if point == (unit.x + 1, unit.y) {
                    UnitEdge::RIGHT
                } else if point == (unit.x, unit.y + 1) {
                    UnitEdge::LEFT
                } else if point == (unit.x + 1, unit.y + 1) {
                    UnitEdge::BOTTOM
                } else {
                    UnitEdge::TOP
                }
            }

            let mut stop = false;
            for edge in EdgeIterator::start_with(start_edge) {
                if !unit.value.contains(edge) {
                    if stop {
                        break;
                    } else {
                        continue;
                    }
                }
                stop = true;
                unit.value.remove(edge);

                let seg = unit.segment(edge);
                if let Some(mut l) = last {
                    if l.end == seg.start && l.is_collinear(&seg) {
                        l.end = seg.end;
                        path.pop();
                        path.push(l);
                        continue;
                    }
                }
                last = None;
                path.push(seg);
            }

            if unit.value.is_empty() {
                complete.insert(next);
            }

            if complete.len() == unit_ids.len() {
                break;
            }

            if let Some(n) = unit_ids.iter().find(|u| {
                let unit = unit_map.get(u).expect("should get unit");
                **u != next
                    && unit.contains(path.last().expect("should have at least one segment").end)
                    && !complete.contains(u)
            }) {
                // Find the unit that intersect with the current end path.
                next = *n;
            } else {
                // If there are no intersecting paths, randomly select one from the units that have never been merged.
                let n = unit_ids
                    .iter()
                    .find(|u| !complete.contains(u))
                    .expect("should find the unit id");
                next = *n;
            }
        }

        #[derive(Debug, Default, Clone, Copy)]
        struct DrawAdj {
            start_offset: f64,
            end_offset: f64,
            corner_radius: f64,

            /// A signum representation of the direction of the corner.
            corner_direction: (f64, f64),
            clockwise: bool,
        }

        // Add rounded corners to the path.
        let mut corners = HashMap::new();
        let mut closed_path_start_idx = None;
        for idx in 0..path.len() + 1 {
            let current_idx = idx % path.len();
            let mut next_idx = (idx + 1) % path.len();

            let a = &path[current_idx];
            let mut b = &path[next_idx];

            if a.end != b.start {
                if let Some(path_start_idx) = closed_path_start_idx {
                    b = &path[path_start_idx];
                    next_idx = path_start_idx;
                    closed_path_start_idx = None;
                } else {
                    continue;
                }
            } else if closed_path_start_idx.is_none() {
                closed_path_start_idx = Some(current_idx);
            }

            if a.is_collinear(b) {
                continue;
            }

            let is_clockwise = Segment::is_clockwise(a, b);

            let a_adj = corners.entry(current_idx).or_insert(DrawAdj::default());
            let radius = if is_clockwise { 0.5 } else { 0.25 };
            a_adj.end_offset = radius;
            a_adj.corner_radius = radius;
            a_adj.corner_direction = (
                (b.end.0 - a.start.0).signum() as f64,
                (b.end.1 - a.start.1).signum() as f64,
            );
            a_adj.clockwise = is_clockwise;

            let b_adj = corners.entry(next_idx).or_insert(DrawAdj::default());
            b_adj.start_offset = radius;
        }

        let draw_unit = size.width.min(size.height) / code.size() as f64;
        let mut end = None;
        for (idx, seg) in path.iter().enumerate() {
            let adj = corners.get(&idx).cloned().unwrap_or_default();
            let start_offset = (
                (seg.end.0 - seg.start.0).signum() as f64 * adj.start_offset,
                (seg.end.1 - seg.start.1).signum() as f64 * adj.start_offset,
            );
            let end_offset = (
                (seg.start.0 - seg.end.0).signum() as f64 * adj.end_offset,
                (seg.start.1 - seg.end.1).signum() as f64 * adj.end_offset,
            );

            let new_start = (
                seg.start.0 as f64 + start_offset.0,
                seg.start.1 as f64 + start_offset.1,
            );
            let new_end = (
                seg.end.0 as f64 + end_offset.0,
                seg.end.1 as f64 + end_offset.1,
            );

            if end.map(|end| end != seg.start).unwrap_or(true) {
                recorder.move_to(Point::new(new_start.0 * draw_unit, new_start.1 * draw_unit));
            }
            if new_start != new_end {
                recorder.line_to(Point::new(new_end.0 * draw_unit, new_end.1 * draw_unit));
            }

            let corner_radius = adj.corner_radius;
            if adj.corner_radius > 0_f64 {
                let is_clockwise = adj.clockwise;
                let (x_direction, y_direction) = adj.corner_direction;
                let is_axis_congruence = (x_direction * y_direction) > 0_f64;
                let arc_start = Point::new(new_end.0, new_end.1);
                let arc_end = Point::new(
                    new_end.0 + corner_radius * x_direction,
                    new_end.1 + corner_radius * y_direction,
                );
                let center = if is_clockwise {
                    if is_axis_congruence {
                        Point::new(arc_start.x, arc_end.y)
                    } else {
                        Point::new(arc_end.x, arc_start.y)
                    }
                } else if is_axis_congruence {
                    Point::new(arc_end.x, arc_start.y)
                } else {
                    Point::new(arc_start.x, arc_end.y)
                };
                let start_angle = if arc_start.x == center.x {
                    if arc_start.y < center.y {
                        f32::consts::PI * 3_f32 / 2_f32
                    } else {
                        f32::consts::PI / 2_f32
                    }
                } else if arc_start.x < center.x {
                    f32::consts::PI
                } else {
                    0_f32
                };
                let end_angle = start_angle
                    + f32::consts::PI / 2_f32 * if is_clockwise { 1_f32 } else { -1_f32 };
                recorder.arc_to(
                    center * draw_unit,
                    corner_radius * draw_unit,
                    start_angle,
                    end_angle,
                    is_clockwise,
                );
            }

            end = Some(seg.end);
        }
        recorder.close_path();
    }
}
