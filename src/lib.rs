#[derive(derive_new::new, derive_getters::Getters, Clone)]
struct Fruit {
    species: Species,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Species {
    Apple,
    Pear,
}

#[derive(derive_new::new)]
struct FruitPrioritisationService;

impl FruitPrioritisationService {
    fn prioritise(&self, fruit: Vec<Fruit>) -> Fruit {
        prioritise_species(fruit).first().unwrap().clone()
    }
}

fn prioritise_species(mut input: Vec<Fruit>) -> Vec<Fruit> {
    input.sort_by(|a, b| {
        const ORDER: [Species; 2] = [Species::Apple, Species::Pear];

        let a_index = ORDER
            .iter()
            .position(|species| species == a.species())
            .unwrap();
        let b_index = ORDER
            .iter()
            .position(|species| species == b.species())
            .unwrap();

        a_index.cmp(&b_index)
    });

    input
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use super::*;

    #[test]
    fn test() {
        let under_test = FruitPrioritisationService::new();
        let mut fruit_input: Vec<Fruit> =
            vec![Fruit::new(Species::Apple), Fruit::new(Species::Pear)];

        assert_that(&under_test.prioritise(fruit_input.clone()).species())
            .is_equal_to(&Species::Apple);
        fruit_input.reverse();
        assert_that(&under_test.prioritise(fruit_input).species()).is_equal_to(&Species::Apple);
    }
}
