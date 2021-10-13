#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
    White,
    Yellow,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Nationality {
    German,
    Swedish,
    Danish,
    British,
    Norwegian,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Brand {
    Rothmanns,
    Winfield,
    Dunhill,
    PallMall,
    Marlboro,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Drink {
    Tea,
    Coffee,
    Beer,
    Milk,
    Water,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Pet {
    Dog,
    Bird,
    Cat,
    Horse,
    Fish,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct House {
    pub color_vec: Vec<Trait>,
    pub nationality_vec: Vec<Trait>,
    pub brand_vec: Vec<Trait>,
    pub drink_vec: Vec<Trait>,
    pub pet_vec: Vec<Trait>,
    pub number_vec: Vec<Trait>,
}

// Linter Warning for Dead Code is switched off here as it throws a warning for "Right"
// For consistency, the Right direction is being left in though
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[allow(dead_code)]
pub enum Location {
    NextTo,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Trait {
    Color(Color),
    Nationality(Nationality),
    Brand(Brand),
    Drink(Drink),
    Pet(Pet),
    Number(i32),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct DirectConstraint {
    pub trait_1: Trait,
    pub trait_2: Trait,
    pub fulfilled: bool,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct LocationConstraint {
    pub trait_1: Trait,
    pub trait_2: Trait,
    pub location: Location,
    pub fulfilled: bool,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Constraint {
    DirectConstraint(DirectConstraint),
    LocationConstraint(LocationConstraint),
}
