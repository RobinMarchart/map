#![feature(min_const_generics)]

use serde::{Deserialize, Serialize};

pub mod position;

pub use position::*;

pub trait Borders {}
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Borders4(bool, bool, bool, bool);
impl Borders for Borders4 {}
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Borders6(bool, bool, bool, bool, bool, bool);
impl Borders for Borders6 {}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Element<P: Position<P>> {
    variant: u16,
    position: P,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Entity<P: Position<P>> {
    id: u64,
    ent_type: String,
    variant: u8,
    orientation: u8,
    base_position: P,
    elements: Option<Vec<Element<P>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RenderEntity<P: Position<P>, B: Borders> {
    id: u64,
    ent_type: String,
    variant: u8,
    orientation: u8,
    base_position: P,
    elements: Option<Vec<P>>,
    impassable_tiles: Vec<Vec<bool>>,
    impassable_borders: Vec<Vec<B>>,
}
