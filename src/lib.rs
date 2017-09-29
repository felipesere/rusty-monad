extern crate num_traits;

use num_traits::Num;

enum Term<VALUE> {
    Con(VALUE),
    Div(Box<Term<VALUE>>, Box<Term<VALUE>>)
}

fn div<V>(t1: Term<V>, t2: Term<V>) -> Term<V> {
    Term::Div(Box::new(t1), Box::new(t2))
}

trait Monad<T>: Sized {
    fn unit(value: T) -> Self where Self: Sized;

    fn bind<F>(&self, func: F) -> Self
        where F: FnOnce(T) -> Self;
}

fn eval<V: Num>(term: Term<V>) -> V {
    use Term::*;
    match term {
        Con(val) => val,
        Div(t1, t2) =>  eval(*t1) / eval(*t2)
    }
}

fn eval_m<V, M>(term: Term<V>) -> M
  where M: Sized + Monad<V>, V: Num {
    use Term::*;
    match term {
        Con(val) => M::unit(val),
        Div(t1, t2) => {
            let left = *t1;
            let right = *t2;
            eval_m::<V, M>(left).bind(|a| eval_m::<V, M>(right).bind(|b| {
                if b == V::zero() { M::unit(V::zero()) }
                else  { M::unit( a / b ) }
            }))
        }
    }
}

fn eval_i<V: Num + Clone>(term: Term<V>) -> Identity<V> {
    use Term::*;
    match term {
        Con(val) => Identity::unit(val),
        Div(t1, t2) => {
            let x = *t1;
            let y = *t2;
            eval_i(x).bind( |a| eval_i(y).bind( |b| Identity::unit(a / b)))
        }
    }
}

#[derive(Debug)]
struct Identity<T> {
    value: T
}

impl <T: Sized + Clone> Monad<T> for Identity<T> {
    fn unit(value: T) -> Self where Self: Sized {
        Identity { value: value }
    }

    fn bind<F>(&self, func: F) -> Identity<T> where F: FnOnce(T) -> Self {
        func(self.value.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Term::*;

    #[test]
    fn it_works() {
        let formula = div(Con(34), div(Con(16), Con(2)));

        let result = eval(formula);
        println!("{}", result);

        assert_eq!(result, 4)
    }

    #[test]
    fn identity_monadic() {
        let formula = div(Con(34), div(Con(16), Con(2)));

        let result = eval_i(formula);
        println!("{:?}", result);

        assert_eq!(result.value, 4)
    }

    #[test]
    fn general_monadic() {
        let formula = div(Con(34), div(Con(16), Con(2)));

        let result: Identity<i16> = eval_m(formula);
        println!("{:?}", result);

        assert_eq!(result.value, 4)
    }

}
