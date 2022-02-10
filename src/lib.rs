#[derive(derive_new::new, derive_getters::Getters, Clone, Debug)]
struct Fruit {
    species: Species,
    height: usize,
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
        let species_prioritised = prioritise_species(fruit);
        let height_prioritised = prioritise_height(species_prioritised);

        height_prioritised.first().unwrap().clone()
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

fn prioritise_height(mut input: Vec<Fruit>) -> Vec<Fruit> {
    input.sort_by(|a, b| b.height().cmp(a.height()));
    input
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use super::*;

    #[test]
    fn test() {
        let under_test = FruitPrioritisationService::new();
        let mut fruit_input: Vec<Fruit> = vec![
            Fruit::new(Species::Apple, 1),
            Fruit::new(Species::Apple, 2),
            Fruit::new(Species::Pear, 1),
            Fruit::new(Species::Pear, 2),
        ];

        let selected = under_test.prioritise(fruit_input.clone());
        assert_that(&selected.species()).is_equal_to(&Species::Apple);
        assert_that(&selected.height()).is_equal_to(&2);

        fruit_input.reverse();
        let selected_reverse = under_test.prioritise(fruit_input);
        assert_that(&selected_reverse.species()).is_equal_to(&Species::Apple);
        assert_that(&selected_reverse.height()).is_equal_to(&2);
    }
}
