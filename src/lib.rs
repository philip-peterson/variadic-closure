pub struct Closure<A, B, C, D, E, F, G, H, RET> {
    variant: ClosureVariant<A, B, C, D, E, F, G, H, RET>,
}

fn too_many(a: usize, b: usize) -> String {
    format!(
        "Too many arguments provided to closure (expected: {}, provided: {})",
        a, b
    )
}

fn too_few(a: usize, b: usize) -> String {
    format!(
        "Too few arguments provided to closure (expected: {}, provided: {})",
        a, b
    )
}

impl<A, B, C, D, E, F, G, H, RET> Closure<A, B, C, D, E, F, G, H, RET> {
    pub fn arity(&self) -> usize {
        self.variant.arity()
    }

    pub fn call0(&self) -> RET {
        let provided = 0;

        match &self.variant {
            ClosureVariant::Zero(clos) => (*clos.wrapped)(),
            _ => {
                panic!("{}", too_few(self.variant.arity(), provided))
            }
        }
    }

    pub fn call1(&self, a: A) -> RET {
        let provided = 1;

        match &self.variant {
            ClosureVariant::Zero(_) => {
                panic!("{}", too_many(self.variant.arity(), provided))
            }
            ClosureVariant::One(clos) => (*clos.wrapped)(a),
            _ => {
                panic!("{}", too_few(self.variant.arity(), provided))
            }
        }
    }

    pub fn call2(&self, a: A, b: B) -> RET {
        let provided = 2;

        match &self.variant {
            ClosureVariant::Zero(_) | ClosureVariant::One(_) => {
                panic!("{}", too_many(self.variant.arity(), provided))
            }
            ClosureVariant::Two(clos) => (*clos.wrapped)(a, b),
            _ => {
                panic!("{}", too_few(self.variant.arity(), provided))
            }
        }
    }

    pub fn call3(&self, a: A, b: B, c: C) -> RET {
        let provided = 3;

        match &self.variant {
            ClosureVariant::Zero(_) | ClosureVariant::One(_) | ClosureVariant::Two(_) => {
                panic!("{}", too_many(self.variant.arity(), provided))
            }
            ClosureVariant::Three(clos) => (*clos.wrapped)(a, b, c),
            _ => {
                panic!("{}", too_few(self.variant.arity(), provided))
            }
        }
    }

    pub fn call4(&self, a: A, b: B, c: C, d: D) -> RET {
        let provided = 4;

        match &self.variant {
            ClosureVariant::Zero(_)
            | ClosureVariant::One(_)
            | ClosureVariant::Two(_)
            | ClosureVariant::Three(_) => {
                panic!("{}", too_many(self.variant.arity(), provided))
            }
            ClosureVariant::Four(clos) => (*clos.wrapped)(a, b, c, d),
            _ => {
                panic!("{}", too_few(self.variant.arity(), provided))
            }
        }
    }

    pub fn call5(&self, a: A, b: B, c: C, d: D, e: E) -> RET {
        let provided = 5;

        match &self.variant {
            ClosureVariant::Zero(_)
            | ClosureVariant::One(_)
            | ClosureVariant::Two(_)
            | ClosureVariant::Three(_)
            | ClosureVariant::Four(_) => {
                panic!("{}", too_many(self.variant.arity(), provided))
            }
            ClosureVariant::Five(clos) => (*clos.wrapped)(a, b, c, d, e),
            _ => {
                panic!("{}", too_few(self.variant.arity(), provided))
            }
        }
    }

    pub fn call6(&self, a: A, b: B, c: C, d: D, e: E, f: F) -> RET {
        let provided = 6;

        match &self.variant {
            ClosureVariant::Zero(_)
            | ClosureVariant::One(_)
            | ClosureVariant::Two(_)
            | ClosureVariant::Three(_)
            | ClosureVariant::Four(_)
            | ClosureVariant::Five(_) => {
                panic!("{}", too_many(self.variant.arity(), provided))
            }
            ClosureVariant::Six(clos) => (*clos.wrapped)(a, b, c, d, e, f),
            _ => {
                panic!("{}", too_few(self.variant.arity(), provided))
            }
        }
    }

