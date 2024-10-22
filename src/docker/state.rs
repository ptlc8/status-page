pub trait HasState {
    fn get_state(&self) -> &String;
}

pub fn get_state<T: HasState>(vec: &Vec<T>) -> String {
    vec.iter()
        .all(|i| i.get_state() == "up")
        .then_some("up")
        .or_else(|| {
            vec.iter()
                .all(|i| i.get_state() == "down")
                .then_some("down")
        })
        .unwrap_or("partial")
        .to_string()
}
