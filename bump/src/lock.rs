pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> AsRef<spin::Mutex<A>> for Locked<A> {
    fn as_ref(&self) -> &spin::Mutex<A> {
        &self.inner
    }
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Self {
            inner: spin::Mutex::new(inner),
        }
    }
}
