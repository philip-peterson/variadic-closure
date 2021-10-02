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

#[macro_export]
macro_rules! variadic_closure {
    ( fn $fn:ident ( ) -> $ret:ty $body:block ) => {
        $crate::Closure {
            variant: $crate::ClosureVariant::Zero::<(), (), (), (), (), (), (), (), $ret>(
                $crate::Closure0 {
                    wrapped: Box::new(|| -> $ret {
                        $body
                    }),
                }
            ),
        }
    };

    ( fn $fn:ident (
        $arg1: ident : $argty1:ty
    ) -> $ret:ty $body:block ) => {
        $crate::Closure {
            variant: $crate::ClosureVariant::One::<
                $argty1,
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                $ret
            >($crate::Closure1 {
                wrapped: Box::new(|
                    $arg1: $argty1
                | -> $ret {
                    $body
                }),
            }),
        }
    };

    ( fn $fn:ident (
        $arg1: ident : $argty1:ty,
        $arg2: ident : $argty2:ty
    ) -> $ret:ty $body:block ) => {
        $crate::Closure {
            variant: $crate::ClosureVariant::Two::<
                $argty1,
                $argty2,
                (),
                (),
                (),
                (),
                (),
                (),
                $ret
            >($crate::Closure2 {
                wrapped: Box::new(|
                    $arg1: $argty1,
                    $arg2: $argty2
                | -> $ret {
                    $body
                }),
            }),
        }
    };

    ( fn $fn:ident (
        $arg1: ident : $argty1:ty,
        $arg2: ident : $argty2:ty,
        $arg3: ident : $argty3:ty
    ) -> $ret:ty $body:block ) => {
        $crate::Closure {
            variant: $crate::ClosureVariant::Three::<
                $argty1,
                $argty2,
                $argty3,
                (),
                (),
                (),
                (),
                (),
                $ret
            >($crate::Closure3 {
                wrapped: Box::new(|
                    $arg1: $argty1,
                    $arg2: $argty2,
                    $arg3: $argty3
                | -> $ret {
                    $body
                }),
            }),
        }
    };

    ( fn $fn:ident (
        $arg1: ident : $argty1:ty,
        $arg2: ident : $argty2:ty,
        $arg3: ident : $argty3:ty,
        $arg4: ident : $argty4:ty
    ) -> $ret:ty $body:block ) => {
        $crate::Closure {
            variant: $crate::ClosureVariant::Four::<
                $argty1,
                $argty2,
                $argty3,
                $argty4,
                (),
                (),
                (),
                (),
                $ret
            >($crate::Closure4 {
                wrapped: Box::new(|
                    $arg1: $argty1,
                    $arg2: $argty2,
                    $arg3: $argty3,
                    $arg4: $argty4
                | -> $ret {
                    $body
                }),
            }),
        }
    };

    ( fn $fn:ident (
        $arg1: ident : $argty1:ty,
        $arg2: ident : $argty2:ty,
        $arg3: ident : $argty3:ty,
        $arg4: ident : $argty4:ty,
        $arg5: ident : $argty5:ty
    ) -> $ret:ty $body:block ) => {
        $crate::Closure {
            variant: $crate::ClosureVariant::Five::<
                $argty1,
                $argty2,
                $argty3,
                $argty4,
                $argty5,
                (),
                (),
                (),
                $ret
            >($crate::Closure5 {
                wrapped: Box::new(|
                    $arg1: $argty1,
                    $arg2: $argty2,
                    $arg3: $argty3,
                    $arg4: $argty4,
                    $arg5: $argty5
                | -> $ret {
                    $body
                }),
            }),
        }
    };

    ( fn $fn:ident (
        $arg1: ident : $argty1:ty,
        $arg2: ident : $argty2:ty,
        $arg3: ident : $argty3:ty,
        $arg4: ident : $argty4:ty,
        $arg5: ident : $argty5:ty,
        $arg6: ident : $argty6:ty
    ) -> $ret:ty $body:block ) => {
        $crate::Closure {
            variant: $crate::ClosureVariant::Six::<
                $argty1,
                $argty2,
                $argty3,
                $argty4,
                $argty5,
                $argty6,
                (),
                (),
                $ret
            >($crate::Closure6 {
                wrapped: Box::new(|
                    $arg1: $argty1,
                    $arg2: $argty2,
                    $arg3: $argty3,
                    $arg4: $argty4,
                    $arg5: $argty5,
                    $arg6: $argty6
                | -> $ret {
                    $body
                }),
            }),
        }
    };

    ( fn $fn:ident (
        $arg1: ident : $argty1:ty,
        $arg2: ident : $argty2:ty,
        $arg3: ident : $argty3:ty,
        $arg4: ident : $argty4:ty,
        $arg5: ident : $argty5:ty,
        $arg6: ident : $argty6:ty,
        $arg7: ident : $argty7:ty
    ) -> $ret:ty $body:block ) => {
        $crate::Closure {
            variant: $crate::ClosureVariant::Seven::<
                $argty1,
                $argty2,
                $argty3,
                $argty4,
                $argty5,
                $argty6,
                $argty7,
                (),
                $ret
            >($crate::Closure7 {
                wrapped: Box::new(|
                    $arg1: $argty1,
                    $arg2: $argty2,
                    $arg3: $argty3,
                    $arg4: $argty4,
                    $arg5: $argty5,
                    $arg6: $argty6,
                    $arg7: $argty7
                | -> $ret {
                    $body
                }),
            }),
        }
    };

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works_0_args() {
        let stored_closure = variadic_closure! {
            fn foo() -> isize {
                3
            }
        };
        let result = stored_closure.call0();

        assert_eq!(result, 3);
    }

    #[test]
    fn it_works_1_args() {
        let stored_closure = variadic_closure! {
            fn foo(a: i8) -> i32 {
                a as i32
            }
        };
        let result = stored_closure.call1(9);

        assert_eq!(result, 9);
    }

    #[test]
    fn it_works_5_args() {
        let stored_closure = variadic_closure! {
            fn foo(a: i8, _b: (), _c: (), _d: (), e: i32, f: i8) -> i32 {
                a as i32 + e + f as i32
            }
        };
        let result = stored_closure.call6(1, (), (), (), 4, 3);

        assert_eq!(result, 8);
    }

    #[test]
    #[should_panic]
    fn it_panics_wrong_args() {
        let stored_closure = variadic_closure! {
            fn foo() -> isize {
                3
            }
        };
        stored_closure.call1(());
    }
}
