//! This module contains `Impossible` serializer and its implementations.

use lib::*;

use ser::{self, Serialize, SerializeSeq, SerializeTuple, SerializeTupleStruct,
          SerializeTupleVariant, SerializeMap, SerializeStruct, SerializeStructVariant};

/// Helper type for implementing a `Serializer` that does not support
/// serializing one of the compound types.
///
/// This type cannot be instantiated, but implements every one of the traits
/// corresponding to the `Serializer` compound types: `SerializeSeq`,
/// `SerializeTuple`, `SerializeTupleStruct`, `SerializeTupleVariant`,
/// `SerializeMap`, `SerializeStruct`, and `SerializeStructVariant`.
///
/// ```rust
/// # #[macro_use]
/// # extern crate serde;
/// #
/// # use serde::ser::{Serializer, Impossible};
/// # use serde::private::ser::Error;
/// #
/// # struct MySerializer;
/// #
/// impl Serializer for MySerializer {
///     type Ok = ();
///     type Error = Error;
///
///     type SerializeSeq = Impossible<(), Error>;
///     /* other associated types */
///
///     /// This data format does not support serializing sequences.
///     fn serialize_seq(self,
///                      len: Option<usize>)
///                      -> Result<Self::SerializeSeq, Error> {
///         // Given Impossible cannot be instantiated, the only
///         // thing we can do here is to return an error.
/// #         stringify! {
///         Err(...)
/// #         };
/// #         unimplemented!()
///     }
///
///     /* other Serializer methods */
/// #     __serialize_unimplemented! {
/// #         bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str bytes none some
/// #         unit unit_struct unit_variant newtype_struct newtype_variant
/// #         seq_fixed_size tuple tuple_struct tuple_variant map struct
/// #         struct_variant
/// #     }
/// }
/// #
/// # fn main() {}
/// ```
pub struct Impossible<Ok, E> {
    void: Void,
    _marker: PhantomData<(Ok, E)>,
}

enum Void {}

impl<Ok, E> SerializeSeq for Impossible<Ok, E>
    where E: ser::Error
{
    type Ok = Ok;
    type Error = E;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<(), E> {
        match self.void {}
    }

    fn end(self) -> Result<Ok, E> {
        match self.void {}
    }
}

impl<Ok, E> SerializeTuple for Impossible<Ok, E>
    where E: ser::Error
{
    type Ok = Ok;
    type Error = E;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<(), E> {
        match self.void {}
    }

    fn end(self) -> Result<Ok, E> {
        match self.void {}
    }
}

impl<Ok, E> SerializeTupleStruct for Impossible<Ok, E>
    where E: ser::Error
{
    type Ok = Ok;
    type Error = E;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<(), E> {
        match self.void {}
    }

    fn end(self) -> Result<Ok, E> {
        match self.void {}
    }
}

impl<Ok, E> SerializeTupleVariant for Impossible<Ok, E>
    where E: ser::Error
{
    type Ok = Ok;
    type Error = E;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<(), E> {
        match self.void {}
    }

    fn end(self) -> Result<Ok, E> {
        match self.void {}
    }
}

impl<Ok, E> SerializeMap for Impossible<Ok, E>
    where E: ser::Error
{
    type Ok = Ok;
    type Error = E;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, _key: &T) -> Result<(), E> {
        match self.void {}
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<(), E> {
        match self.void {}
    }

    fn end(self) -> Result<Ok, E> {
        match self.void {}
    }
}

impl<Ok, E> SerializeStruct for Impossible<Ok, E>
    where E: ser::Error
{
    type Ok = Ok;
    type Error = E;

    fn serialize_field<T: ?Sized + Serialize>(&mut self,
                                              _key: &'static str,
                                              _value: &T)
                                              -> Result<(), E> {
        match self.void {}
    }

    fn end(self) -> Result<Ok, E> {
        match self.void {}
    }
}

impl<Ok, E> SerializeStructVariant for Impossible<Ok, E>
    where E: ser::Error
{
    type Ok = Ok;
    type Error = E;

    fn serialize_field<T: ?Sized + Serialize>(&mut self,
                                              _key: &'static str,
                                              _value: &T)
                                              -> Result<(), E> {
        match self.void {}
    }

    fn end(self) -> Result<Ok, E> {
        match self.void {}
    }
}
