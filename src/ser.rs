use crate::common::*;

pub enum Serializer<'encoder> {
  Encoder(&'encoder mut Encoder),
  SingleItemEncoder(Option<SingleItemEncoder<'encoder>>),
}

impl<'encoder> Serializer<'encoder> {
  pub fn emit<E: ToBencode>(&mut self, value: E) -> Result<(), encoding::Error> {
    match self {
      Self::Encoder(encoder) => encoder.emit(value),
      Self::SingleItemEncoder(encoder) => encoder
        .take()
        .expect("Serializer::emit: single item encoder reused")
        .emit(&value),
    }
  }

  pub fn emit_list<F>(&mut self, list_cb: F) -> Result<(), encoding::Error>
  where
    F: FnOnce(&mut Encoder) -> Result<(), encoding::Error>,
  {
    match self {
      Self::Encoder(encoder) => encoder.emit_list(list_cb),
      Self::SingleItemEncoder(encoder) => encoder
        .take()
        .expect("Serializer::emit_list: single item encoder reused")
        .emit_list(list_cb),
    }
  }

  pub fn emit_dict<F>(&mut self, content_cb: F) -> Result<(), encoding::Error>
  where
    F: FnOnce(SortedDictEncoder) -> Result<(), encoding::Error>,
  {
    match self {
      Self::Encoder(encoder) => encoder.emit_dict(content_cb),
      Self::SingleItemEncoder(encoder) => encoder
        .take()
        .expect("Serializer::emit_dict: single item encoder reused")
        .emit_dict(content_cb),
    }
  }
}

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
  T: Serialize,
{
  let mut encoder = Encoder::new();
  let mut serializer = Serializer::Encoder(&mut encoder);
  value.serialize(&mut serializer)?;
  Ok(encoder.get_output()?)
}

impl<'a, 'encoder> serde::ser::Serializer for &'a mut Serializer<'encoder> {
  type Ok = ();
  type Error = Error;
  type SerializeSeq = Self;
  type SerializeTuple = Self;
  type SerializeTupleStruct = Self;
  type SerializeTupleVariant = Self;
  type SerializeMap = Self;
  type SerializeStruct = Self;
  type SerializeStructVariant = Self;

  fn serialize_bool(self, v: bool) -> Result<()> {
    self.emit(if v { 1 } else { 0 })?;
    Ok(())
  }

  fn serialize_i8(self, v: i8) -> Result<()> {
    self.emit(v)?;
    Ok(())
  }

  fn serialize_i16(self, v: i16) -> Result<()> {
    self.emit(v)?;
    Ok(())
  }

  fn serialize_i32(self, v: i32) -> Result<()> {
    self.emit(v)?;
    Ok(())
  }

  fn serialize_i64(self, v: i64) -> Result<()> {
    self.emit(v)?;
    Ok(())
  }

  fn serialize_u8(self, v: u8) -> Result<()> {
    self.emit(v)?;
    Ok(())
  }

  fn serialize_u16(self, v: u16) -> Result<()> {
    self.emit(v)?;
    Ok(())
  }

  fn serialize_u32(self, v: u32) -> Result<()> {
    self.emit(v)?;
    Ok(())
  }

  fn serialize_u64(self, v: u64) -> Result<()> {
    self.emit(v)?;
    Ok(())
  }

  fn serialize_f32(self, v: f32) -> Result<()> {
    let array: [u8; 4] = v.to_le_bytes();
    let slice: &[u8] = &array;
    self.emit(slice)?;
    Ok(())
  }

  fn serialize_f64(self, v: f64) -> Result<()> {
    let array: [u8; 8] = v.to_le_bytes();
    let slice: &[u8] = &array;
    self.emit(slice)?;
    Ok(())
  }

  fn serialize_char(self, v: char) -> Result<()> {
    self.emit(v as u32)?;
    Ok(())
  }

  fn serialize_str(self, v: &str) -> Result<()> {
    self.emit(v)?;
    Ok(())
  }

  fn serialize_bytes(self, v: &[u8]) -> Result<()> {
    self.emit(v)?;
    Ok(())
  }

  fn serialize_none(self) -> Result<()> {
    self.emit_list(|_| Ok(()))?;
    Ok(())
  }

  fn serialize_some<T>(self, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    let mut err = Ok(());

    self.emit_list(|encoder| {
      let mut serializer = Serializer::Encoder(encoder);
      err = value.serialize(&mut serializer);
      Ok(())
    })?;

    err
  }

  fn serialize_unit(self) -> Result<()> {
    self.emit_list(|_| Ok(()))?;
    Ok(())
  }

  fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
    self.emit_list(|_| Ok(()))?;
    Ok(())
  }

  fn serialize_unit_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
  ) -> Result<()> {
    self.serialize_str(variant)
  }

  fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    value.serialize(self)
  }

  fn serialize_newtype_variant<T>(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    value: &T,
  ) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    let mut err = Ok(());
    self.emit_dict(|mut encoder| {
      encoder.emit_pair_with(variant.as_bytes(), |encoder| {
        let mut serializer = Serializer::SingleItemEncoder(Some(encoder));
        err = value.serialize(&mut serializer);
        Ok(())
      })
    })?;
    err
  }

  fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
    todo!()
  }

  fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
    todo!()
  }

  fn serialize_tuple_struct(
    self,
    _name: &'static str,
    len: usize,
  ) -> Result<Self::SerializeTupleStruct> {
    todo!()
  }

  fn serialize_tuple_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeTupleVariant> {
    todo!()
  }

  fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
    todo!()
  }

  fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
    todo!()
  }

  fn serialize_struct_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeStructVariant> {
    todo!()
  }
}

