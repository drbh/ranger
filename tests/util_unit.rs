#![allow(dead_code, unused_imports)]

#[cfg(test)]
mod tests {
    use ranger::utils::calculate_contract_values;
    use ranger::utils::get_lines_through_zero;
    use ranger::utils::get_zero_intersections;

    #[test]
    fn buttefly_at_zero() {
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

        let contracts = vec![option_input, option_input2, option_input3, option_input4];

        let current_price = 20.0;
        let total: f64 = contracts
            .iter()
            .map(|contract| optioncontracts::execute_option(&contract, current_price))
            .sum();
        assert_eq!(total, 0.0);
    }

    #[test]
    fn at_strikes() {
        let option_input = optioncontracts::OptionBuilder::new()
            .kind(optioncontracts::Type::Call)
            .direction(optioncontracts::Direction::Long)
            .strike(8.5)
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

        let option_input5 = optioncontracts::OptionBuilder::new()
            .kind(optioncontracts::Type::Put)
            .direction(optioncontracts::Direction::Long)
            .strike(25.0)
            .price(1.0)
            .finish();

        let option_input6 = optioncontracts::OptionBuilder::new()
            .kind(optioncontracts::Type::Put)
            .direction(optioncontracts::Direction::Long)
            .strike(26.0)
            .price(1.0)
            .finish();

        let option_input7 = optioncontracts::OptionBuilder::new()
            .kind(optioncontracts::Type::Call)
            .direction(optioncontracts::Direction::Long)
            .strike(27.0)
            .price(1.0)
            .finish();

        let contracts = vec![
            option_input,
            option_input2,
            option_input3,
            option_input4,
            option_input5,
            option_input6,
            option_input7,
        ];

        let contract_values = calculate_contract_values(contracts);
        let lines_through_zero = get_lines_through_zero(contract_values);
        let zero_intersections = get_zero_intersections(lines_through_zero);

        assert_eq!(zero_intersections[0].0, 24.75);
        assert_eq!(zero_intersections[1].0, 28.5);
    }

    #[test]
    fn at_strikes2() {
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

        let contracts = vec![option_input, option_input2, option_input3, option_input4];

        let contract_values = calculate_contract_values(contracts);
        let lines_through_zero = get_lines_through_zero(contract_values);
        let zero_intersections = get_zero_intersections(lines_through_zero);

        assert_eq!(zero_intersections[0].0, 10.0);
        assert_eq!(zero_intersections[1].0, 20.0);
    }
}
