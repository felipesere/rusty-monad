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


trait Monad<T> {
    fn unit(value: T) -> Self where Self: Sized;

    fn bind<U, X>(&self, func: (FnOnce(T, U))) -> U where U: Sized + Monad<X>;
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
