use crate::app::App;
use crate::open::Open;

#[derive(Debug)]
pub enum Action {
    OpenApp(App),
}

impl Action {
    pub fn execute(&self) {
        match self {
            Action::OpenApp(app) => app.open().unwrap(),
        }
    }
}
