mod back_of_house;
mod front_of_house;

pub use crate::front_of_house::hosting;

// eat at restaurant is a different scope from the front of house
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

fn deliver_order() {}
