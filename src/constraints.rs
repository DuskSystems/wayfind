use std::net::{Ipv4Addr, Ipv6Addr};

pub trait Constraint {
    const NAME: &'static str;

    fn check(segment: &str) -> bool;
}

macro_rules! impl_constraint_fromstr {
    ($type:ty, $name:expr) => {
        impl Constraint for $type {
            const NAME: &'static str = $name;

            fn check(segment: &str) -> bool {
                <$type as std::str::FromStr>::from_str(segment).is_ok()
            }
        }
    };
}

impl_constraint_fromstr!(u8, "u8");
impl_constraint_fromstr!(u16, "u16");
impl_constraint_fromstr!(u32, "u32");
impl_constraint_fromstr!(u64, "u64");
impl_constraint_fromstr!(u128, "u128");
impl_constraint_fromstr!(usize, "usize");

impl_constraint_fromstr!(i8, "i8");
impl_constraint_fromstr!(i16, "i16");
impl_constraint_fromstr!(i32, "i32");
impl_constraint_fromstr!(i64, "i64");
impl_constraint_fromstr!(i128, "i128");
impl_constraint_fromstr!(isize, "isize");

impl_constraint_fromstr!(f32, "f32");
impl_constraint_fromstr!(f64, "f64");

impl_constraint_fromstr!(bool, "bool");

impl_constraint_fromstr!(Ipv4Addr, "ipv4");
impl_constraint_fromstr!(Ipv6Addr, "ipv6");
