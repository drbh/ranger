#![allow(dead_code, unused_imports)]

#[cfg(test)]
mod tests {
    use ranger::chain::ContractInstrument;
    use ranger::chain::Moment;
    use ranger::chain::OptionChain;
    use ranger::chain::Position;

    #[test]
    fn new_chain() {
        let mut call_option_chain = OptionChain::default();

        for strike in 1..5 {
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

        // let mut put_option_chain = OptionChain::new();

        // for strike in 1..5 {
        //     put_option_chain.add(Position {
        //         moment: Moment {
        //             timestamp: 1,
        //             price: 1.0,
        //         },
        //         contract: ContractInstrument {
        //             optiontype: optioncontracts::Type::Put,
        //             strike: (strike * 5) as f64,
        //         },
        //     });
        // }

        let call_sorted = call_option_chain.sorted_elements();
        // let put_sorted = put_option_chain.sorted_elements();

        println!("{:#?}", call_option_chain);

        // call_option_chain.get_all_n_items(2);
        call_option_chain.get_all_n_items(3);
        // call_option_chain.get_all_n_items(4);
        // println!("{:#?}", put_sorted);

        assert_eq!(1, 0);
    }
}
