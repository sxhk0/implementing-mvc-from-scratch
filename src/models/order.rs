use super::{book::Book, user::User};

#[derive(Debug)]
pub enum OrderStatus {
    None,
    Pending,
    // Delivered,
}

#[derive(Debug)]
pub struct Order<'a> {
    pub user: &'a User,
    pub book: &'a Book,
    pub status: OrderStatus,
}

impl Order<'_> {
    pub fn new<'a>(user: &'a User, book: &'a Book) -> Order<'a> {
        Order {
            user,
            book,
            status: OrderStatus::None,
        }
    }

    pub fn update_status(&mut self, status: OrderStatus) {
        self.status = status;
    }
}
