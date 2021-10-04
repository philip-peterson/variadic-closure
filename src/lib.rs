#![allow(unsafe_code)]

use destructure_traitobject::data;
use std::any::{Any, TypeId};
use std::rc::Rc;

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
    pub fn new_with_0_args(
        code: Box<dyn Fn() -> RET>,
    ) -> Closure<(), (), (), (), (), (), (), (), RET> {
        Closure {
            variant: ClosureVariant::Zero(Closure0 { wrapped: code }),
        }
    }
}

impl<A, RET> Closure<A, (), (), (), (), (), (), (), RET> {
    pub fn new_with_1_arg(
        code: Box<dyn Fn(A) -> RET>,
    ) -> Closure<A, (), (), (), (), (), (), (), RET> {
        Closure {
            variant: ClosureVariant::One(Closure1 { wrapped: code }),
        }
    }
}

impl<A, B, RET> Closure<A, B, (), (), (), (), (), (), RET> {
    pub fn new_with_2_args(
        code: Box<dyn Fn(A, B) -> RET>,
    ) -> Closure<A, B, (), (), (), (), (), (), RET> {
        Closure {
            variant: ClosureVariant::Two(Closure2 { wrapped: code }),
        }
    }
}

impl<A, B, C, RET> Closure<A, B, C, (), (), (), (), (), RET> {
    pub fn new_with_3_args(
        code: Box<dyn Fn(A, B, C) -> RET>,
    ) -> Closure<A, B, C, (), (), (), (), (), RET> {
        Closure {
            variant: ClosureVariant::Three(Closure3 { wrapped: code }),
        }
    }
}

impl<A, B, C, D, RET> Closure<A, B, C, D, (), (), (), (), RET> {
    pub fn new_with_4_args(
        code: Box<dyn Fn(A, B, C, D) -> RET>,
    ) -> Closure<A, B, C, D, (), (), (), (), RET> {
        Closure {
            variant: ClosureVariant::Four(Closure4 { wrapped: code }),
        }
    }
}

impl<A, B, C, D, E, RET> Closure<A, B, C, D, E, (), (), (), RET> {
    pub fn new_with_5_args(
        code: Box<dyn Fn(A, B, C, D, E) -> RET>,
    ) -> Closure<A, B, C, D, E, (), (), (), RET> {
        Closure {
            variant: ClosureVariant::Five(Closure5 { wrapped: code }),
        }
    }
}

impl<A, B, C, D, E, F, RET> Closure<A, B, C, D, E, F, (), (), RET> {
    pub fn new_with_6_args(
        code: Box<dyn Fn(A, B, C, D, E, F) -> RET>,
    ) -> Closure<A, B, C, D, E, F, (), (), RET> {
        Closure {
            variant: ClosureVariant::Six(Closure6 { wrapped: code }),
        }
    }
}

impl<A, B, C, D, E, F, G, RET> Closure<A, B, C, D, E, F, G, (), RET> {
    pub fn new_with_7_args(
        code: Box<dyn Fn(A, B, C, D, E, F, G) -> RET>,
    ) -> Closure<A, B, C, D, E, F, G, (), RET> {
        Closure {
            variant: ClosureVariant::Seven(Closure7 { wrapped: code }),
        }
    }
}

impl<A, B, C, D, E, F, G, H, RET> Closure<A, B, C, D, E, F, G, H, RET> {
    pub fn new_with_8_args(
        code: Box<dyn Fn(A, B, C, D, E, F, G, H) -> RET>,
    ) -> Closure<A, B, C, D, E, F, G, H, RET> {
        Closure {
            variant: ClosureVariant::Eight(Closure8 { wrapped: code }),
        }
    }
}

// Convenience types

pub type ClosurePrime0<RET> = Closure<(), (), (), (), (), (), (), (), RET>;
pub type ClosurePrime1<A, RET> = Closure<A, (), (), (), (), (), (), (), RET>;
pub type ClosurePrime2<A, B, RET> = Closure<A, B, (), (), (), (), (), (), RET>;
pub type ClosurePrime3<A, B, C, RET> = Closure<A, B, C, (), (), (), (), (), RET>;
pub type ClosurePrime4<A, B, C, D, RET> = Closure<A, B, C, D, (), (), (), (), RET>;
pub type ClosurePrime5<A, B, C, D, E, RET> = Closure<A, B, C, D, E, (), (), (), RET>;
pub type ClosurePrime6<A, B, C, D, E, F, RET> = Closure<A, B, C, D, E, F, (), (), RET>;
pub type ClosurePrime7<A, B, C, D, E, F, G, RET> = Closure<A, B, C, D, E, F, G, (), RET>;
pub type ClosurePrime8<A, B, C, D, E, F, G, H, RET> = Closure<A, B, C, D, E, F, G, H, RET>;

