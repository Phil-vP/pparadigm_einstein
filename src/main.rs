mod types;
use types::{
    Brand, Color, Constraint, DirectConstraint, Drink, House, Location, LocationConstraint,
    Nationality, Pet, Trait,
};

fn main() {
    solve();
}

fn solve() {
    print!("Moin");
    // Initiate and fill the constraint and house vectors
    let mut constraint_vector: Vec<Constraint> = fill_vectors();
    let mut houses: Vec<House> = init_houses();

    let mut counter = 1;

    while constraint_vector.len() > 0 {
        // Initiate the vector that contains the indices of the constraints that are fulfilled this run and therefor need to be removed
        let mut removal_vec: Vec<usize> = Vec::new();

        for index in 0..constraint_vector.len() {
            let constraint = constraint_vector[index];

            if let Constraint::DirectConstraint(value) = constraint {

                let constraint_fulfilled = check_direct_constraint(&mut houses, value);
                if constraint_fulfilled {
                    // If this constraint was fully fulfilled, it can be removed from the list as it doesn't need to be checked anymore
                    println!(
                        "Direct Constraint {:?} | {:?} is fulfilled in round {}",
                        value.trait_1, value.trait_2, counter
                    );
                    removal_vec.push(index);
                }
            }
            if let Constraint::LocationConstraint(value) = constraint {

                let constraint_fulfilled = check_location_constraint(&mut houses, value);
                if constraint_fulfilled {
                    // If this constraint was fully fulfilled, it can be removed from the list as it doesn't need to be checked anymore
                    println!(
                        "Direct Constraint {:?} | {:?} is fulfilled in round {}",
                        value.trait_1, value.trait_2, counter
                    );
                    removal_vec.push(index);
                }
            }
        }

        // Remove all constraints that are fulfilled
        removal_vec.reverse();
        for i in removal_vec {
            constraint_vector.remove(i);
        }

        cleanup(&mut houses);

        counter += 1;
    }

    println!("\nFinal Result:\n");
    print_houses(&houses);
}

// Go through all houses
// If a house has a vector with only one occurence, this trait is being deleted everywhere else
fn cleanup(houses: &mut Vec<House>) {
    for index_1 in 0..houses.len() {
        let house = houses[index_1].clone();
        let all_traits_vec = vec![
            house.color_vec,
            house.nationality_vec,
            house.brand_vec,
            house.drink_vec,
            house.pet_vec,
        ];
        for vector in all_traits_vec {
            if vector.len() != 1 {
                continue;
            }
            for index_2 in 0..houses.len() {
                if index_2 == index_1 {
                    continue;
                }
                delete_trait(&mut houses[index_2], vector[0]);
            }
        }
    }
}

fn check_direct_constraint(houses: &mut Vec<House>, constraint: DirectConstraint) -> bool {
    let mut fulfilled_constraints: Vec<usize> = Vec::new();
    let trait_1 = constraint.trait_1;
    let trait_2 = constraint.trait_2;

    for index in 0..houses.len() {
        let house = &mut houses[index];

        let trait_1_contains = contains(house, &trait_1);
        let trait_2_contains = contains(house, &trait_2);

        if trait_1_contains && trait_2_contains {
            // If both contains are true, this pairing it possible for this house.
            fulfilled_constraints.push(index);
        } else {
            // If not, we can delete the values for this pairing as it's impossible
            if trait_1_contains {
                delete_trait(house, trait_1);
            }
            if trait_2_contains {
                delete_trait(house, trait_2);
            }
        }
    }

    // If only one house fulfills both trait, this constraint can be seen as solved
    if fulfilled_constraints.len() == 1 {
        // We found the house that fits this constraint. We now delete all other occurences of both traits
        // This is theoretically also done in the cleanup but we may save some rounds by doing this here already
        for index in 0..houses.len() {
            if index == fulfilled_constraints[0] {
                continue;
            }
            let house = &mut houses[index];
            delete_trait(house, trait_1);
            delete_trait(house, trait_2);
        }

        // We can also remove all other occurences apart from the fulfilled one from the current house
        let house = &mut houses[fulfilled_constraints[0]];
        delete_all_except(house, trait_1);
        delete_all_except(house, trait_2);

        // We return true so this constraint can be deleted from the list to get checked
        true
    } else {
        // This constraint wasn't completely fulfilled yet, we still have multiple combinations that might fulfill it
        false
    }
}

