//! Motivation (lot 3): levels (SUB001501), stars (SUB001502), badges (SUB001503), avatar
//! accessories (SUB001504) and the goal of the day (SUB001507). Rewards the effort and the
//! regularity: nothing is ever taken away.

pub mod accessories;
pub mod badges;
pub mod daily_goal;
pub mod levels;
mod service;
pub mod stars;

pub use service::*;
