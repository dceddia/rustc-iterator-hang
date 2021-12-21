trait OnChangeExt<Cond, F>: Iterator
where
    Self: Sized,
    Cond: Fn(Option<Self::Item>) -> bool,
    F: Fn((Option<Self::Item>, Option<Self::Item>)) -> Option<Self::Item>,
{
    fn on_change(self, cond: Cond, f: F) -> OnChange<Self, Cond, F>;
}

impl<I, Cond, F> OnChangeExt<Cond, F> for I
where
    I: Iterator,
{
    fn on_change(self, cond: Cond, f: F) -> OnChange<Self, Cond, F> {
        let prev = self.next();
        OnChange {
            prev,
            prev_cond: cond(prev),
            cond,
            f,
        }
    }
}

struct OnChange<T, Cond, F>
where
    F: Fn((Option<T>, Option<T>)) -> Option<T>,
{
    prev: Option<T>,
    cond: Cond,
    f: F,
}

impl<T, Cond, F> Iterator for OnChange<T, Cond, F>
where
    F: Fn((Option<Self::Item>, Option<Self::Item>)) -> Option<Self::Item>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
