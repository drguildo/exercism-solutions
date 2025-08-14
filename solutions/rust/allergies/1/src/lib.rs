pub struct Allergies(u32);

#[derive(Debug, PartialEq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        match allergen {
            &Allergen::Eggs => (self.0 & 0b00000001) > 0,
            &Allergen::Peanuts => (self.0 & 0b00000010) > 0,
            &Allergen::Shellfish => (self.0 & 0b00000100) > 0,
            &Allergen::Strawberries => (self.0 & 0b00001000) > 0,
            &Allergen::Tomatoes => (self.0 & 0b00010000) > 0,
            &Allergen::Chocolate => (self.0 & 0b00100000) > 0,
            &Allergen::Pollen => (self.0 & 0b01000000) > 0,
            &Allergen::Cats => (self.0 & 0b10000000) > 0,
        }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergens = Vec::new();

        if self.is_allergic_to(&Allergen::Eggs) {
            allergens.push(Allergen::Eggs);
        }
        if self.is_allergic_to(&Allergen::Peanuts) {
            allergens.push(Allergen::Peanuts);
        }
        if self.is_allergic_to(&Allergen::Shellfish) {
            allergens.push(Allergen::Shellfish);
        }
        if self.is_allergic_to(&Allergen::Strawberries) {
            allergens.push(Allergen::Strawberries);
        }
        if self.is_allergic_to(&Allergen::Tomatoes) {
            allergens.push(Allergen::Tomatoes);
        }
        if self.is_allergic_to(&Allergen::Chocolate) {
            allergens.push(Allergen::Chocolate);
        }
        if self.is_allergic_to(&Allergen::Pollen) {
            allergens.push(Allergen::Pollen);
        }
        if self.is_allergic_to(&Allergen::Cats) {
            allergens.push(Allergen::Cats);
        }

        allergens
    }
}
