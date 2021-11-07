// use optioncontracts::OptionContract;
use itertools::Itertools;
use std::collections::HashMap;
use std::convert::From;

#[derive(Debug, Clone)]
pub struct ContractInstrument {
    pub optiontype: optioncontracts::Type,
    pub strike: f64,
}

#[derive(Debug, Clone)]
pub struct Moment {
    pub timestamp: i32,
    pub price: f64,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub moment: Moment,
    pub contract: ContractInstrument,
}

#[derive(Debug, Clone)]
pub struct OptionChain {
    pub option_map: HashMap<String, Position>,
    pub sorted_key_list: Vec<String>,
}

impl From<&Position> for optioncontracts::OptionContract {
    fn from(item: &Position) -> Self {
        optioncontracts::OptionContract {
            optiontype: item.contract.optiontype.clone(),
            direction: optioncontracts::Direction::Long,
            strike: item.contract.strike,
            price: item.moment.price,
        }
    }
}

impl OptionChain {
    pub fn default() -> Self {
        Self {
            option_map: HashMap::new(),
            sorted_key_list: vec![],
        }
    }

    // upsert option in chain
    pub fn upsert(&mut self, input: Position) {
        // 10::Long
        let key = format!("{}", input.contract.strike);

        match self.sorted_key_list.binary_search_by(|strike| {
            strike
                .parse::<f64>()
                .unwrap_or_default()
                .partial_cmp(&input.contract.strike)
                .unwrap_or(std::cmp::Ordering::Less)
        }) {
            Ok(_pos) => {
                // should always be true since we have item in our map
                if let Some(position) = self.option_map.get_mut(&key) {
                    *position = input;
                }
            }
            Err(pos) => {
                // we should insert into map
                self.sorted_key_list.insert(pos, key.clone());

                // add option to map
                self.option_map.insert(key, input);
            }
        }
    }
    // replaces option in chain
    pub fn replace(&mut self, input: Position) {
        // 10::Long
        let key = format!("{}", input.contract.strike);

        match self.sorted_key_list.binary_search_by(|strike| {
            strike
                .parse::<f64>()
                .unwrap_or_default()
                .partial_cmp(&input.contract.strike)
                .unwrap_or(std::cmp::Ordering::Less)
        }) {
            Ok(_pos) => {
                // should always be true since we have item in our map
                if let Some(position) = self.option_map.get_mut(&key) {
                    *position = input;
                }
            }
            Err(_pos) => {
                unreachable!()
            }
        }
    }

    pub fn add(&mut self, input: Position) {
        // 10::Long
        let key = format!("{}", input.contract.strike);

        match self.sorted_key_list.binary_search_by(|strike| {
            strike
                .parse::<f64>()
                .unwrap_or_default()
                .partial_cmp(&input.contract.strike)
                .unwrap_or(std::cmp::Ordering::Less)
        }) {
            Ok(_pos) => {
                // element already in vector @ `pos`
                // we should update the quantity

                // should always be true since we have item in our map
                if let Some(position) = self.option_map.get_mut(&key) {
                    let new_position = position.clone();
                    // new_position.moment.price += 1.0;
                    *position = new_position;
                }
            }
            Err(pos) => {
                // we should insert into map
                self.sorted_key_list.insert(pos, key.clone());

                // add option to map
                self.option_map.insert(key, input);
            }
        }
    }

    pub fn sorted_elements(&self) -> Vec<&Position> {
        self.sorted_key_list
            .iter()
            .map(|key| self.option_map.get(key))
            .filter_map(|e| e)
            .collect()
    }

    pub fn sorted_elements_as_contracts(&self) -> Vec<optioncontracts::OptionContract> {
        self.sorted_key_list
            .iter()
            .map(|key| self.option_map.get(key))
            .filter_map(|e| e)
            .map(|y|  optioncontracts::OptionContract::from(y) )
            .collect()
    }

    // pub fn get_all_n_items(&self) -> Vec<&Position> {
    pub fn get_all_n_items(&self, n: usize) -> Vec<Vec<&str>> {
        self.sorted_key_list
            .iter()
            .map(|s| &**s)
            .combinations(n)
            .collect()
    }

    // pub fn get_all_n_items(&self) -> Vec<&Position> {
    pub fn get_all_n_options(&self, n: usize) -> Vec<Vec<&Position>> {
        self.sorted_key_list
            .iter()
            .map(|s| &**s)
            .combinations(n)
            .map(|key| {
                key.iter()
                    .map(|x| self.option_map.get(x.clone()))
                    .filter_map(|e| e)
                    .collect::<Vec<&Position>>()
            })
            .collect()
    }

    pub fn get_all_n_options_contract(
        &self,
        n: usize,
    ) -> Vec<Vec<optioncontracts::OptionContract>> {
        self.sorted_key_list
            .iter()
            .map(|s| &**s)
            .combinations(n)
            .map(|key| {
                key.iter()
                    .map(|x| self.option_map.get(x.clone()))
                    .filter_map(|e| e)
                    .map(|y| optioncontracts::OptionContract::from(y))
                    .collect::<Vec<optioncontracts::OptionContract>>()
            })
            .collect()
    }
}
