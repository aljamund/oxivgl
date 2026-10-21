#![cfg_attr(target_arch = "xtensa", no_std, no_main)]
#![cfg_attr(
    target_arch = "xtensa",
    feature(impl_trait_in_assoc_type, type_alias_impl_trait)
)]
// SPDX-License-Identifier: MIT OR Apache-2.0
//! QR code 1 — Create a QR code
//!
//! A 150 px QR code for "https://lvgl.io", dark blue on light blue, centred,
//! with a 5 px border in the light colour. The border uses a shared `Style`
//! instead of the C example's per-object style setters.

use oxivgl::{
    style::{Palette, Style, StyleBuilder, palette_darken, palette_lighten},
    view::{NavAction, View},
    widgets::{Obj, Part, Qrcode, WidgetError},
};

#[derive(Default)]
struct Qrcode1 {
    _qr: Option<Qrcode<'static>>,
    _style_border: Option<Style>,
}

impl View for Qrcode1 {
    fn create(&mut self, container: &Obj<'static>) -> Result<(), WidgetError> {
        let bg_color = palette_lighten(Palette::LightBlue, 5);
        let fg_color = palette_darken(Palette::Blue, 4);

        let qr = Qrcode::new(container)?;
        qr.set_size(150).set_dark_color(fg_color).set_light_color(bg_color);
        qr.update(b"https://lvgl.io")?;
        qr.center();

        // Add a border with bg_color.
        let mut style_border = StyleBuilder::new();
        style_border.border_color(bg_color).border_width(5);
        let style_border = style_border.build();
        qr.add_style(&style_border, Part::Main);

        self._qr = Some(qr);
        self._style_border = Some(style_border);
        Ok(())
    }

    fn update(&mut self) -> Result<NavAction, WidgetError> {
        Ok(NavAction::None)
    }
}

oxivgl_examples_common::example_main!(Qrcode1::default());
