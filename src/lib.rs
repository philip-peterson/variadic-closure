use std::marker::PhantomData;

pub struct Closure<A, B, C, D, E, F, RET> {
    variant: ClosureVariant<A, B, C, D, E, F, RET>,
}

fn too_many(a: usize, b: usize) -> String {
    format!(
        "Too many arguments provided to closure (expected: {}, provided: {})",
        a,
        b
    )
}

fn too_few(a: usize, b: usize) -> String {
    format!(
        "Too few arguments provided to closure (expected: {}, provided: {})",
        a,
        b
    )
}

impl<A, B, C, D, E, F, RET> Closure<A, B, C, D, E, F, RET> {
    pub fn call0(&self) -> RET {
        let provided = 0;

        match &self.variant {
            ClosureVariant::Zero(clos) => (*clos.wrapped)(),
            _ => {
                panic!(too_few(self.variant.arity(), provided))
            }
        }
    }

    pub fn call1(&self, a: A) -> RET {
        let provided = 1;

        match &self.variant {
            ClosureVariant::Zero(_) => {
                panic!(too_many(self.variant.arity(), provided))
            }
            ClosureVariant::One(clos) => (*clos.wrapped)(a),
            _ => {
                panic!(too_few(self.variant.arity(), provided))
            }
        }
    }

    pub fn call2(&self, a: A, b: B) -> RET {
        let provided = 2;

        match &self.variant {
            ClosureVariant::Zero(_) |
            ClosureVariant::One(_) => {
                panic!(too_many(self.variant.arity(), provided))
            }
            ClosureVariant::Two(clos) => (*clos.wrapped)(a, b),
            _ => {
                panic!(too_few(self.variant.arity(), provided))
            }
        }
    }

    pub fn call3(&self, a: A, b: B, c: C) -> RET {
        let provided = 3;

        match &self.variant {
            ClosureVariant::Zero(_) |
            ClosureVariant::One(_) |
            ClosureVariant::Two(_) => {
                panic!(too_many(self.variant.arity(), provided))
            }
            ClosureVariant::Three(clos) => (*clos.wrapped)(a, b, c),
            _ => {
                panic!(too_few(self.variant.arity(), provided))
            }
        }
    }

    pub fn call4(&self, a: A, b: B, c: C, d: D) -> RET {
        let provided = 4;

        match &self.variant {
            ClosureVariant::Zero(_) |
            ClosureVariant::One(_) |
            ClosureVariant::Two(_) |
            ClosureVariant::Three(_) => {
                panic!(too_many(self.variant.arity(), provided))
            }
            ClosureVariant::Four(clos) => (*clos.wrapped)(a, b, c, d),
            _ => {
                panic!(too_few(self.variant.arity(), provided))
            }
        }
    }

    pub fn call5(&self, a: A, b: B, c: C, d: D, e: E) -> RET {
        let provided = 5;

        match &self.variant {
            ClosureVariant::Zero(_) |
            ClosureVariant::One(_) |
            ClosureVariant::Two(_) |
            ClosureVariant::Three(_) |
            ClosureVariant::Four(_) => {
                panic!(too_many(self.variant.arity(), provided))
            }
            ClosureVariant::Five(clos) => (*clos.wrapped)(a, b, c, d, e),
            _ => {
                panic!(too_few(self.variant.arity(), provided))
            }
        }
    }

    pub fn call6(&self, a: A, b: B, c: C, d: D, e: E, f: F) -> RET {
        let provided = 6;

        match &self.variant {
            ClosureVariant::Zero(_) |
            ClosureVariant::One(_) |
            ClosureVariant::Two(_) |
            ClosureVariant::Three(_) |
            ClosureVariant::Four(_) |
            ClosureVariant::Five(_) => {
                panic!(too_many(self.variant.arity(), provided))
            }
            ClosureVariant::Six(clos) => (*clos.wrapped)(a, b, c, d, e, f),
            _ => {
                panic!(too_few(self.variant.arity(), provided))
            }
        }
    }
}

