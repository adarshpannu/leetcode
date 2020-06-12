// Experimenting with trait objects
// Ref: https://doc.rust-lang.org/book/ch17-02-trait-objects.html

pub trait Draw {
    fn draw(&self);
    // fn describe<T>() {} // <- This will will make this trait ineligible for trait objects
}

// A generic type parameter can only be substituted with one concrete type at a time,
// whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime.

pub struct ScreenGeneric<T: Draw> {
    _widgets: Vec<T>,
}

pub struct ScreenWithTraitObjects<'a> {
    _widgets: Vec<&'a dyn Draw>,
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_draw() {
        assert_eq!(true, true);
    }
}
