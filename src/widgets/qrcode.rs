// SPDX-License-Identifier: MIT OR Apache-2.0
use core::{ops::Deref, ptr::null_mut};

use oxivgl_sys::*;

use super::{
    WidgetError,
    obj::{AsLvHandle, Obj},
};

/// LVGL QR code widget (`lv_qrcode`, a canvas subclass). Wraps
/// [`Obj`](super::obj::Obj) and `Deref`s to it for style and layout methods.
///
/// Requires `LV_USE_QRCODE 1` (and `LV_USE_CANVAS 1`) in the application's
/// `lv_conf.h`; the type only exists when the QR code symbols reached the
/// bindings (`cfg(lvgl_qrcode)`).
///
/// Set the size and colours first, then call [`update`](Self::update):
/// [`set_size`](Self::set_size) clears the code, and the colours are applied
/// when the code is encoded.
///
/// ```ignore
/// let qr = Qrcode::new(&screen)?;
/// qr.set_size(150).set_dark_color(color_black()).set_light_color(color_white());
/// qr.update(b"https://lvgl.io")?;
/// ```
#[derive(Debug)]
pub struct Qrcode<'p> {
    obj: Obj<'p>,
}

impl<'p> AsLvHandle for Qrcode<'p> {
    fn lv_handle(&self) -> *mut lv_obj_t {
        self.obj.lv_handle()
    }
}

impl<'p> Deref for Qrcode<'p> {
    type Target = Obj<'p>;
    fn deref(&self) -> &Obj<'p> {
        &self.obj
    }
}

impl<'p> Qrcode<'p> {
    /// Create a QR code widget as a child of `parent`. Returns
    /// [`WidgetError::LvglNullPointer`] on OOM.
    pub fn new(parent: &impl AsLvHandle) -> Result<Self, WidgetError> {
        let parent_ptr = parent.lv_handle();
        assert_ne!(parent_ptr, null_mut(), "Parent widget cannot be null");
        // SAFETY: parent_ptr non-null (asserted above); lv_init() called via
        // LvglDriver.
        let handle = unsafe { lv_qrcode_create(parent_ptr) };
        if handle.is_null() { Err(WidgetError::LvglNullPointer) } else { Ok(Qrcode { obj: Obj::from_raw(handle) }) }
    }

    /// Set the side length in pixels. Allocates a new 1-bit canvas buffer from
    /// LVGL's heap and **clears** the code: call [`update`](Self::update)
    /// afterwards.
    pub fn set_size(&self, size: i32) -> &Self {
        // SAFETY: handle non-null (constructor guarantees). On allocation
        // failure LVGL logs and keeps the previous buffer (lv_qrcode.c,
        // lv_qrcode_set_size).
        unsafe { lv_qrcode_set_size(self.obj.handle(), size) };
        self
    }

    /// Set the colour of the dark modules. Takes effect on the next
    /// [`update`](Self::update).
    pub fn set_dark_color(&self, color: lv_color_t) -> &Self {
        // SAFETY: handle non-null (constructor guarantees).
        unsafe { lv_qrcode_set_dark_color(self.obj.handle(), color) };
        self
    }

    /// Set the colour of the light modules (background). Takes effect on the
    /// next [`update`](Self::update).
    pub fn set_light_color(&self, color: lv_color_t) -> &Self {
        // SAFETY: handle non-null (constructor guarantees).
        unsafe { lv_qrcode_set_light_color(self.obj.handle(), color) };
        self
    }

    /// Enable or disable the quiet zone (the light border around the code that
    /// scanners need). Takes effect on the next [`update`](Self::update).
    pub fn set_quiet_zone(&self, enable: bool) -> &Self {
        // SAFETY: handle non-null (constructor guarantees).
        unsafe { lv_qrcode_set_quiet_zone(self.obj.handle(), enable) };
        self
    }

    /// Encode `data` and redraw the code.
    ///
    /// Returns [`WidgetError::LvglInvalid`] if the data does not fit a QR code
    /// (more than `qrcodegen_BUFFER_LEN_MAX` bytes) or the canvas buffer could
    /// not be allocated.
    pub fn update(&self, data: &[u8]) -> Result<&Self, WidgetError> {
        let len = u32::try_from(data.len()).map_err(|_| WidgetError::LvglInvalid)?;
        // SAFETY: handle non-null (constructor guarantees); `data` is valid for
        // `len` bytes. LVGL encodes it immediately into its own buffers and keeps
        // no pointer (lv_qrcode.c, lv_qrcode_update), so no lifetime is required.
        let res = unsafe { lv_qrcode_update(self.obj.handle(), data.as_ptr().cast(), len) };
        if res == lv_result_t_LV_RESULT_OK { Ok(self) } else { Err(WidgetError::LvglInvalid) }
    }
}
