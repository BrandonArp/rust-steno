use serde::ser::Serialize;

pub trait StenoSerialize {
}

impl <T> StenoSerialize for T where T: Serialize {
}