pub enum ClosureVariant<A, B, C, D, E, F, RET> {
    Zero(Closure0<RET>),
    One(Closure1<A, RET>),
    Two(Closure2<A, B, RET>),
    Three(Closure3<A, B, C, RET>),
    Four(Closure4<A, B, C, D, RET>),
    Five(Closure5<A, B, C, D, E, RET>),
    Six(Closure6<A, B, C, D, E, F, RET>),
}

impl<A, B, C, D, E, F, RET> ClosureVariant<A, B, C, D, E, F, RET> {
    fn arity(&self) -> usize {
        match &self {
            ClosureVariant::Zero(_) => 0,
            ClosureVariant::One(_) => 1,
            ClosureVariant::Two(_) => 2,
            ClosureVariant::Three(_) => 3,
            ClosureVariant::Four(_) => 4,
            ClosureVariant::Five(_) => 5,
            ClosureVariant::Six(_) => 6,
        }
    }
}

pub struct Closure0<RET> {
    wrapped: Box<dyn Fn() -> RET>,
    _ret: PhantomData<RET>,
}

pub struct Closure1<A, RET> {
    wrapped: Box<dyn Fn(A) -> RET>,
    _a: PhantomData<A>,
    _ret: PhantomData<RET>,
}

pub struct Closure2<A, B, RET> {
    wrapped: Box<dyn Fn(A, B) -> RET>,
    _a: PhantomData<A>,
    _b: PhantomData<B>,
    _ret: PhantomData<RET>,
}

pub struct Closure3<A, B, C, RET> {
    wrapped: Box<dyn Fn(A, B, C) -> RET>,
    _a: PhantomData<A>,
    _b: PhantomData<B>,
    _c: PhantomData<C>,
    _ret: PhantomData<RET>,
}

pub struct Closure4<A, B, C, D, RET> {
    wrapped: Box<dyn Fn(A, B, C, D) -> RET>,
    _a: PhantomData<A>,
    _b: PhantomData<B>,
    _c: PhantomData<C>,
    _d: PhantomData<D>,
    _ret: PhantomData<RET>,
}

pub struct Closure5<A, B, C, D, E, RET> {
    wrapped: Box<dyn Fn(A, B, C, D, E) -> RET>,
    _a: PhantomData<A>,
    _b: PhantomData<B>,
    _c: PhantomData<C>,
    _d: PhantomData<D>,
    _e: PhantomData<E>,
    _ret: PhantomData<RET>,
}

pub struct Closure6<A, B, C, D, E, F, RET> {
    wrapped: Box<dyn Fn(A, B, C, D, E, F) -> RET>,
    _a: PhantomData<A>,
    _b: PhantomData<B>,
    _c: PhantomData<C>,
    _d: PhantomData<D>,
    _e: PhantomData<E>,
    _f: PhantomData<F>,
    _ret: PhantomData<RET>,
}

#[macro_export]
macro_rules! variadic_closure {
    ( fn $fn:ident ( ) -> $ret:ty $body:block ) => {
        $crate::Closure {
            variant: $crate::ClosureVariant::Zero::<(), (), (), (), (), (), $ret>($crate::Closure0 {
                wrapped: Box::new(|| -> $ret { $body }),
                _ret: ::std::marker::PhantomData
            }),
        }
    };
    ( fn $fn:ident ( $arg1: ident : $argty1:ty  ) -> $ret:ty $body:block ) => {

    };
}

#[cfg(test)]
mod tests {
    use cool_asserts;
    #[test]
    fn it_works() {
        let stored_closure = variadic_closure! {
            fn foo() -> isize {
                3
            }
        };
        let result = stored_closure.call0();
        assert_eq!(result, 3);
    }

    #[test]
    #[should_panic]
    fn test_closure0_panics_too_many_args_1() {
        
    }
}

