/// Trait used to identify how to convert between `dyn Trait` and concrete types which implement it. Basically you shouldn't implement this trait by hand, use [`dinamify!()`] macro instead.
pub trait MakeConcrete<T> {
    unsafe fn as_concrete(&self) -> &T {
        unsafe { &*(self as *const Self as *const T) }
    }

    unsafe fn as_concrete_mut(&mut self) -> &mut T {
        unsafe { &mut *(self as *mut Self as *mut T) }
    }

    fn from_concrete(value: T) -> Box<Self>;
}

/// This macro is used to make a trait usable in collections from this crate. This macro accepts trait and all trait bounds for associated types.
///
/// ## Example
/// ```rust
/// # use consolaphics::dynamify;
///
/// trait Message {
///     fn message(&self) -> &'static str;
/// }
///
/// // We pass trait name as parameter
/// dynamify!(Message);
///
/// trait WithAssociated<T>
/// where T: std::fmt::Display,
/// {
///     fn output_string(&self, other: T) -> String {
///         format!("I have accepted: {other}")
///     }
/// }
///
/// // We also specify all trait bounds for all associated types. Note that we don't include `<T>` in the trait name
/// dynamify!(WithAssociated where T: std::fmt::Display);
/// ```
///
/// ## Why do we need this macro?
///
/// This macro basically implements [`MakeConcrete<T>`] trait for respective `dyn Trait` type for all types which implement this trait. All collections in this crate are designed to accept only `dyn Trait`s which implement [`MakeConcrete<T>`].
#[macro_export]
macro_rules! dynamify {
    (
        $pa:ident
        $(where $( $gen_bound:ident $(: $($bounds:tt)+)? ),+)?
    ) => {
        impl<MakeConcreteType, $($($gen_bound $(: $($bounds)+)? ),+)?> $crate::make_concrete::MakeConcrete<MakeConcreteType> for dyn $pa $(<$($gen_bound),+>)?
        where
            MakeConcreteType: $pa $(<$($gen_bound),+>)? + 'static,
        {
            fn from_concrete(value: MakeConcreteType) -> Box<Self> {
                Box::new(value) as Box<Self>
            }
        }
    };
}

#[cfg(test)]
mod test {
    trait Simple {}
    dynamify!(Simple);

    trait Associated<T> {}
    dynamify!(Associated where T);

    trait AssociatedWithBounds<T>
    where
        T: std::fmt::Display,
    {
    }
    dynamify!(AssociatedWithBounds where T: std::fmt::Display);
}

#[cfg(feature = "impl_std")]
mod std_impl {
    use std::any::Any;
    use std::borrow::{Borrow, BorrowMut};
    use std::error::Error;
    use std::fmt::{
        Binary, Debug, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex,
        Write as FmtWrite,
    };
    use std::io::{Read, Seek, Write};

    dynamify!(Any);
    dynamify!(Borrow where T: ?Sized);
    dynamify!(BorrowMut where T: ?Sized);
    dynamify!(Error);
    dynamify!(Debug);
    dynamify!(Display);
    dynamify!(Binary);
    dynamify!(LowerExp);
    dynamify!(LowerHex);
    dynamify!(Octal);
    dynamify!(Pointer);
    dynamify!(UpperExp);
    dynamify!(UpperHex);
    dynamify!(FmtWrite);
    dynamify!(Write);
    dynamify!(Read);
    dynamify!(Seek);
    dynamify!(ToString);
}
