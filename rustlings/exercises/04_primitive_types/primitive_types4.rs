fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        let r = 1..=3;

        let nice_slice = &a[r];

        assert_eq!([2, 3, 4], nice_slice);
    }
}
