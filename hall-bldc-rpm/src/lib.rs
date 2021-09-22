pub mod app;
pub mod commutation;
pub mod device;
pub mod domain;
pub mod values;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
