pub struct PoisonError<T>(T);

impl<T> PoisonError<T> {
    pub fn new(i: T) -> Self
    {
        Self(i)
    }

    pub fn into_inner(self) -> T
    {
        self.0
    }

    pub fn get_ref(&self) -> &T
    {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut T
    {
        &mut self.0
    }
}