/// A type that is like Closure<A, B, C...> but which has no generic params. This is accomplished
/// via unsafe techniques.
///
/// Note, when invoking the function through the call*() family of methods, the exact types must
/// be passed through, including the return type. No implicit conversions will be performed, as
/// it is a strict TypeId check. Mismatching types will cause a panic.
pub struct Function {
    type_a: TypeId,
    type_b: TypeId,
    type_c: TypeId,
    type_d: TypeId,
    type_e: TypeId,
    type_f: TypeId,
    type_g: TypeId,
    type_h: TypeId,
    type_ret: TypeId,
    code: Rc<dyn Any>,
}

impl Function {
    pub fn from_closure<
        A: Any,
        B: Any,
        C: Any,
        D: Any,
        E: Any,
        F: Any,
        G: Any,
        H: Any,
        RET: Any,
    >(
        code: Closure<A, B, C, D, E, F, G, H, RET>,
    ) -> Function {
        Function {
            type_a: TypeId::of::<A>(),
            type_b: TypeId::of::<B>(),
            type_c: TypeId::of::<C>(),
            type_d: TypeId::of::<D>(),
            type_e: TypeId::of::<E>(),
            type_f: TypeId::of::<F>(),
            type_g: TypeId::of::<G>(),
            type_h: TypeId::of::<H>(),
            type_ret: TypeId::of::<RET>(),
            code: Rc::new(code),
        }
    }

    pub fn call0<A: Any, RET: Any>(&self) -> RET {
        if TypeId::of::<RET>() != self.type_ret {
            panic!(
                "return value is not the correct type ({})",
                std::any::type_name::<RET>()
            )
        }

        let proc: &ClosurePrime0<RET>;
        unsafe {
            let code_ptr = data(self.code.as_ref());
            proc = &*(code_ptr as *const ClosurePrime0<RET>);
        }

        proc.call0()
    }

    pub fn call1<A: Any, RET: Any>(&self, a: A) -> RET {
        if TypeId::of::<RET>() != self.type_ret {
            panic!(
                "return value is not the correct type ({})",
                std::any::type_name::<RET>()
            )
        }
        if TypeId::of::<A>() != self.type_a {
            panic!(
                "argument 1 is not the correct type ({})",
                std::any::type_name::<A>()
            )
        }

        let proc: &ClosurePrime1<A, RET>;
        unsafe {
            let code_ptr = data(self.code.as_ref());
            proc = &*(code_ptr as *const ClosurePrime1<A, RET>);
        }

        proc.call1(a)
    }

    pub fn call2<A: Any, B: Any, RET: Any>(&self, a: A, b: B) -> RET {
        if TypeId::of::<RET>() != self.type_ret {
            panic!(
                "return value is not the correct type ({})",
                std::any::type_name::<RET>()
            )
        }
        if TypeId::of::<A>() != self.type_a {
            panic!(
                "argument 1 is not the correct type ({})",
                std::any::type_name::<A>()
            )
        }
        if TypeId::of::<B>() != self.type_b {
            panic!(
                "argument 2 is not the correct type ({})",
                std::any::type_name::<B>()
            )
        }

        let proc: &ClosurePrime2<A, B, RET>;
        unsafe {
            let code_ptr = data(self.code.as_ref());
            proc = &*(code_ptr as *const ClosurePrime2<A, B, RET>);
        }

        proc.call2(a, b)
    }

    pub fn call3<A: Any, B: Any, C: Any, RET: Any>(&self, a: A, b: B, c: C) -> RET {
        if TypeId::of::<RET>() != self.type_ret {
            panic!(
                "return value is not the correct type ({})",
                std::any::type_name::<RET>()
            )
        }
        if TypeId::of::<A>() != self.type_a {
            panic!(
                "argument 1 is not the correct type ({})",
                std::any::type_name::<A>()
            )
        }
        if TypeId::of::<B>() != self.type_b {
            panic!(
                "argument 2 is not the correct type ({})",
                std::any::type_name::<B>()
            )
        }
        if TypeId::of::<C>() != self.type_c {
            panic!(
                "argument 3 is not the correct type ({})",
                std::any::type_name::<C>()
            )
        }

        let proc: &ClosurePrime3<A, B, C, RET>;
        unsafe {
            let code_ptr = data(self.code.as_ref());
            proc = &*(code_ptr as *const ClosurePrime3<A, B, C, RET>);
        }

        proc.call3(a, b, c)
    }

