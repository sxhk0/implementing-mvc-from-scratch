use super::{
    order::{Order, OrderStatus},
    user::User,
};

#[derive(Debug)]
pub struct Book {
    pub id: u32,
    pub title: String,
}

impl Book {
    pub fn new(id: u32, title: String) -> Book {
        Book { id, title }
    }

    pub fn create_pending_order_for<'a>(&'a self, user: &'a User) -> Order {
        let mut order = Order::new(user, self);
        order.update_status(OrderStatus::Pending);

        order
    }
}
