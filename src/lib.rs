use std::ops::Shl;

#[inline(always)]
pub fn i32<T: Sized>(i: T) -> i32
where
 i32: From<T>
{
    i.into()
}
#[inline(always)]
pub fn i64<T: Sized>(i: T) -> i64
where
     i64: From<T>
{
    i.into()
}
#[inline(always)]
pub fn i16<T: Sized>(i: T) -> i16
where
 i16: From<T>
{
    i.into()
}
#[inline(always)]
pub fn f32<T: Sized>(i: T) -> f32
where
  if32: From<T>
{
    i.into()
}
#[inline(always)]
pub fn f64<T: Sized>(i: T) -> f64
where
 f64: From<T>
{
    i.into()
}
#[inline(always)]
pub fn i128<T: Sized>(i: T) -> i128
where
   i128: From<T>
{
    i.into()
}
#[inline(always)]
pub fn u128<T: Sized>(i: T) -> u128
where
 u128: From<T>
{
    i.into()
}
#[inline(always)]
pub fn u32<T: Sized>(i: T) -> u32
where     u32: From<T>,
   
{
    i.into()
}
#[inline(always)]
pub fn u16<T: Sized>(i: T) -> u16
where
    u16: From<T>,
{
    i.into()
}
pub fn u8<T: Sized>(i: T) -> u8
where
    u8: From<T>,
{
    i.into()
}
#[inline(always)]
pub fn i8<T: Sized>(i: T) -> i8
where
    i8: From<T>,
{
    i.into()
}
#[inline(always)]
pub fn u64<T: Sized>(i: T) -> u64
where
    u64: From<T>,
{
    i.into()
}

