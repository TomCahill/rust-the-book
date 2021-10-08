mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}

    fn seat_at_table() {}
  }

  mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
  }
}

mod back_of_house {
  pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
  }

  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        seasonal_fruit: String::from("blueberries"),
      }
    }
  }

  pub enum Appetizer {
    Soup,
    Salad,
  }

  fn fix_incorrect_order() {
    cook_order();
    super::serve_order();
  }

  fn cook_order() {}
}

pub fn eat_at_restaurant() {
  crate::front_of_house::hosting::add_to_waitlist();

  front_of_house::hosting::add_to_waitlist();

  let mut meal = back_of_house::Breakfast::summer("Rye");
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please and what ever the seasonal fruit is!", meal.toast);

  // NOTE: Can't do this as the seasonal fruit field is private we can't even read it!
  // meal.seasonal_fruit = String::from("Foo");
  // println!("I'd like {} toast please with {} on top!", meal.toast, meal.seasonal_fruit);

  // In contrast, if we make an enum public, all of its variants are then public.
  let order1 = back_of_house::Appetizer::Soup;
  let order2 = back_of_house::Appetizer::Salad;
}

fn serve_order() {

}