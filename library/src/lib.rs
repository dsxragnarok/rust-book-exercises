// Rules of item visibility
// * If an item is public, it can be accessed through any of its parent modules
// * If an item is private, it can be accessed only by its immediate parent module and any of the parent's child modules

mod outermost {
    pub fn middle_function() {}
    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {
            ::outermost::middle_secret_function()
        }

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
