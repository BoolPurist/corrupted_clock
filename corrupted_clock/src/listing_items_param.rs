use crate::{
    cli_args::{GetClockArgs, ListArgs},
    AtLeastOne,
};

#[derive(Debug, Clone, Default)]
pub struct ListingItemsParams(Option<AtLeastOne>);

impl ListingItemsParams {
    pub fn column_num(&self) -> Option<AtLeastOne> {
        self.0
    }
}

impl From<&GetClockArgs> for ListingItemsParams {
    fn from(value: &GetClockArgs) -> Self {
        Self(value.column_num())
    }
}

impl From<&ListArgs> for ListingItemsParams {
    fn from(value: &ListArgs) -> Self {
        Self(value.colums_num())
    }
}
