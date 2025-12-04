use crate::array::list::ListArray;
use crate::array::{ArrayImpl, ArrayImplRef};
use crate::data_type::F64;
use crate::scalar::{Scalar, ScalarRef, ScalarRefImpl};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ListValue(Box<ArrayImpl>);

impl Scalar for ListValue {
    type ArrayType = ListArray;
    type RefType<'a> = ListValueRef<'a>;

    fn as_scalar_ref(&self) -> Self::RefType<'_> {
        ListValueRef {
            data: self.0.as_ref(),
            start: 0,
            end: self.0.len() as u32,
        }
    }
}

impl ListValue {
    pub fn iter(&self) -> impl Iterator<Item = Option<ScalarRefImpl<'_>>> {
        self.0.iter()
    }

    pub fn pretty(&self) -> String {
        format!(
            "[{}]",
            self.iter()
                .map(|x| x.map(|x| x.pretty()).unwrap_or_else(|| "NULL".to_string()))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ListValueRef<'a> {
    data: &'a ArrayImpl,
    start: u32,
    end: u32,
}

impl<'a> std::fmt::Debug for ListValueRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data.data_type() {
            crate::data_type::DataType::Bool => f.debug_list().entries(self.data.as_bool().iter()).finish(),
            crate::data_type::DataType::Integer => f.debug_list().entries(self.data.as_integer().iter()).finish(),
            crate::data_type::DataType::Float => f.debug_list().entries(self.data.as_float().iter()).finish(),
            crate::data_type::DataType::String => f.debug_list().entries(self.data.as_string().iter()).finish(),
            crate::data_type::DataType::U16 => f.debug_list().entries(self.data.as_u16().iter()).finish(),
            crate::data_type::DataType::NodeId => f.debug_list().entries(self.data.as_node_id().iter()).finish(),
            crate::data_type::DataType::RelId => f.debug_list().entries(self.data.as_rel_id().iter()).finish(),
            crate::data_type::DataType::List(data_type) => {
                todo!()
            }
            crate::data_type::DataType::Node => f.debug_list().entries(self.data.as_node().iter()).finish(),
            crate::data_type::DataType::Rel => f.debug_list().entries(self.data.as_rel().iter()).finish(),
            crate::data_type::DataType::Path => todo!(),
            crate::data_type::DataType::Property => f.debug_list().entries(self.data.as_property().iter()).finish(),
            crate::data_type::DataType::PropertyMap => {
                f.debug_list().entries(self.data.as_property_map().iter()).finish()
            }
        }
    }
}

impl<'a> ListValueRef<'a> {
    pub fn new(data: &'a ArrayImpl, start: u32, end: u32) -> Self {
        Self { data, start, end }
    }

    pub fn len(&self) -> usize {
        (self.end - self.start) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = Option<ScalarRefImpl<'a>>> {
        (0..self.len()).map(|i| self.data.get(i))
    }

    pub fn as_u16_slice(&self) -> Option<&[u16]> {
        let array_ref = self.data.as_ref();
        let range = self.start as usize..self.end as usize;
        if let ArrayImplRef::U16(inner) = array_ref {
            Some(&(inner.as_slice()[range]))
        } else {
            None
        }
    }

    pub fn as_integer_slice(&self) -> Option<&[i64]> {
        let array_ref = self.data.as_ref();
        let range = self.start as usize..self.end as usize;
        if let ArrayImplRef::Integer(inner) = array_ref {
            Some(&(inner.as_slice()[range]))
        } else {
            None
        }
    }

    pub fn as_float_slice(&self) -> Option<&[F64]> {
        let array_ref = self.data.as_ref();
        let range = self.start as usize..self.end as usize;
        if let ArrayImplRef::Float(inner) = array_ref {
            Some(&(inner.as_slice()[range]))
        } else {
            None
        }
    }

    pub fn pretty(&self) -> String {
        format!(
            "[{}]",
            self.iter()
                .map(|x| x.map(|x| x.pretty()).unwrap_or_else(|| "NULL".to_string()))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl<'a> ScalarRef<'a> for ListValueRef<'a> {
    type ArrayType = ListArray;
    type ScalarType = ListValue;

    fn to_owned_scalar(&self) -> Self::ScalarType {
        let mut builder = self.data.new_builder(self.len());
        for value in self.data.iter().skip(self.start as usize).take(self.len()) {
            builder.append(value);
        }
        ListValue(Box::new(builder.finish()))
    }
}
