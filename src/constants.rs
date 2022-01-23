pub mod atoms {
    pub const WM_NAME: &str = "WM_NAME";
    pub const _NET_ACTIVE_WINDOW: &str = "_NET_ACTIVE_WINDOW";
    pub const _NET_CURRENT_DESKTOP: &str = "_NET_CURRENT_DESKTOP";
    pub const _NET_NUMBER_OF_DESKTOPS: &str = "_NET_NUMBER_OF_DESKTOPS";
    pub const _NET_WM_STATE_STICKY: &str = "_NET_WM_STATE_STICKY";
    pub const _NET_WM_STATE_ABOVE: &str = "_NET_WM_STATE_ABOVE";
    pub const _NET_WM_STATE: &str = "_NET_WM_STATE";
    pub const _NET_WM_WINDOW_TYPE: &str = "_NET_WM_WINDOW_TYPE";
    pub const _NET_WM_WINDOW_TYPE_DOCK: &str = "_NET_WM_WINDOW_TYPE_DOCK";
    pub const _NET_WM_STRUT: &str = "_NET_WM_STRUT";
    pub const _NET_WM_STRUT_PARTIAL: &str = "_NET_WM_STRUT_PARTIAL";
    pub const _NET_WM_DESKTOP: &str = "_NET_WM_DESKTOP";
    pub const _NET_WM_NAME: &str = "_NET_WM_NAME";
}

pub mod defaults {
    pub const BAR_FONT: &str = "Monospace 10";
    pub const BAR_BACKGROUND: &str = "#000000";
    pub const BAR_COLOR: &str = "#ffffff";
    pub const BAR_HEIGHT: i64 = 24;

    pub const BLOCK_PADDING: i32 = 10;
    pub const BLOCK_BORDER: i32 = 0;

    pub const CLOCK_FORMAT: &str = "%H:%M";
}

pub mod config {
    pub const BACKGROUND: &str = "background";
    pub const COLOR: &str = "color";
    pub const FONT: &str = "font";
    pub const HEIGHT: &str = "height";
    pub const TEXT: &str = "text";
    pub const PADDING: &str = "padding";
    pub const BORDER_BOTTOM: &str = "border-bottom";
    pub const BORDER_TOP: &str = "border-top";
    pub const BORDER_COLOR: &str = "border-color";

    pub const TIME_FORMAT: &str = "format";
}