fn check_location_constraint(houses: &mut Vec<House>, constraint: LocationConstraint) -> bool {
    // This Vector contains tuples instead of just an index which indicate which pair fits.
    // It's constructed as follows: <index_left, index_right, trait_left, trait_right>
    let mut fulfilled_constraints: Vec<(usize, usize, Trait, Trait)> = Vec::new();
    let trait_1 = constraint.trait_1;
    let trait_2 = constraint.trait_2;

    // First, let's check left or right
    if constraint.location != Location::NextTo {
        let trait_left: Trait;
        let trait_right: Trait;
        if constraint.location == Location::Left {
            trait_left = trait_1;
            trait_right = trait_2;
        } else {
            trait_left = trait_2;
            trait_right = trait_1;
        }

        for index in 0..houses.len() - 1 {
            let house_slice = &mut houses[index..=index + 1];

            let trait_left_contains = contains(&house_slice[0], &trait_left);
            let trait_right_contains = contains(&house_slice[1], &trait_right);

            if trait_left_contains && trait_right_contains {
                // If both contains are true, this pairing it still possible for this house.
                // If this ends up being the only house (counter == 1), we can delete these traits from the rest of the houses
                fulfilled_constraints.push((index, index + 1, trait_left, trait_right));
            } else {
                // If not, we can delete the values for this pairing as it's impossible
                if trait_left_contains {
                    delete_trait(&mut house_slice[0], trait_left);
                }
                if trait_right_contains {
                    delete_trait(&mut house_slice[1], trait_right);
                }
            }
        }
    } else {
        // If we have to check if it's next to each other, it's unfortunately not as restrictive as only left or right.
        // We go through left to right and always check both possibilities (left and right trait)

        for index in 0..houses.len() {
            let trait_1_contains = contains(&houses[index], &trait_1);
            let trait_2_contains = contains(&houses[index], &trait_2);

            if trait_1_contains {
                // Does the left side exist for the current index and if yes contain trait_2?
                let first_trait_left_possible = if index > 0 {
                    contains(&houses[index - 1], &trait_2)
                } else {
                    false
                };

                // Does the right side exist for the current index and if yes contain trait_2?
                let first_trait_right_possible = if index < houses.len() - 1 {
                    contains(&houses[index + 1], &trait_2)
                } else {
                    false
                };

                // If this house is only surrounded by houses that don't contain trait 2, we can delete it from this house
                if !first_trait_left_possible && !first_trait_right_possible {
                    delete_trait(&mut houses[index], trait_1);
                }

                if first_trait_left_possible {
                    // This constraint fits for trait_1 one existing in the current house and trait_2 to the left
                    // <index_left, index_right, trait_left, trait_right>
                    fulfilled_constraints.push((index - 1, index, trait_2, trait_1));
                }

                if first_trait_right_possible {
                    // This constraint fits for trait_1 one existing in the current house and trait_2 to the right
                    // <index_left, index_right, trait_left, trait_right>
                    fulfilled_constraints.push((index, index + 1, trait_1, trait_2));
                }
            }

            if trait_2_contains {
                // Does the left side exist for the current index and if yes contain trait_1?
                let second_trait_left_possible = if index > 0 {
                    contains(&houses[index - 1], &trait_1)
                } else {
                    false
                };

                // Does the right side exist for the current index and if yes contain trait_1?
                let second_trait_right_possible = if index < houses.len() - 1 {
                    contains(&houses[index + 1], &trait_1)
                } else {
                    false
                };

                // If this house is only surrounded by houses that don't contain trait 1, we can delete it from this house
                if !second_trait_left_possible && !second_trait_right_possible {
                    delete_trait(&mut houses[index], trait_2);
                }
            }
        }
    }

    // If only one house fulfills both trait, it can be seen as solved
    if fulfilled_constraints.len() == 1 {
        // We found the house that fits this constraint. We now delete all other occurences of both traits
        // We can also remove all other occurences apart from the fulfilled one from the current house
        // <index_left, index_right, trait_left, trait_right>
        let tuple = fulfilled_constraints[0];
        for index in 0..houses.len() - 1 {
            if index == tuple.0 {
                continue;
            }
            let house_slice = &mut houses[index..=index + 1];
            delete_trait(&mut house_slice[0], tuple.2);
            delete_trait(&mut house_slice[1], tuple.3);
        }

        let house_slice = &mut houses[tuple.0..=tuple.1];

        delete_all_except(&mut house_slice[0], tuple.2);
        delete_all_except(&mut house_slice[1], tuple.3);

        // We return true so this constraint can be deleted from the list to get checked
        true
    } else {
        // This constraint wasn't completely fulfilled yet, we still have multiple combinations that might fulfill it
        false
    }
}

