pub fn get_active_class(is_active: bool) -> String {
    if is_active {
        return "Active".to_owned();
    }

    "".to_owned()
}