    pub fn call7(&self, a: A, b: B, c: C, d: D, e: E, f: F, g: G) -> RET {
        let provided = 7;

        match &self.variant {
            ClosureVariant::Zero(_)
            | ClosureVariant::One(_)
            | ClosureVariant::Two(_)
            | ClosureVariant::Three(_)
            | ClosureVariant::Four(_)
            | ClosureVariant::Five(_) => {
                panic!("{}", too_many(self.variant.arity(), provided))
            }
            ClosureVariant::Six(clos) => (*clos.wrapped)(a, b, c, d, e, f),
            ClosureVariant::Seven(clos) => (*clos.wrapped)(a, b, c, d, e, f, g),
            _ => {
                panic!("{}", too_few(self.variant.arity(), provided))
            }
        }
    }

    pub fn call8(&self, a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H) -> RET {
        let provided = 8;

        match &self.variant {
            ClosureVariant::Zero(_)
            | ClosureVariant::One(_)
            | ClosureVariant::Two(_)
            | ClosureVariant::Three(_)
            | ClosureVariant::Four(_)
            | ClosureVariant::Five(_) => {
                panic!("{}", too_many(self.variant.arity(), provided))
            }
            ClosureVariant::Six(clos) => (*clos.wrapped)(a, b, c, d, e, f),
            ClosureVariant::Seven(clos) => (*clos.wrapped)(a, b, c, d, e, f, g),
            ClosureVariant::Eight(clos) => (*clos.wrapped)(a, b, c, d, e, f, g, h),
        }
    }

}

pub enum ClosureVariant<A, B, C, D, E, F, G, H, RET> {
    Zero(Closure0<RET>),
    One(Closure1<A, RET>),
    Two(Closure2<A, B, RET>),
    Three(Closure3<A, B, C, RET>),
    Four(Closure4<A, B, C, D, RET>),
    Five(Closure5<A, B, C, D, E, RET>),
    Six(Closure6<A, B, C, D, E, F, RET>),
    Seven(Closure7<A, B, C, D, E, F, G, RET>),
    Eight(Closure8<A, B, C, D, E, F, G, H, RET>),
}

impl<A, B, C, D, E, F, G, H, RET> ClosureVariant<A, B, C, D, E, F, G, H, RET> {
    pub fn arity(&self) -> usize {
        match &self {
            ClosureVariant::Zero(_) => 0,
            ClosureVariant::One(_) => 1,
            ClosureVariant::Two(_) => 2,
            ClosureVariant::Three(_) => 3,
            ClosureVariant::Four(_) => 4,
            ClosureVariant::Five(_) => 5,
            ClosureVariant::Six(_) => 6,
            ClosureVariant::Seven(_) => 7,
            ClosureVariant::Eight(_) => 8,
        }
    }
}

pub struct Closure0<RET> {
    wrapped: Box<dyn Fn() -> RET>,
}

pub struct Closure1<A, RET> {
    wrapped: Box<dyn Fn(A) -> RET>,
}

pub struct Closure2<A, B, RET> {
    wrapped: Box<dyn Fn(A, B) -> RET>,
}

pub struct Closure3<A, B, C, RET> {
    wrapped: Box<dyn Fn(A, B, C) -> RET>,
}

pub struct Closure4<A, B, C, D, RET> {
    wrapped: Box<dyn Fn(A, B, C, D) -> RET>,
}

pub struct Closure5<A, B, C, D, E, RET> {
    wrapped: Box<dyn Fn(A, B, C, D, E) -> RET>,
}

pub struct Closure6<A, B, C, D, E, F, RET> {
    wrapped: Box<dyn Fn(A, B, C, D, E, F) -> RET>,
}

pub struct Closure7<A, B, C, D, E, F, G, RET> {
    wrapped: Box<dyn Fn(A, B, C, D, E, F, G) -> RET>,
}

pub struct Closure8<A, B, C, D, E, F, G, H, RET> {
    wrapped: Box<dyn Fn(A, B, C, D, E, F, G, H) -> RET>,
}

impl<RET> Closure<(), (), (), (), (), (), (), (), RET> {
    fn new_with_0_args(code: Box<dyn Fn() -> RET>) -> Closure<(), (), (), (), (), (), (), (), RET> {
        Closure {
            variant: ClosureVariant::Zero(Closure0 {
                wrapped: code,
            })
        }
    }
}

