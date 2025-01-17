mod view;

use crate::transport::Request;
use view::View;

#[derive(Debug, Clone)]
pub struct Membership {
    view: View,
}

impl Membership {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn view(&self) -> View {
        self.view.clone()
    }

    pub async fn handle_message(&mut self, req: Request) {
        unimplemented!()
    }

    pub async fn tick(&mut self) {
        unimplemented!()
    }
}
