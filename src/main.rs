#![allow(dead_code)]

use std::collections::HashSet;
trait Caloric {
    fn calories(&self) -> u32;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Vegetable {
    Tomato,
    Cucumber,
    SweetPotato,
}

impl Caloric for Vegetable {
    fn calories(&self) -> u32 {
        match self {
            Self::Tomato => 20,
            Self::Cucumber => 15,
            Self::SweetPotato => 100,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Protein {
    CrispyChicken,
    FriedChicken,
    Steak,
    Tofu,
}

impl Caloric for Protein {
    fn calories(&self) -> u32 {
        match self {
            Self::CrispyChicken => 400,
            Self::FriedChicken => 500,
            Self::Steak => 300,
            Self::Tofu => 200,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum Dressing {
    Ranch,
    Vinaigrette,
    Italian,
}

impl Caloric for Dressing {
    fn calories(&self) -> u32 {
        match self {
            Self::Ranch => 150,
            Self::Vinaigrette => 120,
            Self::Italian => 130,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Salad {
    protein: Protein,
    vegetables: Vec<Vegetable>,
    dressing: Dressing,
}

impl Salad {
    fn new(protein: Protein, vegetables: Vec<Vegetable>, dressing: Dressing) -> Self {
        Self {
            protein,
            vegetables,
            dressing,
        }
    }

    fn is_valid(&self) -> bool {
        self.vegetables.len() > 0
    }

    fn calories(&self) -> u32 {
        self.protein.calories()
            + self.dressing.calories()
            + self
                .vegetables
                .iter()
                .map(|vegie| vegie.calories())
                .sum::<u32>()
    }

    fn has_duplicate_veg(&self) -> bool {
        self.vegetables
            .clone()
            .into_iter()
            .fold(HashSet::<Vegetable>::new(), |mut data, vegetable| {
                data.insert(vegetable);
                data
            })
            .len()
            < self.vegetables.len()
    }
}
#[cfg(test)]

mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::{fixture, rstest};

    fn salad_contains_protein_vegetables_and_dressing() {
        let salad = Salad::new(
            Protein::Steak,
            vec![Vegetable::SweetPotato, Vegetable::Tomato],
            Dressing::Italian,
        );
        assert_eq!(salad.protein, Protein::Steak);
        assert_eq!(salad.dressing, Dressing::Italian);
    }
    #[fixture]
    fn chicken_salad_with_three_vegies_and_dressing() -> Salad {
        Salad::new(
            Protein::CrispyChicken,
            vec![
                Vegetable::SweetPotato,
                Vegetable::Tomato,
                Vegetable::Cucumber,
            ],
            Dressing::Ranch,
        )
    }
    #[rstest]
    fn salad_should_have_at_least_one_vegetable(
        chicken_salad_with_three_vegies_and_dressing: Salad,
    ) {
        assert!(chicken_salad_with_three_vegies_and_dressing.is_valid());
    }

    #[rstest]
    fn calculating_the_total_calories_in_the_sald(
        chicken_salad_with_three_vegies_and_dressing: Salad,
    ) {
        assert_eq!(chicken_salad_with_three_vegies_and_dressing.calories(), 685);
    }

    #[rstest]

    fn salad_has_duplicate() {
        let salad = Salad::new(
            Protein::Tofu,
            vec![Vegetable::Cucumber, Vegetable::Cucumber],
            Dressing::Ranch,
        );

        assert!(salad.has_duplicate_veg());
    }
}

fn main() {}

