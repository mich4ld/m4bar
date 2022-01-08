use std::ffi::CString;

use cairo_sys::{cairo_xlib_surface_create, cairo_create, cairo_set_source_rgb, cairo_xlib_surface_set_size};
use pango::ffi::{pango_font_description_from_string, pango_layout_set_font_description, pango_layout_set_text, pango_layout_get_pixel_size, PangoLayout};
use pangocairo::ffi::{pango_cairo_create_layout, pango_cairo_show_layout};
use cairo_sys::{cairo_t, cairo_surface_t};

use crate::{protocol::X11, colors::rgb};

pub struct BlockAttributes {
    pub x: i32,
    pub width: u32,
    pub height: u32,
    pub background: u64,
    pub border_bottom: i32,
    pub border_top: i32,
    pub border_color: u64,
    pub padding: i32,
    pub color: i32,
    pub font: String,
}

pub struct Block<'a> {
    x11: &'a X11,
    window: u64,
    attributes: BlockAttributes,
    layout: *mut PangoLayout,
    cairo_context: *mut cairo_t,
    surface: *mut cairo_surface_t,
}

impl Block<'_> {
    pub unsafe fn new(x11: &X11, bar: u64, attributes: BlockAttributes) -> Block {
        let window = x11.create_subwindow(
            bar,
            attributes.x, 
            0, 
            attributes.width, 
            attributes.height, 
            attributes.background
        );

        let surface = cairo_xlib_surface_create(
            x11.display,
            window,
            x11.visual,
            attributes.width as i32,
            attributes.height as i32,
        );

        let cairo_context = cairo_create(surface);
        let layout = pango_cairo_create_layout(cairo_context);

        Block { x11, window, attributes, layout, cairo_context, surface }
    }

    pub unsafe fn render(&mut self, text: String) {
        let font = self.attributes.font.clone();
        let c_font = CString::new(font).unwrap();
        let pango_font = pango_font_description_from_string(c_font.as_ptr());

        let text_len = text.len();
        let c_text = CString::new(text).unwrap();

        let black = rgb(1, 0, 0);
        println!("RGB: ({}, {}, {})", black[0], black[1], black[2]);
        cairo_set_source_rgb(self.cairo_context, black[0], black[1], black[2]);
        pango_layout_set_font_description(self.layout, pango_font);
        pango_layout_set_text(self.layout, c_text.as_ptr(), text_len as i32);

        let mut width = 0;
        let mut height = 0;
        pango_layout_get_pixel_size(self.layout, &mut width, &mut height);

        self.attributes.width = width as u32;
        println!("WIDTH: {} HEIGHT: {}", width, height);
        self.x11.resize_window(self.window, width as u32, self.attributes.height);
        cairo_xlib_surface_set_size(self.surface, width, self.attributes.height as i32);
        self.x11.show_window(self.window);
    }

    pub unsafe fn show(&self) {
        pango_cairo_show_layout(self.cairo_context, self.layout);
    }
}