impl<A, RET> Closure<A, (), (), (), (), (), (), (), RET> {
    fn new_with_1_arg(code: Box<dyn Fn(A) -> RET>) -> Closure<A, (), (), (), (), (), (), (), RET> {
        Closure {
            variant: ClosureVariant::One(Closure1 {
                wrapped: code,
            })
        }
    }
}

impl<A, B, RET> Closure<A, B, (), (), (), (), (), (), RET> {
    fn new_with_2_args(code: Box<dyn Fn(A, B) -> RET>) -> Closure<A, B, (), (), (), (), (), (), RET> {
        Closure {
            variant: ClosureVariant::Two(Closure2 {
                wrapped: code,
            })
        }
    }
}

impl<A, B, C, RET> Closure<A, B, C, (), (), (), (), (), RET> {
    fn new_with_3_args(code: Box<dyn Fn(A, B, C) -> RET>) -> Closure<A, B, C, (), (), (), (), (), RET> {
        Closure {
            variant: ClosureVariant::Three(Closure3 {
                wrapped: code,
            })
        }
    }
}

impl<A, B, C, D, RET> Closure<A, B, C, D, (), (), (), (), RET> {
    fn new_with_4_args(code: Box<dyn Fn(A, B, C, D) -> RET>) -> Closure<A, B, C, D, (), (), (), (), RET> {
        Closure {
            variant: ClosureVariant::Four(Closure4 {
                wrapped: code,
            })
        }
    }
}

impl<A, B, C, D, E, RET> Closure<A, B, C, D, E, (), (), (), RET> {
    fn new_with_5_args(code: Box<dyn Fn(A, B, C, D, E) -> RET>) -> Closure<A, B, C, D, E, (), (), (), RET> {
        Closure {
            variant: ClosureVariant::Five(Closure5 {
                wrapped: code,
            })
        }
    }
}

impl<A, B, C, D, E, F, RET> Closure<A, B, C, D, E, F, (), (), RET> {
    fn new_with_6_args(code: Box<dyn Fn(A, B, C, D, E, F) -> RET>) -> Closure<A, B, C, D, E, F, (), (), RET> {
        Closure {
            variant: ClosureVariant::Six(Closure6 {
                wrapped: code,
            })
        }
    }
}

impl<A, B, C, D, E, F, G, RET> Closure<A, B, C, D, E, F, G, (), RET> {
    fn new_with_7_args(code: Box<dyn Fn(A, B, C, D, E, F, G) -> RET>) -> Closure<A, B, C, D, E, F, G, (), RET> {
        Closure {
            variant: ClosureVariant::Seven(Closure7 {
                wrapped: code,
            })
        }
    }
}

impl<A, B, C, D, E, F, G, H, RET> Closure<A, B, C, D, E, F, G, H, RET> {
    fn new_with_8_args(code: Box<dyn Fn(A, B, C, D, E, F, G, H) -> RET>) -> Closure<A, B, C, D, E, F, G, H, RET> {
        Closure {
            variant: ClosureVariant::Eight(Closure8 {
                wrapped: code,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Closure;
    use std::boxed::Box;

    #[test]
    fn it_works_0_args() {
        let stored_closure = Closure::new_with_0_args(
            Box::new(|| -> isize {
                3
            })
        );
        let result = stored_closure.call0();

        assert_eq!(result, 3);
    }

    #[test]
    fn it_works_1_args() {
        let stored_closure = Closure::new_with_1_arg(
            Box::new(|a: i8| -> i32 {
                a as i32
            })
        );
        let result = stored_closure.call1(9);

        assert_eq!(result, 9);
    }

    #[test]
    fn it_works_5_args() {
        let stored_closure = Closure::new_with_6_args(
            Box::new(|a: i8, _b: (), _c: (), _d: (), e: i32, f: i8| -> i32 {
                a as i32 + e + f as i32
            })
        );
        let result = stored_closure.call6(1, (), (), (), 4, 3);

        assert_eq!(result, 8);
    }

    #[test]
    #[should_panic]
    fn it_panics_wrong_args() {
        let stored_closure = Closure::new_with_0_args(
            Box::new(|| -> i32 {
                3
            })
        );
        stored_closure.call1(());
    }
}
