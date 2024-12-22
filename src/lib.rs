#[inline(always)]
pub fn i32<T: Sized>(i: T) -> i32
where
    T: Shl<i32> ,
{
    i.into()
}
#[inline(always)]
pub fn i64<T: Sized>(i: T) -> i64
where
    T: Shl<i64>,
{
    i.into()
}
#[inline(always)]
pub fn i16<T: Sized>(i: T) -> i16
where
    T: Shl<i16>,
{
    i.into()
}
#[inline(always)]
pub fn f32<T: Sized>(i: T) -> f32
where
    T: Shl<f32>,
{
    i.into()
}
#[inline(always)]
pub fn f64<T: Sized>(i: T) -> f64
where
    T: Shl<f64>,
{
    i.into()
}
#[inline(always)]
pub fn i128<T: Sized>(i: T) -> i128
where
    T: Shl<i128>,
{
    i.into()
}
#[inline(always)]
pub fn u128<T: Sized>(i: T) -> u128
where
    T: Shl<u128>,
{
    i.into()
}
#[inline(always)]
pub fn u32<T: Sized>(i: T) -> u32
where
    T: Shl<u32>,
{
    i.into()
}
#[inline(always)]
pub fn u16<T: Sized>(i: T) -> u16
where
    T: Shl<u16>,
{
    i.into()
}
pub fn u8<T: Sized>(i: T) -> u8
where
    T: Shl<u8>,
{
    i.into()
}
#[inline(always)]
pub fn i8<T: Sized>(i: T) -> i8
where
    T: Shl<i8>,
{
    i.into()
}
#[inline(always)]
pub fn u64<T: Sized>(i: T) -> u64
where
    T: Shl<u64>,
{
    i.into()
}

