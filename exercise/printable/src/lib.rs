pub fn print_info<T: AsRef<str>>(v: &T) {
    println!("{}", v.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer() {
        print_info(&"2".to_string());
    }
}
