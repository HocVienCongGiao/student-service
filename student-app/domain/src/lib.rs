mod entities;
pub mod ports;
pub mod usecases;

#[derive(PartialEq, Clone)]
pub enum SortDirection {
    Asc,
    Desc,
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 99;
        assert_eq!(result, 99);
    }
}
