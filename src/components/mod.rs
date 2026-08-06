//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.

mod hero;
pub use hero::Hero;

mod ccna;
pub use ccna::CcnaBookPage;

pub mod green_div;
pub mod red_div;

pub mod input;
pub mod dialog;
pub mod button;
pub mod demo;
pub mod card;
pub mod separator;
