extern crate xor_distance_exercise;

use xor_distance_exercise::delivery_system::FoodDeliverySystem;

fn main() {
    let farms = vec![
        0, 1, 2, 4, 6, 8, 12, 18, 19, 20, 21, 22, 406, 407, 408, 409, 410, 444, 445,
    ];

    let delivery_system: FoodDeliverySystem<u64> = FoodDeliverySystem::new(farms.clone());

    let position = 10;
    let count = 10;

    // Get closest farms and reversed guess of possible customer's `position`.
    let closest_farms = delivery_system.closest_farms(position, count);
    let position_guess = delivery_system
        .reverse_closest_farms(&closest_farms)
        .unwrap();

    // Check that both `position` and `position_guess` produce the same result.
    let closest_farms_to_guess = delivery_system.closest_farms(position_guess, count);
    assert_eq!(closest_farms, closest_farms_to_guess);

    println!("Farms list: {:?}", farms);
    println!(
        "Closest {} farms to customer's position {} are: {:?}",
        count, position, closest_farms
    );
    println!(
        "Reversed guess of customer's possible position is: {}",
        position_guess
    );
    println!(
        "Closest {} farms to reversed guess of customer's possible position{} are: {:?}",
        count, position_guess, closest_farms_to_guess
    );
}
