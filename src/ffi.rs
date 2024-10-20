use std::ffi::{c_float, c_int, c_void};

use crate::builder::Builder;
use crate::rendering::display_list::{ArcTo, ClosePath, DisplayListOpReceiver, LineTo, MoveTo};
use crate::types::{ErrorCorrectionLevel, Size};

#[no_mangle]
pub extern "C" fn rqr_builder_create_with_data(
    data: *const u8,
    length: usize,
    ecl: c_int,
    width: c_float,
    height: c_float,
) -> *mut c_void {
    let bytes = unsafe { std::slice::from_raw_parts(data, length) };
    let ecl = match ecl {
        1 => ErrorCorrectionLevel::Medium,
        2 => ErrorCorrectionLevel::Quartile,
        3 => ErrorCorrectionLevel::High,
        _ => ErrorCorrectionLevel::Low,
    };
    let builder = Box::new(
        Builder::binary(bytes)
            .error_correction_level(ecl)
            .size(Size::new(width as f32, height as f32)),
    );
    Box::into_raw(builder) as *mut c_void
}

#[no_mangle]
pub extern "C" fn rqr_builder_build_path(
    builder: *mut c_void,
    context: *mut c_void,
    move_to_point_fn: extern "C" fn(*mut c_void, c_float, c_float),
    line_to_point_fn: extern "C" fn(*mut c_void, c_float, c_float),
    arc_to_fn: extern "C" fn(*mut c_void, c_float, c_float, c_float, c_float, c_float, c_int),
    close_path_fn: extern "C" fn(*mut c_void),
) {
    let builder = unsafe { Box::<Builder>::from_raw(builder as *mut _) };

    struct ExternReceiver {
        context: *mut c_void,
        move_to_point_fn: extern "C" fn(*mut c_void, c_float, c_float),
        line_to_point_fn: extern "C" fn(*mut c_void, c_float, c_float),
        arc_to_fn: extern "C" fn(*mut c_void, c_float, c_float, c_float, c_float, c_float, c_int),
        close_path_fn: extern "C" fn(*mut c_void),
    }

    impl DisplayListOpReceiver for ExternReceiver {
        fn dispatch_move_to(&mut self, op: MoveTo) {
            (self.move_to_point_fn)(self.context, op.point.x as c_float, op.point.y as c_float);
        }

        fn dispatch_line_to(&mut self, op: LineTo) {
            (self.line_to_point_fn)(self.context, op.point.x as c_float, op.point.y as c_float);
        }

        fn dispatch_arc_to(&mut self, op: ArcTo) {
            (self.arc_to_fn)(
                self.context,
                op.center.x as c_float,
                op.center.y as c_float,
                op.radius as c_float,
                op.start_angle as c_float,
                op.end_angle as c_float,
                op.clockwise as c_int,
            );
        }

        fn dispatch_close_path(&mut self, _op: ClosePath) {
            (self.close_path_fn)(self.context);
        }
    }

    let mut receiver = ExternReceiver {
        context,
        move_to_point_fn,
        line_to_point_fn,
        arc_to_fn,
        close_path_fn,
    };
    _ = builder.build_with_receiver(&mut receiver);
}