    pub fn call4<A: Any, B: Any, C: Any, D: Any, RET: Any>(&self, a: A, b: B, c: C, d: D) -> RET {
        if TypeId::of::<RET>() != self.type_ret {
            panic!(
                "return value is not the correct type ({})",
                std::any::type_name::<RET>()
            )
        }
        if TypeId::of::<A>() != self.type_a {
            panic!(
                "argument 1 is not the correct type ({})",
                std::any::type_name::<A>()
            )
        }
        if TypeId::of::<B>() != self.type_b {
            panic!(
                "argument 2 is not the correct type ({})",
                std::any::type_name::<B>()
            )
        }
        if TypeId::of::<C>() != self.type_c {
            panic!(
                "argument 3 is not the correct type ({})",
                std::any::type_name::<C>()
            )
        }
        if TypeId::of::<D>() != self.type_d {
            panic!(
                "argument 4 is not the correct type ({})",
                std::any::type_name::<D>()
            )
        }

        let proc: &ClosurePrime4<A, B, C, D, RET>;
        unsafe {
            let code_ptr = data(self.code.as_ref());
            proc = &*(code_ptr as *const ClosurePrime4<A, B, C, D, RET>);
        }

        proc.call4(a, b, c, d)
    }

    pub fn call5<A: Any, B: Any, C: Any, D: Any, E: Any, RET: Any>(
        &self,
        a: A,
        b: B,
        c: C,
        d: D,
        e: E,
    ) -> RET {
        if TypeId::of::<RET>() != self.type_ret {
            panic!(
                "return value is not the correct type ({})",
                std::any::type_name::<RET>()
            )
        }
        if TypeId::of::<A>() != self.type_a {
            panic!(
                "argument 1 is not the correct type ({})",
                std::any::type_name::<A>()
            )
        }
        if TypeId::of::<B>() != self.type_b {
            panic!(
                "argument 2 is not the correct type ({})",
                std::any::type_name::<B>()
            )
        }
        if TypeId::of::<C>() != self.type_c {
            panic!(
                "argument 3 is not the correct type ({})",
                std::any::type_name::<C>()
            )
        }
        if TypeId::of::<D>() != self.type_d {
            panic!(
                "argument 4 is not the correct type ({})",
                std::any::type_name::<D>()
            )
        }
        if TypeId::of::<E>() != self.type_e {
            panic!(
                "argument 5 is not the correct type ({})",
                std::any::type_name::<E>()
            )
        }

        let proc: &ClosurePrime5<A, B, C, D, E, RET>;
        unsafe {
            let code_ptr = data(self.code.as_ref());
            proc = &*(code_ptr as *const ClosurePrime5<A, B, C, D, E, RET>);
        }

        proc.call5(a, b, c, d, e)
    }

    pub fn call6<A: Any, B: Any, C: Any, D: Any, E: Any, F: Any, RET: Any>(
        &self,
        a: A,
        b: B,
        c: C,
        d: D,
        e: E,
        f: F,
    ) -> RET {
        if TypeId::of::<RET>() != self.type_ret {
            panic!(
                "return value is not the correct type ({})",
                std::any::type_name::<RET>()
            )
        }
        if TypeId::of::<A>() != self.type_a {
            panic!(
                "argument 1 is not the correct type ({})",
                std::any::type_name::<A>()
            )
        }
        if TypeId::of::<B>() != self.type_b {
            panic!(
                "argument 2 is not the correct type ({})",
                std::any::type_name::<B>()
            )
        }
        if TypeId::of::<C>() != self.type_c {
            panic!(
                "argument 3 is not the correct type ({})",
                std::any::type_name::<C>()
            )
        }
        if TypeId::of::<D>() != self.type_d {
            panic!(
                "argument 4 is not the correct type ({})",
                std::any::type_name::<D>()
            )
        }
        if TypeId::of::<E>() != self.type_e {
            panic!(
                "argument 5 is not the correct type ({})",
                std::any::type_name::<E>()
            )
        }
        if TypeId::of::<F>() != self.type_f {
            panic!(
                "argument 6 is not the correct type ({})",
                std::any::type_name::<F>()
            )
        }

        let proc: &ClosurePrime6<A, B, C, D, E, F, RET>;
        unsafe {
            let code_ptr = data(self.code.as_ref());
            proc = &*(code_ptr as *const ClosurePrime6<A, B, C, D, E, F, RET>);
        }

        proc.call6(a, b, c, d, e, f)
    }

