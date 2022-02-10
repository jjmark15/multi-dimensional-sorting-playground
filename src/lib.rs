struct PrioritisationBuilder {
    functions: Vec<fn(Vec<Fruit>) -> Vec<Fruit>>,
}

impl PrioritisationBuilder {
    fn first(prioritisation: fn(Vec<Fruit>) -> Vec<Fruit>) -> Self {
        let prioritisation_builder = PrioritisationBuilder { functions: vec![] };

        prioritisation_builder.then(prioritisation)
    }

    fn then(mut self, prioritisation: fn(Vec<Fruit>) -> Vec<Fruit>) -> Self {
        self.functions.push(prioritisation);
        self
    }

    fn execute(self, input: Vec<Fruit>) -> Vec<Fruit> {
        self.functions
            .into_iter()
            .rev()
            .fold(input, |prioritised, func| func(prioritised))
    }
}

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
    fn prioritise(&self, fruit: Vec<Fruit>) -> Vec<Fruit> {
        PrioritisationBuilder::first(prioritise_height)
            .then(prioritise_species)
            .execute(fruit)
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

    fn input_data() -> Vec<Fruit> {
        vec![
            Fruit::new(Species::Apple, 1),
            Fruit::new(Species::Apple, 2),
            Fruit::new(Species::Pear, 1),
            Fruit::new(Species::Pear, 2),
        ]
    }

    fn reversed_input_data() -> Vec<Fruit> {
        let mut data = input_data();
        data.reverse();
        data
    }

    trait SecondPosition<T> {
        fn second(&self) -> Option<&T>;
    }

    impl<T> SecondPosition<T> for [T] {
        fn second(&self) -> Option<&T> {
            if let [_first, second, ..] = self {
                Some(second)
            } else {
                None
            }
        }
    }

    #[test]
    fn prioritises_greater_height_then_apples_over_pears() {
        let under_test = FruitPrioritisationService::new();
        let fruit_input: Vec<Fruit> = input_data();

        let prioritised = under_test.prioritise(fruit_input);

        let first = prioritised.first().unwrap().clone();
        assert_that(first.species()).is_equal_to(Species::Apple);
        assert_that(first.height()).is_equal_to(2);

        let second = prioritised.second().unwrap().clone();
        assert_that(second.species()).is_equal_to(Species::Pear);
        assert_that(second.height()).is_equal_to(2);
    }

    #[test]
    fn prioritises_greater_height_then_apples_over_pears_with_reversed_input() {
        let under_test = FruitPrioritisationService::new();
        let fruit_input: Vec<Fruit> = reversed_input_data();

        let prioritised = under_test.prioritise(fruit_input);

        let first = prioritised.first().unwrap().clone();
        assert_that(first.species()).is_equal_to(Species::Apple);
        assert_that(first.height()).is_equal_to(2);

        let second = prioritised.second().unwrap().clone();
        assert_that(second.species()).is_equal_to(Species::Pear);
        assert_that(second.height()).is_equal_to(2);
    }
}
