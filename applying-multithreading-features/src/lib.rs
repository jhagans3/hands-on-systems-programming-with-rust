pub mod args;
pub mod read;
pub mod stats;
pub mod write;

const CHUNK_SIZE: usize = 16 * 1024;

/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/
