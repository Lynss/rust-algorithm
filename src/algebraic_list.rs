
pub enum Cons<T: Clone> {
    Cons(T, Box<Cons<T>>),
    Null,
}

impl<T: Clone> Cons<T> {
    pub fn to_vec(&self) -> Vec<T> {
        match self {
            &Cons::Null => vec![],
            &Cons::Cons(ref head, ref tail) => {
                let mut head = vec![head.clone()];
                head.extend(tail.to_vec());
                head
            }
        }
    }

    pub fn from_iter<I>(it: I) -> Self
        where
            I: IntoIterator<Item = T>,
    {
        let mut it = it.into_iter();
        match it.next() {
            Some(v) => Cons::Cons(v, Box::new(Cons::from_iter(it))),
            None => Cons::Null ,
        }
    }

    pub fn filter<F>(&self, fun: F) -> Self
        where
            F: Fn(&T) -> bool,
    {
        match self {
            Cons::Null => Cons::Null,
            Cons::Cons( head,tail) => {
                if fun(head) {
                    Cons::Cons(head.clone(),Box::new(tail.filter(fun)))
                }else{
                    tail.filter(fun)
                }
            }
        }
    }

    pub fn map<F, S>(&self, fun: F) -> Cons<S>
        where
            F: Fn(T) -> S,
            S: Clone,
    {
        match self {
            Cons::Null => Cons::Null,
            Cons::Cons( head,tail) => {
                Cons::Cons(fun(head.clone()),Box::new(tail.map(fun)) )
            }
        }
    }
}
