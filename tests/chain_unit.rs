#![allow(dead_code, unused_imports)]

#[cfg(test)]
mod tests {
    use ranger::chain::ContractInstrument;
    use ranger::chain::Moment;
    use ranger::chain::OptionChain;
    use ranger::chain::Position;

    use ranger::utils;

    #[test]
    fn new_chain() {
        let mut call_option_chain = OptionChain::default();

        // build some silly chain
        for strike in 1..6 {
            call_option_chain.add(Position {
                moment: Moment {
                    timestamp: 1,
                    price: 1.0,
                },
                contract: ContractInstrument {
                    optiontype: optioncontracts::Type::Call,
                    strike: (strike * 5) as f64,
                },
            });
        }

        let _call_sorted = call_option_chain.sorted_elements();

        // this builds all 3 option combination
        let combinations = call_option_chain.get_all_n_options_contract(3);

        for opts in combinations.iter() {
            let mut contracts = opts.clone();

            // duplicate the middle option (make a butterfly)
            contracts[1].direction = optioncontracts::Direction::Short;
            contracts.insert(1, contracts[1].clone());

            // tell us whats happening
            println!("{}", "");
            println!(
                "Strikes: {} {} {}",
                contracts[0].strike, contracts[1].strike, contracts[3].strike,
            );

            // calc at current price
            let current_price = 20.0;
            let total: f64 = contracts
                .iter()
                .map(|contract| optioncontracts::execute_option(&contract, current_price))
                .sum();
            println!("@ $20: {:?}", total);

            // now build as points and find zeros
            let contract_values = utils::calculate_contract_values(contracts);
            let lines_through_zero = utils::get_lines_through_zero(contract_values);
            let zero_intersections: Vec<f64> = utils::get_zero_intersections(lines_through_zero)
                .iter()
                .map(|x| x.0)
                .collect();

            println!("Intersections: {:?}", zero_intersections);
            println!("{}", "");
        }

        assert_eq!(1, 0);
    }

    // #[test]
    // fn new_chain_with_update() {
    //     let mut call_option_chain = OptionChain::default();

    //     // build some silly chain
    //     for strike in 1..6 {
    //         call_option_chain.add(Position {
    //             moment: Moment {
    //                 timestamp: 1,
    //                 price: 1.0,
    //             },
    //             contract: ContractInstrument {
    //                 optiontype: optioncontracts::Type::Call,
    //                 strike: (strike * 5) as f64,
    //             },
    //         });
    //     }

    //     // let mut put_option_chain = OptionChain::new();

    //     // for strike in 1..5 {
    //     //     put_option_chain.add(Position {
    //     //         moment: Moment {
    //     //             timestamp: 1,
    //     //             price: 1.0,
    //     //         },
    //     //         contract: ContractInstrument {
    //     //             optiontype: optioncontracts::Type::Put,
    //     //             strike: (strike * 5) as f64,
    //     //         },
    //     //     });
    //     // }

    //     let _call_sorted = call_option_chain.sorted_elements();
    //     // let put_sorted = put_option_chain.sorted_elements();

    //     // this builds all 3 option combination
    //     let combinations = call_option_chain.get_all_n_options_contract(3);

    //     for opts in combinations.iter() {
    //         let mut contracts = opts.clone();

    //         // duplicate the middle option (make a butterfly)
    //         contracts[1].direction = optioncontracts::Direction::Short;
    //         contracts.insert(1, contracts[1].clone());

    //         // tell us whats happening
    //         println!("{}", "");
    //         println!(
    //             "Strikes: {} {} {}",
    //             contracts[0].strike, contracts[1].strike, contracts[3].strike,
    //         );

    //         // calc at current price
    //         let current_price = 20.0;
    //         let total: f64 = contracts
    //             .iter()
    //             .map(|contract| optioncontracts::execute_option(&contract, current_price))
    //             .sum();
    //         println!("@ $20: {:?}", total);

    //         // now build as points and find zeros
    //         let contract_values = utils::calculate_contract_values(contracts);
    //         let lines_through_zero = utils::get_lines_through_zero(contract_values);
    //         let zero_intersections: Vec<f64> = utils::get_zero_intersections(lines_through_zero)
    //             .iter()
    //             .map(|x| x.0)
    //             .collect();

    //         println!("Intersections: {:?}", zero_intersections);
    //         println!("{}", "");
    //     }
    //     // call_option_chain.get_all_n_items(4);
    //     // println!("{:#?}", put_sorted);

    //     assert_eq!(1, 0);
    // }
}