    pub fn call7<A: Any, B: Any, C: Any, D: Any, E: Any, F: Any, G: Any, RET: Any>(
        &self,
        a: A,
        b: B,
        c: C,
        d: D,
        e: E,
        f: F,
        g: G,
    ) -> RET {
        if TypeId::of::<RET>() != self.type_ret {
            panic!(
                "return value is not the correct type ({})",
                std::any::type_name::<RET>()
            )
        }
        if TypeId::of::<A>() != self.type_a {
            panic!(
                "argument 1 is not the correct type ({})",
                std::any::type_name::<A>()
            )
        }
        if TypeId::of::<B>() != self.type_b {
            panic!(
                "argument 2 is not the correct type ({})",
                std::any::type_name::<B>()
            )
        }
        if TypeId::of::<C>() != self.type_c {
            panic!(
                "argument 3 is not the correct type ({})",
                std::any::type_name::<C>()
            )
        }
        if TypeId::of::<D>() != self.type_d {
            panic!(
                "argument 4 is not the correct type ({})",
                std::any::type_name::<D>()
            )
        }
        if TypeId::of::<E>() != self.type_e {
            panic!(
                "argument 5 is not the correct type ({})",
                std::any::type_name::<E>()
            )
        }
        if TypeId::of::<F>() != self.type_f {
            panic!(
                "argument 6 is not the correct type ({})",
                std::any::type_name::<F>()
            )
        }
        if TypeId::of::<G>() != self.type_g {
            panic!(
                "argument 7 is not the correct type ({})",
                std::any::type_name::<G>()
            )
        }

        let proc: &ClosurePrime7<A, B, C, D, E, F, G, RET>;
        unsafe {
            let code_ptr = data(self.code.as_ref());
            proc = &*(code_ptr as *const ClosurePrime7<A, B, C, D, E, F, G, RET>);
        }

        proc.call7(a, b, c, d, e, f, g)
    }

    pub fn call8<A: Any, B: Any, C: Any, D: Any, E: Any, F: Any, G: Any, H: Any, RET: Any>(
        &self,
        a: A,
        b: B,
        c: C,
        d: D,
        e: E,
        f: F,
        g: G,
        h: H,
    ) -> RET {
        if TypeId::of::<RET>() != self.type_ret {
            panic!(
                "return value is not the correct type ({})",
                std::any::type_name::<RET>()
            )
        }
        if TypeId::of::<A>() != self.type_a {
            panic!(
                "argument 1 is not the correct type ({})",
                std::any::type_name::<A>()
            )
        }
        if TypeId::of::<B>() != self.type_b {
            panic!(
                "argument 2 is not the correct type ({})",
                std::any::type_name::<B>()
            )
        }
        if TypeId::of::<C>() != self.type_c {
            panic!(
                "argument 3 is not the correct type ({})",
                std::any::type_name::<C>()
            )
        }
        if TypeId::of::<D>() != self.type_d {
            panic!(
                "argument 4 is not the correct type ({})",
                std::any::type_name::<D>()
            )
        }
        if TypeId::of::<E>() != self.type_e {
            panic!(
                "argument 5 is not the correct type ({})",
                std::any::type_name::<E>()
            )
        }
        if TypeId::of::<F>() != self.type_f {
            panic!(
                "argument 6 is not the correct type ({})",
                std::any::type_name::<F>()
            )
        }
        if TypeId::of::<G>() != self.type_g {
            panic!(
                "argument 7 is not the correct type ({})",
                std::any::type_name::<G>()
            )
        }
        if TypeId::of::<H>() != self.type_h {
            panic!(
                "argument 8 is not the correct type ({})",
                std::any::type_name::<H>()
            )
        }

        let proc: &ClosurePrime8<A, B, C, D, E, F, G, H, RET>;
        unsafe {
            let code_ptr = data(self.code.as_ref());
            proc = &*(code_ptr as *const ClosurePrime8<A, B, C, D, E, F, G, H, RET>);
        }

        proc.call8(a, b, c, d, e, f, g, h)
    }
}

#[cfg(test)]
mod tests {
    use super::{Closure, Function};
    use std::boxed::Box;

    #[test]
    fn it_works_0_args() {
        let stored_closure = Closure::new_with_0_args(Box::new(|| -> isize { 3 }));
        let result = stored_closure.call0();

        assert_eq!(result, 3);
    }

    #[test]
    fn it_works_1_args() {
        let stored_closure = Closure::new_with_1_arg(Box::new(|a: i8| -> i32 { a as i32 }));
        let result = stored_closure.call1(9);

        assert_eq!(result, 9);

        let stored_func = Function::from_closure(stored_closure);
        let result: i32 = stored_func.call1(3 as i8);
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works_5_args() {
        let stored_closure = Closure::new_with_6_args(Box::new(
            |a: i8, _b: (), _c: (), _d: (), e: i32, f: i8| -> i32 { a as i32 + e + f as i32 },
        ));
        let result = stored_closure.call6(1, (), (), (), 4, 3);

        assert_eq!(result, 8);

        let stored_func = Function::from_closure(stored_closure);
        let result: i32 = stored_func.call6(3 as i8, (), (), (), 9, 2 as i8);
        assert_eq!(result, 14);
    }

    #[test]
    #[should_panic]
    fn it_panics_wrong_args() {
        let stored_closure = Closure::new_with_0_args(Box::new(|| -> i32 { 3 }));
        stored_closure.call1(());
    }
}
