mod quest01;

fn main() {
    let total_potions = quest01::no_of_potions::count_and_sum_characters();
    println!("My total potions: {:?}", total_potions);
}
