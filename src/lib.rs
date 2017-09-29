enum Term {
    Con(i16),
    Div(Box<Term>, Box<Term>)
}

fn div(t1: Term, t2: Term) -> Term {
    Term::Div(Box::new(t1), Box::new(t2))
}

fn eval(term: Term) -> i16 {
    use Term::*;
    match term {
        Con(val) => val,
        Div(t1, t2) =>  eval(*t1) / eval(*t2)
    }
}

fn eval2<M: Sized + Monad<i16>>(term: Term) -> M {
    M::unit(1)
}


trait Monad<T>: Sized {
    fn unit(value: T) -> Self where Self: Sized;

    fn bind<F>(&self, func: F) -> Self
        where F: FnOnce(T) -> Self;
}

struct Identity<T> {
    value: T
}

impl <T: Sized + Clone> Monad<T> for Identity<T> {
    fn unit(value: T) -> Self where Self: Sized {
        Identity { value: value }
    }

    fn bind<F>(&self, func: F) -> Identity<T>
        where F: FnOnce(T) -> Self {

        let copy = self.value.clone();
        func(copy)
    }
}

#[cfg(test)]
mod tests {
    use Term::*;
    use eval;
    use div;

    #[test]
    fn it_works() {
        let formula = div(Con(34), div(Con(16), Con(2)));

        let result = eval(formula);
        println!("{}", result);

        assert!(false)
    }
}
