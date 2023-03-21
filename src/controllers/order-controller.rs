use crate::models::{book::Book, order::Order, user::User};

pub fn create<'a>(user: &'a User, book: &'a Book) -> Result<Order<'a>, String> {
    let order = book.create_pending_order_for(user);
    Ok(order)
}
