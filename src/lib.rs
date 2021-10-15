mod coord;
mod direction;
mod grid;
pub mod iterator;
pub mod macros;
mod movement;
pub mod num;
pub mod parser;
mod range;
mod turtle;

pub use coord::Coord;
pub use direction::Direction;
pub use grid::Grid;
pub use movement::Movement;
pub use range::Range;
pub use turtle::Turtle;

pub use anyhow::*;
pub use rayon::prelude::*;
pub use termion;

#[macro_export]
macro_rules! answer {
    () => (println!());
    ($base:tt, $($args:expr)*) => ({
        use $crate::termion::{color, style};
        print!("{}", color::Fg(color::LightWhite));
        print!($base
        $(, format!("{}{}{}{}{}{}",
            style::Bold, style::Blink, color::Fg(color::Yellow),
            $args, style::Reset, color::Fg(color::LightWhite))
        )*);
        println!("{}", style::Reset);
    })
}