impl<'a, 'encoder> serde::ser::SerializeSeq for &'a mut Serializer<'encoder> {
  type Ok = ();
  type Error = Error;

  fn serialize_element<T>(&mut self, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    todo!()
  }

  fn end(self) -> Result<()> {
    todo!()
  }
}

impl<'a, 'encoder> serde::ser::SerializeTuple for &'a mut Serializer<'encoder> {
  type Ok = ();
  type Error = Error;

  fn serialize_element<T>(&mut self, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    todo!()
  }

  fn end(self) -> Result<()> {
    todo!()
  }
}

impl<'a, 'encoder> serde::ser::SerializeTupleStruct for &'a mut Serializer<'encoder> {
  type Ok = ();
  type Error = Error;

  fn serialize_field<T>(&mut self, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    todo!()
  }

  fn end(self) -> Result<()> {
    todo!()
  }
}

impl<'a, 'encoder> serde::ser::SerializeTupleVariant for &'a mut Serializer<'encoder> {
  type Ok = ();
  type Error = Error;

  fn serialize_field<T>(&mut self, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    todo!()
  }

  fn end(self) -> Result<()> {
    todo!()
  }
}

impl<'a, 'encoder> serde::ser::SerializeMap for &'a mut Serializer<'encoder> {
  type Ok = ();
  type Error = Error;

  fn serialize_key<T>(&mut self, key: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    todo!()
  }

  fn serialize_value<T>(&mut self, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    todo!()
  }

  fn end(self) -> Result<()> {
    todo!()
  }
}

impl<'a, 'encoder> serde::ser::SerializeStruct for &'a mut Serializer<'encoder> {
  type Ok = ();
  type Error = Error;

  fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    todo!()
  }

  fn end(self) -> Result<()> {
    todo!()
  }
}

impl<'a, 'encoder> serde::ser::SerializeStructVariant for &'a mut Serializer<'encoder> {
  type Ok = ();
  type Error = Error;

  fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    todo!()
  }

  fn end(self) -> Result<()> {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use serde_derive::Serialize;

  fn case<V, B>(value: V, want: B)
  where
    V: Serialize + Debug,
    B: AsRef<[u8]>,
  {
    let want = want.as_ref();
    match to_bytes(&value) {
      Ok(have) => assert_eq!(
        have,
        want,
        "Expected `{}` but got `{}` when serializing `{:?}`",
        String::from_utf8_lossy(&want),
        String::from_utf8_lossy(&have),
        value
      ),
      Err(err) => panic!("Failed to serialize `{:?}`: {}", value, err),
    }
  }

  #[test]
  fn bool() {
    case(true, "i1e");
    case(false, "i0e");
  }

  #[test]
  fn u8() {
    case(0u8, "i0e");
    case(100u8, "i100e");
  }

  #[test]
  fn newtype_variant() {
    #[derive(Debug, Serialize)]
    enum Foo {
      A(u8),
      B(u8),
    }

    case(Foo::A(0), "d1:Ai0ee");
    case(Foo::B(10), "d1:Bi10ee");
  }
}
