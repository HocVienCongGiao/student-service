pub fn test_func() {
    println!("hello");
}

pub mod boundaries;
mod entity;
pub mod interactors;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 99;
        assert_eq!(result, 99);
    }
}
