fn main() {
    println!("privet oleh");
    println!("privet nikita");
    println!("privet oleh");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_pass() {
        let result = 2 - 2;
        assert_eq!(result, 0);
    }
}
