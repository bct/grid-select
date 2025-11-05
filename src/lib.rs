pub mod config;
pub mod grid;
pub mod layout;
pub mod render;
pub mod spiral;
pub mod state;
pub mod window;

mod colour;
mod text;

#[macro_export]
macro_rules! prog_name {
    () => {
        "grid-select"
    };
}
