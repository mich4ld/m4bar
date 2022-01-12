use std::ffi::CString;

use cairo_sys::{cairo_xlib_surface_create, cairo_create, cairo_set_source_rgb, cairo_xlib_surface_set_size, cairo_rectangle, cairo_fill};
use pango::ffi::{pango_font_description_from_string, pango_layout_set_font_description, pango_layout_set_text, pango_layout_get_pixel_size, PangoLayout};
use pangocairo::ffi::{pango_cairo_create_layout, pango_cairo_show_layout, pango_cairo_update_layout};
use cairo_sys::{cairo_t, cairo_surface_t};

use crate::{protocol::X11, colors};

pub struct BlockAttributes {
    pub x: i32,
    pub width: u32,
    pub height: u32,
    pub background: String,
    pub border_bottom: i32,
    pub border_top: i32,
    pub border_color: String,
    pub padding: i32,
    pub color: String,
    pub font: String,
}

pub struct Block<'a> {
    x11: &'a X11,
    window: u64,
    attributes: BlockAttributes,
    layout: *mut PangoLayout,
    cairo_context: *mut cairo_t,
    surface: *mut cairo_surface_t,
    color_rgb: [f64; 3],
    bg_rgb: [f64; 3],
    border_rgb: [f64; 3],
    is_exposed: bool,
    text_height: i32,
    text: String,
}

impl Block<'_> {
    pub unsafe fn new(x11: &X11, bar: u64, mut attributes: BlockAttributes) -> Block {
        attributes.width = attributes.padding as u32 + attributes.width + attributes.padding as u32;
        let window = x11.create_subwindow(
            bar,
            attributes.x, 
            0, 
            attributes.width, 
            attributes.height, 
            colors::hex64(&attributes.background),
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

        Block { 
            x11, 
            window, 
            bg_rgb: colors::hex_to_rgb(&attributes.background),
            color_rgb: colors::hex_to_rgb(&attributes.color),
            border_rgb: colors::hex_to_rgb(&attributes.border_color),
            attributes, 
            layout, 
            cairo_context, 
            surface,
            is_exposed: false,
            text_height: 0,
            text: String::new()
        }
    }

    unsafe fn set_color(&self, rgb: [f64; 3]) {
        cairo_set_source_rgb(self.cairo_context, rgb[0], rgb[1], rgb[2]);
    }

    unsafe fn compute(&mut self, text: &str) {
        let font = self.attributes.font.clone();
        let c_font = CString::new(font).unwrap();
        let pango_font = pango_font_description_from_string(c_font.as_ptr());

        let text_len = text.len();
        let c_text = CString::new(text).unwrap();
        pango_layout_set_font_description(self.layout, pango_font);
        pango_layout_set_text(self.layout, c_text.as_ptr(), text_len as i32);

        let (width, height) = self.get_layout_size();
        self.resize_width(width);
        self.text_height = height;
    }

    unsafe fn draw_borders(&self) {
        if self.attributes.border_top != 0 {
            self.set_color(self.border_rgb);
            cairo_rectangle(
                self.cairo_context, 
                0.0, 
                0.0, 
                self.attributes.width as f64, 
                self.attributes.border_top as f64,
            );

            cairo_fill(self.cairo_context);
        }

        if self.attributes.border_bottom != 0 {
            self.set_color(self.border_rgb);
            cairo_rectangle(
                self.cairo_context, 
                0.0,
                self.attributes.height as f64 - self.attributes.border_bottom as f64, 
                self.attributes.width as f64, 
                self.attributes.border_bottom as f64,
            );

            cairo_fill(self.cairo_context);
        }
    }

    unsafe fn draw(&self, text: String) {
        let text_len = text.len();
        let c_text = CString::new(text).unwrap();

        self.set_color(self.bg_rgb);
        cairo_rectangle(self.cairo_context, 0.0, 0.0, self.attributes.width as f64, self.attributes.height as f64);
        cairo_fill(self.cairo_context);

        self.draw_borders();

        self.set_color(self.color_rgb);
        let text_y = self.attributes.height as f64 / 2.0 - self.text_height as f64 / 2.0;
        cairo_rectangle(self.cairo_context, self.attributes.padding as f64, text_y as f64, self.attributes.width as f64, self.attributes.height as f64);
        pango_layout_set_text(self.layout, c_text.as_ptr(), text_len as i32);
    }

    unsafe fn get_layout_size(&self) -> (i32, i32) {
        let padding = self.attributes.padding;
        let mut width = 0;
        let mut height = 0;
        pango_layout_get_pixel_size(self.layout, &mut width, &mut height);

        (padding + width + padding, height)
    }

    unsafe fn resize_width(&mut self, width: i32) {
        self.attributes.width = width as u32;

        self.x11.resize_window(self.window, width as u32, self.attributes.height);
        cairo_xlib_surface_set_size(self.surface, width, self.attributes.height as i32);
        
        self.x11.show_window(self.window);
    }

    pub unsafe fn init(&mut self, text: String) {
        self.compute(&text);
        self.text = text;
        
        let (width, _height) = self.get_layout_size();
        self.resize_width(width);
    }

    pub unsafe fn expose(&mut self) {
        if !self.is_exposed {
            self.show();
            self.render(self.text.clone());
            self.is_exposed = true;
        }
    }

    pub unsafe fn render(&mut self, text: String) {
        self.text = text.clone();
        self.draw(text);
        let (width, _height) = self.get_layout_size();
        
        if width != self.attributes.width as i32 {
            self.resize_width(width);
        }

        pango_cairo_update_layout(self.cairo_context, self.layout);
        self.show();
    }

    pub unsafe fn rerender(&mut self, text: String) {
        if text != self.text {
            self.render(text);
        }
    }

    pub unsafe fn show(&mut self) {
        pango_cairo_show_layout(self.cairo_context, self.layout);
    }
}