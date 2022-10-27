use near_sdk::collections::UnorderedMap;
use near_sdk::{near_bindgen, PanicOnDefault};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    m: UnorderedMap<u8, String>,
    m2: UnorderedMap<u8, String>,
}

// Bug 2: Should not use the same prefix as another collection
// or there will be unexpected side effects.

#[near_bindgen]
impl Contract {

    #[init]
    pub fn new() -> Self {
        Self { m: UnorderedMap::<u8, String>::new(b"m"), m2: UnorderedMap::<u8, String>::new(b"m")}
    }

    pub fn ep_add_state(&mut self) -> bool {
        self.m.insert(&1, &"test".to_string());
        assert_eq!(self.m.len(), 1, "we are testing collections with {} and {}", self.m.len(), 1);
        assert_eq!(self.m.get(&1), Some("test".to_string()), "we are testing collections with {:#?} and {:#?}", self.m.get(&1), Some("test".to_string()));

        return self.m.get(&1) == Some("test".to_string());
    }


    pub fn ep_check_state(&mut self) -> (bool, bool) {
        // self.m = UnorderedMap::new(b"m");
        assert!(self.m2.is_empty(), "is_empty value: {}", self.m2.is_empty());
        assert_eq!(self.m2.get(&1), Some("test".to_string()), "we are testing collections with {:#?} and {:#?}", self.m2.get(&1), Some("test".to_string()));
        return (self.m2.is_empty(), self.m2.get(&1) == Some("test".to_string()));
    }
}

