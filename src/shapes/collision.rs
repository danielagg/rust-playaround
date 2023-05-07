pub trait Collidable<T> {
    fn collide(&self, other: &T) -> bool;
    fn collides(&self, other: &[T]) -> bool {
        for item in other {
            if self.collide(item) {
                return true;
            }
        }

        return false;
    }
}
