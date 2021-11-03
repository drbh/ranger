use optioncontracts::OptionContract;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Position {
    pub quantity: u32,
    pub contract: OptionContract,
}

#[derive(Debug)]
pub struct PositionManager {
    pub option_map: HashMap<String, Position>,
    pub sorted_key_list: Vec<String>,
}

impl PositionManager {
    pub fn new() -> PositionManager {
        PositionManager {
            option_map: HashMap::new(),
            sorted_key_list: vec![],
        }
    }

    pub fn add(&mut self, input: OptionContract) {
        // 10::Long
        let key = format!(
            "{}::{:?}::{:?}",
            input.strike, input.direction, input.optiontype
        );

        match self.sorted_key_list.binary_search(&key) {
            Ok(_pos) => {
                // element already in vector @ `pos`
                // we should update the quantity

                // should always be true since we have item in our map
                if let Some(position) = self.option_map.get_mut(&key) {
                    let mut new_position = position.clone();
                    new_position.quantity += 1;
                    *position = new_position;
                }
            }
            Err(pos) => {
                // we should insert into map
                self.sorted_key_list.insert(pos, key.clone());

                // add option to map
                self.option_map.insert(
                    key,
                    Position {
                        quantity: 1,
                        contract: input,
                    },
                );
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
}
