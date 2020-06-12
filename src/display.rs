use std::fmt; // Import the `fmt` module.

// Define a structure named `List` containing a `Vec`.
#[derive(Debug)]
struct List(Vec<i32>);

impl From<i32> for List {
    fn from(f: i32) -> Self {
        List(vec![f])
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        let mut contents = "[".to_string();
        vec.iter().for_each(|&e| {
            contents += &e.to_string();
            contents += ", ";
        });
        contents += "]";

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "{}", contents)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_display() {
        let v = super::List(vec![1, 2, 3]);
        println!("{}", v);

        let l = super::List::from(12);
        println!("{:?}", l);

    }
}