// Delete all but the given trait from the corresponding vector in the house
fn delete_all_except(house: &mut House, trait_to_keep: Trait) -> () {
    match trait_to_keep {
        Trait::Color(_) => house.color_vec.retain(|&x| x == trait_to_keep),
        Trait::Nationality(_) => house.nationality_vec.retain(|&x| x == trait_to_keep),
        Trait::Brand(_) => house.brand_vec.retain(|&x| x == trait_to_keep),
        Trait::Drink(_) => house.drink_vec.retain(|&x| x == trait_to_keep),
        Trait::Pet(_) => house.pet_vec.retain(|&x| x == trait_to_keep),
        Trait::Number(_) => house.number_vec.retain(|&x| x == trait_to_keep),
    };
}

// Delete the given trait from the corresponding vector in the house
fn delete_trait(house: &mut House, trait_to_delete: Trait) -> () {
    match trait_to_delete {
        Trait::Color(_) => house.color_vec.retain(|&x| x != trait_to_delete),
        Trait::Nationality(_) => house.nationality_vec.retain(|&x| x != trait_to_delete),
        Trait::Brand(_) => house.brand_vec.retain(|&x| x != trait_to_delete),
        Trait::Drink(_) => house.drink_vec.retain(|&x| x != trait_to_delete),
        Trait::Pet(_) => house.pet_vec.retain(|&x| x != trait_to_delete),
        Trait::Number(_) => house.number_vec.retain(|&x| x != trait_to_delete),
    };
}

// Does this house contain this trait?
fn contains(house: &House, trait_to_check: &Trait) -> bool {
    match trait_to_check {
        Trait::Color(_) => house.color_vec.contains(trait_to_check),
        Trait::Nationality(_) => house.nationality_vec.contains(trait_to_check),
        Trait::Brand(_) => house.brand_vec.contains(trait_to_check),
        Trait::Drink(_) => house.drink_vec.contains(trait_to_check),
        Trait::Pet(_) => house.pet_vec.contains(trait_to_check),
        Trait::Number(_) => house.number_vec.contains(trait_to_check),
    }
}

// Print out all houses
fn print_houses(houses: &Vec<House>) {
    for house in houses {
        println!("Number: {:?}", house.number_vec);
        println!("{} Color Map: {:?}", house.color_vec.len(), house.color_vec);
        println!(
            "{} Nationality Map: {:?}",
            house.nationality_vec.len(),
            house.nationality_vec
        );
        println!("{} Brand Map: {:?}", house.brand_vec.len(), house.brand_vec);
        println!("{} Drink Map: {:?}", house.drink_vec.len(), house.drink_vec);
        println!("{} Pet Map: {:?}\n", house.pet_vec.len(), house.pet_vec);
    }
}

fn init_houses() -> Vec<House> {
    let mut houses = Vec::new();

    let color_vec: Vec<Trait> = vec![
        Trait::Color(Color::Blue),
        Trait::Color(Color::Green),
        Trait::Color(Color::Red),
        Trait::Color(Color::White),
        Trait::Color(Color::Yellow),
    ];
    let nationality_vec = vec![
        Trait::Nationality(Nationality::British),
        Trait::Nationality(Nationality::Danish),
        Trait::Nationality(Nationality::German),
        Trait::Nationality(Nationality::Norwegian),
        Trait::Nationality(Nationality::Swedish),
    ];
    let brand_vec = vec![
        Trait::Brand(Brand::Dunhill),
        Trait::Brand(Brand::Marlboro),
        Trait::Brand(Brand::PallMall),
        Trait::Brand(Brand::Rothmanns),
        Trait::Brand(Brand::Winfield),
    ];
    let drink_vec = vec![
        Trait::Drink(Drink::Beer),
        Trait::Drink(Drink::Coffee),
        Trait::Drink(Drink::Milk),
        Trait::Drink(Drink::Tea),
        Trait::Drink(Drink::Water),
    ];
    let pet_vec = vec![
        Trait::Pet(Pet::Bird),
        Trait::Pet(Pet::Cat),
        Trait::Pet(Pet::Dog),
        Trait::Pet(Pet::Fish),
        Trait::Pet(Pet::Horse),
    ];

    // let number_vec = vec![Trait::Number(1), Trait::Number(2), Trait::Number(3), Trait::Number(4), Trait::Number(5)];

    for number in 1..=5 {
        houses.push(House {
            color_vec: color_vec.clone(),
            nationality_vec: nationality_vec.clone(),
            brand_vec: brand_vec.clone(),
            drink_vec: drink_vec.clone(),
            pet_vec: pet_vec.clone(),
            number_vec: vec![Trait::Number(number)],
        })
    }
    houses
}

