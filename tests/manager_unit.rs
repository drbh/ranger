#![allow(dead_code, unused_imports)]

#[cfg(test)]
mod tests {
    use ranger::manager::PositionManager;
    use ranger::utils::calculate_contract_values;
    use ranger::utils::get_lines_through_zero;
    use ranger::utils::get_zero_intersections;

    #[test]
    fn build_manager() {
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

        let mut option_manager = PositionManager::new();

        option_manager.add(option_input4);
        option_manager.add(option_input3);
        option_manager.add(option_input2);
        option_manager.add(option_input);

        println!("{:#?}", option_manager);

        assert_eq!(
            option_manager.sorted_key_list,
            vec![
                "10::Long::Call".to_string(),
                "15::Short::Call".to_string(),
                "20::Long::Call".to_string(),
            ]
        );
    }

    #[test]
    fn build_manager_cloned() {
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

        let option_input4 = optioncontracts::OptionBuilder::new()
            .kind(optioncontracts::Type::Call)
            .direction(optioncontracts::Direction::Long)
            .strike(20.0)
            .price(1.0)
            .finish();

        let mut option_manager = PositionManager::new();

        option_manager.add(option_input4);

        // should update quantity
        option_manager.add(option_input2.clone());
        option_manager.add(option_input2);

        option_manager.add(option_input);

        println!("{:#?}", option_manager);

        assert_eq!(
            option_manager.sorted_key_list,
            vec![
                "10::Long::Call".to_string(),
                "15::Short::Call".to_string(),
                "20::Long::Call".to_string(),
            ]
        );
    }

    #[test]
    fn sorted_options() {
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

        let mut option_manager = PositionManager::new();

        option_manager.add(option_input4);
        option_manager.add(option_input3);
        option_manager.add(option_input2);
        option_manager.add(option_input);

        let sorted = option_manager.sorted_elements();

        println!("{:#?}", sorted);

        // assert_eq!(1, 1);
        assert_eq!(sorted[1].quantity, 2);
    }
}
