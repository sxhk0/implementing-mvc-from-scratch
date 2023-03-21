use crate::models::{book::Book, order::Order, user::User};

pub fn show(user: &User, book: &Book, order: &Order) {
    println!("{} ordered {}.", user.name, book.title);
    println!("{:?}", order);
}
