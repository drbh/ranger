use ranger::manager::PositionManager;
use ranger::utils::calculate_contract_values;
use ranger::utils::contract_intrinsic_values_at_price;
use ranger::utils::get_lines_through_zero;
use ranger::utils::get_zero_intersections;

fn main() {
    // a butterfly
    let option_input = optioncontracts::OptionBuilder::new()
        .kind(optioncontracts::Type::Call)
        .direction(optioncontracts::Direction::Long)
        .strike(10.0)
        .price(1.0)
        .finish();

    let option_input2 = optioncontracts::OptionBuilder::new()
        .kind(optioncontracts::Type::Call)
        .direction(optioncontracts::Direction::Short)
        .strike(15.0)
        .price(1.0)
        .finish();

    let option_input3 = optioncontracts::OptionBuilder::new()
        .kind(optioncontracts::Type::Call)
        .direction(optioncontracts::Direction::Short)
        .strike(15.0)
        .price(1.0)
        .finish();

    let option_input4 = optioncontracts::OptionBuilder::new()
        .kind(optioncontracts::Type::Call)
        .direction(optioncontracts::Direction::Long)
        .strike(20.0)
        .price(1.0)
        .finish();

    let mut position_manager = PositionManager::new();

    position_manager.add(option_input4);
    position_manager.add(option_input3);
    position_manager.add(option_input2);
    position_manager.add(option_input);

    println!("{:#?}", position_manager);

    let sorted = position_manager.sorted_elements();

    let mut contracts = vec![];
    for position in sorted.iter() {
        for _ in 0..position.quantity {
            contracts.push(position.contract.clone());
        }
    }

    println!("{:#?}", contracts);

    let payout_vertexes = calculate_contract_values(contracts.clone());
    println!("payout_vertexes\t{:?}", payout_vertexes);

    let lines_through_zero = get_lines_through_zero(payout_vertexes);
    let zero_intersections = get_zero_intersections(lines_through_zero);
    println!("zero_intersections\t{:?}", zero_intersections);

    let value_at_price = contract_intrinsic_values_at_price(16.32, contracts);
    println!("value_at_price\t{:?}", value_at_price);
}
