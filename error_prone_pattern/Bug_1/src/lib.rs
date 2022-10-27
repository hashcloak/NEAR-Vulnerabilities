use near_sdk::collections::UnorderedMap;
use near_sdk::{near_bindgen, PanicOnDefault};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    m: UnorderedMap<u8, String>,
}

// Bug 1: Should not replace any collections without clearing state, this will reset any
// metadata, such as the number of elements, leading to bugs. If you replace the collection
// with something with a different prefix, it will be functional, but you will lose any
// previous data and the old values will not be removed from storage.
        

#[near_bindgen]
impl Contract {

    #[init]
    pub fn new() -> Self {
        Self { m: UnorderedMap::<u8, String>::new(b"m")}
    }

    pub fn ep_clear_state(&mut self) -> bool {
        // let mut m = UnorderedMap::<u8, String>::new(b"m");
        self.m.insert(&1, &"test".to_string());
        assert_eq!(self.m.len(), 1, "we are testing collections with {} and {}", self.m.len(), 1);
        assert_eq!(self.m.get(&1), Some("test".to_string()), "we are testing collections with {:#?} and {:#?}", self.m.get(&1), Some("test".to_string()));

        return self.m.get(&1) == Some("test".to_string());
    }

    // pub fn ep_re_initialize(&mut self) {
    //     self.m = UnorderedMap::new(b"m");
    // }

    pub fn ep_check_state(&mut self) -> (bool, bool) {
        self.m = UnorderedMap::new(b"m");
        assert!(self.m.is_empty(), "is_empty value: {}", self.m.is_empty());
        assert_eq!(self.m.get(&1), Some("test".to_string()), "we are testing collections with {:#?} and {:#?}", self.m.get(&1), Some("test".to_string()));
        return (self.m.is_empty(), self.m.get(&1) == Some("test".to_string()));
    }
}

