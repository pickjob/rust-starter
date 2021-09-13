struct GenericArray<T, const LENGTH: usize> {
    inner: [T; LENGTH]
}

impl<T, const LENGTH: usize> GenericArray<T, LENGTH> {
    const fn last(&self) -> Option<&T> {
        if LENGTH == 0 {
            None
        } else {
            Some(&self.inner[LENGTH - 1])
        }
    }
}
