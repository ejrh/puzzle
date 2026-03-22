use bevy::prelude::Entity;

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    Lookup(String),
    Constant(Constant),
    MoveTo,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Constant {
    Entity(Entity),
    Float(f32),
}

macro_rules! impl_froms {
    ($( $item:ident $( ($ty:ty) )? )*) => {
        $(
            impl_froms!(@item $item $( ( $ty ) )? );
        )*
    };

    (@item $name:ident($ty:ty)) => {
        impl From<$ty> for Constant {
            fn from(value: $ty) -> Self {
                Constant::$name(value)
            }
        }
    };

    (@item $ty:ident) => {
        impl From<$ty> for Constant {
            fn from(value: $ty) -> Self {
                Constant::$ty(value)
            }
        }
    };
}

impl_froms!(Entity Float(f32));
