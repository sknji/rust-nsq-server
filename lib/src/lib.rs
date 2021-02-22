pub fn hello() {
    println!("Hello, world lib!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!("Hello, world!", hello());
    }
}
