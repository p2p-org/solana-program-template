use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct State {
}

impl State {
    pub fn len() -> usize {
        Self::default()
            .try_to_vec()
            .expect("State should be serializable")
            .len()
    }
}
