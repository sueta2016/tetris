fn main() {
    println!("privet oleh");
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_fails() {
        let result = 2 - 2;
        assert_eq!(result, 4);
    }
}
