use rltk::RGB;
use specs::prelude::*;

use specs_derive::*;

#[derive(Component, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug)]
pub struct Color {
    // vorrei codificare la mappatura valore -> colore e il numero di valori possibili
    pub value: usize,
}