fn fill_vectors() -> Vec<Constraint> {
    let mut dep_vector: Vec<Constraint> = Vec::new();

    // 5)
    dep_vector.push(Constraint::DirectConstraint(DirectConstraint {
        trait_1: Trait::Nationality(Nationality::British),
        trait_2: Trait::Color(Color::Red),
        fulfilled: false,
    }));

    // 6)
    dep_vector.push(Constraint::DirectConstraint(DirectConstraint {
        trait_1: Trait::Nationality(Nationality::Swedish),
        trait_2: Trait::Pet(Pet::Dog),
        fulfilled: false,
    }));

    // 7)
    dep_vector.push(Constraint::DirectConstraint(DirectConstraint {
        trait_1: Trait::Nationality(Nationality::Danish),
        trait_2: Trait::Drink(Drink::Tea),
        fulfilled: false,
    }));

    // 8)
    dep_vector.push(Constraint::DirectConstraint(DirectConstraint {
        trait_1: Trait::Nationality(Nationality::German),
        trait_2: Trait::Brand(Brand::Rothmanns),
        fulfilled: false,
    }));

    // 9)
    dep_vector.push(Constraint::DirectConstraint(DirectConstraint {
        trait_1: Trait::Color(Color::Green),
        trait_2: Trait::Drink(Drink::Coffee),
        fulfilled: false,
    }));

    // 10)
    dep_vector.push(Constraint::DirectConstraint(DirectConstraint {
        trait_1: Trait::Brand(Brand::Winfield),
        trait_2: Trait::Drink(Drink::Beer),
        fulfilled: false,
    }));

    // 11)
    dep_vector.push(Constraint::DirectConstraint(DirectConstraint {
        trait_1: Trait::Nationality(Nationality::Norwegian),
        trait_2: Trait::Number(1),
        fulfilled: false,
    }));

    // 12)
    dep_vector.push(Constraint::LocationConstraint(LocationConstraint {
        trait_1: Trait::Nationality(Nationality::Norwegian),
        trait_2: Trait::Color(Color::Blue),
        location: Location::NextTo,
        fulfilled: false,
    }));

    // 13)
    dep_vector.push(Constraint::DirectConstraint(DirectConstraint {
        trait_1: Trait::Color(Color::Yellow),
        trait_2: Trait::Brand(Brand::Dunhill),
        fulfilled: false,
    }));

    // 14)
    dep_vector.push(Constraint::DirectConstraint(DirectConstraint {
        trait_1: Trait::Brand(Brand::PallMall),
        trait_2: Trait::Pet(Pet::Bird),
        fulfilled: false,
    }));

    // 15)
    dep_vector.push(Constraint::DirectConstraint(DirectConstraint {
        trait_1: Trait::Number(3),
        trait_2: Trait::Drink(Drink::Milk),
        fulfilled: false,
    }));

    // 16)
    dep_vector.push(Constraint::LocationConstraint(LocationConstraint {
        trait_1: Trait::Color(Color::Green),
        trait_2: Trait::Color(Color::White),
        location: Location::Left,
        fulfilled: false,
    }));

    // 17)
    dep_vector.push(Constraint::LocationConstraint(LocationConstraint {
        trait_1: Trait::Brand(Brand::Marlboro),
        trait_2: Trait::Pet(Pet::Cat),
        location: Location::NextTo,
        fulfilled: false,
    }));

    // 18)
    dep_vector.push(Constraint::LocationConstraint(LocationConstraint {
        trait_1: Trait::Brand(Brand::Marlboro),
        trait_2: Trait::Drink(Drink::Water),
        location: Location::NextTo,
        fulfilled: false,
    }));

    // 19)
    dep_vector.push(Constraint::LocationConstraint(LocationConstraint {
        trait_1: Trait::Pet(Pet::Horse),
        trait_2: Trait::Brand(Brand::Dunhill),
        location: Location::NextTo,
        fulfilled: false,
    }));

    dep_vector
}
