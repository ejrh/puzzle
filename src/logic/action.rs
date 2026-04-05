use std::fmt::{Debug, Display};
use bevy::prelude::{Entity, Transform};
use derive_more::Display;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    Lookup(String),
    Push(Constant),
    GetTransform,
    MoveTo,
    ReparentInPlace,
    RemoveDecoration,
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum StackError {
    #[error("empty stack")]
    EmptyStack,
    #[error("wrong constant")]
    WrongConstant,
}

#[derive(Default)]
pub struct Stack(Vec<Constant>);

impl Stack {
    pub fn push(&mut self, constant: impl Into<Constant>) {
        self.0.push(constant.into());
    }

    pub fn pop(&mut self) -> Result<Constant, StackError> {
        self.0.pop().ok_or(StackError::EmptyStack)
    }

    pub fn pop1<A: TryFrom<Constant, Error=StackError>>(&mut self) -> Result<A, StackError> {
        let a = self.pop()?.try_into()?;

        Ok(a)
    }

    pub fn pop2<A: TryFrom<Constant, Error=StackError>, B: TryFrom<Constant, Error=StackError>>(&mut self) -> Result<(A, B), StackError> {
        let b = self.pop()?.try_into()?;
        let a = self.pop()?.try_into()?;

        Ok((a, b))
    }

    pub fn pop3<A: TryFrom<Constant, Error=StackError>, B: TryFrom<Constant, Error=StackError>, C: TryFrom<Constant, Error=StackError>>(&mut self) -> Result<(A, B, C), StackError> {
        let c = self.pop()?.try_into()?;
        let b = self.pop()?.try_into()?;
        let a = self.pop()?.try_into()?;

        Ok((a, b, c))
    }
}

#[derive(Clone, Display, PartialEq)]
pub enum Constant {
    #[display("{_0:?}")]
    Entity(Entity),
    #[display("{_0:?}")]
    Float(f32),
    #[display("{_0:?}")]
    Transform(Transform),
}

impl Debug for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

macro_rules! impl_froms {
    ($( $item:ident $( ($ty:ty) )? )*) => {
        $(
            impl_froms!(@item $item $( ( $ty ) )? );
        )*
    };

    (@item $name:ident($ty:ty)) => {
        impl_froms!(@name_type $name $ty);
    };

    (@item $ty:ident) => {
        impl_froms!(@name_type $ty $ty);
    };

    (@name_type $name:ident $ty:ty) => {
        impl From<$ty> for Constant {
            fn from(value: $ty) -> Self {
                Constant::$name(value)
            }
        }

        impl TryFrom<Constant> for $ty {
            type Error = StackError;

            fn try_from(value: Constant) -> Result<$ty, Self::Error> {
                match value {
                    Constant::$name(value) => Ok(value),
                    _ => Err(StackError::WrongConstant),
                }
            }
        }
    };
}

impl_froms!(Entity Float(f32) Transform);
