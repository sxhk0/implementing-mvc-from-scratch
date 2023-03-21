use controllers::order;
use models::{book::Book, user::User};

mod controllers;
mod models;
mod views;

fn main() {
    let user = User::new(1, "shwuy".to_owned());
    let book = Book::new(1, "RESTful Web Services".to_owned());

    let order_request = order::create(&user, &book);

    match order_request {
        Ok(order) => {
            views::order::show(&user, &book, &order);
        }
        Err(error) => {
            println!("Cannot order this book!");
            eprintln!("{}", error)
        }
    };
}
