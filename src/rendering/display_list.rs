use super::geometry::Point;
use paste::paste;

/// An object that encapsulates a sequence of rendering operations
/// that can be dispatched to the render backend later.
#[derive(Debug, Clone)]
pub struct DisplayList {
    ops: Vec<DisplayListOp>,
}

pub(crate) struct DisplayListRecorder<'d> {
    display_list: &'d mut DisplayList,
}

impl DisplayList {
    pub fn present<R>(&self, receiver: &mut R)
    where
        R: DisplayListOpReceiver,
    {
        for op in &self.ops {
            match op {
                DisplayListOp::MoveTo(op) => receiver.dispatch_move_to(*op),
                DisplayListOp::LineTo(op) => receiver.dispatch_line_to(*op),
                DisplayListOp::ArcTo(op) => receiver.dispatch_arc_to(*op),
                DisplayListOp::ClosePath(op) => receiver.dispatch_close_path(*op),
            }
        }
    }
}

impl DisplayList {
    pub(crate) fn new() -> Self {
        Self { ops: vec![] }
    }

    pub(crate) fn begin_recording(&mut self) -> DisplayListRecorder<'_> {
        DisplayListRecorder { display_list: self }
    }

    pub(crate) fn add_op(&mut self, op: DisplayListOp) {
        self.ops.push(op);
    }
}

macro_rules! impl_ops {
    ($($op_name:ident($op_fn_name:ident) { $($field:ident : $field_ty:ty),* }),*) => {
        $(
            #[derive(Debug, Copy, Clone)]
            pub struct $op_name {
                $(pub $field : $field_ty),*
            }
        )*

        #[derive(Debug, Copy, Clone)]
        pub enum DisplayListOp {
            $($op_name($op_name)),*
        }

        impl<'d> DisplayListRecorder<'d> {
            $(
                pub fn $op_fn_name(&mut self, $($field : $field_ty),*) {
                    let op = DisplayListOp::$op_name($op_name { $($field),* });
                    self.display_list.add_op(op);
                }
            )*
        }

        paste! {
            pub trait DisplayListOpReceiver {
                $(
                    fn [<dispatch_ $op_fn_name>](&mut self, op : $op_name);
                )*
            }
        }
    };
}

impl_ops!(
    MoveTo(move_to) { point: Point },
    LineTo(line_to) { point: Point },
    ArcTo(arc_to) { center: Point, radius: f64, start_angle: f32, end_angle: f32, clockwise: bool },
    ClosePath(close_path) { }
);
