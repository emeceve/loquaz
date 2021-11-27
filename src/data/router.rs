use druid::Data;

#[derive(Data, Clone, Copy, PartialEq)]
pub enum Route {
    Chat,
    Contacts,
    Settings,
}

impl Route {
    pub fn goto(&mut self, route: Route) {
        *self = route
    }
}
