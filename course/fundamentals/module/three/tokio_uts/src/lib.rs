async fn double(n: u64) -> u64 {
    n * 2
}

#[cfg(test)]
mod test {
	use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(4, 4)
    }

    #[test]
    fn will_compile() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        assert_eq!(rt.block_on(double(2)), 4)
    }

	// this macro allows for the async testing using tokio
	// it actually does the same thing as we did above
    #[tokio::test(flavor = "multi_thread")] 
    async fn using_tokio() {
        assert_eq!(double(2).await, 4)
    }
}
