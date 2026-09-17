
pub struct Allergies{
    //Add the field for the Allergies which is a vector of Allergen
    allergies: Vec<Allergen>,
}

//Add Clone, Copy trait
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

        //initialize the vector of the Allergens in ascending order
        let all_allergens = vec![Allergen::Eggs, Allergen::Peanuts, Allergen::Shellfish, Allergen::Strawberries, Allergen::Tomatoes,
                            Allergen::Chocolate, Allergen::Pollen, Allergen::Cats];

        //Initialize a vector that will contain the allergen as per score
        let mut allergies: Vec<Allergen> = Vec::new();
        
        //if the given score is 0, person is allergic to nothing. so return the empty vector wraping in the struct
        if score == 0 {
            return Self {allergies};
        }

        //make the score mutable
        let mut score = score;

        //itrerate in loop untill all the allergens are found as per score
        loop {
            //get the nearest n of the 2^n of the score
            let allergen_index = score.ilog2();

            //if found allergen exist in the given all allergen list add it to allergies list
            match all_allergens.get(allergen_index as usize) {
                Some(allergen) => allergies.push(allergen.clone()),
                None => ()
            }

            //Update the score
            score = score - 2_u32.pow(allergen_index);

            //if score is less than one, all allergens found. Break the loop.
            if score < 1 {
                break;
            }
        }

        //reverse the allergies list and return wrapping in struct
        allergies.reverse();
        Self { allergies: allergies }
        
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.clone()
    }
}
