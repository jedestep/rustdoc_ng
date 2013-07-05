use its = syntax::parse::token::ident_to_str;

use super::doctree;

pub enum Attribute {
    Word(~str),
    List(~str, ~[~str]),
    NameValue(~str, ~str)
}

pub struct TyParam {
    name: ~str,
    bounds: ~[TyParamBound]
}

pub enum TyParamBound {
    RegionBound,
    TraitBound(Trait)
}

pub struct Lifetime(~str);

// maybe use a Generic enum and use ~[Generic]?
pub struct Generics {
    lifetimes: ~[Lifetime],
    type_params: ~[TyParam]
}

pub struct Method;

pub struct Trait {
    name: ~str,
    methods: ~[Method],
    lifetimes: ~[Lifetime],
    generics: Generics
}

pub struct StructField;

pub struct Struct {
    name: ~str,
    struct_type: doctree::StructType,
    attrs: ~[Attribute],
    generics: Generics,
    fields: ~[StructField]
}

pub trait Stringify<T> {
    pub fn stringify(&self) -> T;
}

impl Stringify<Struct> for doctree::Struct {
    pub fn stringify(&self) -> Struct {
        Struct {
            name: its(self.name),
            struct_type: self.struct_type,
            attrs: self.attrs.iter().transform(|x| x.stringify()).collect(),
            generics: self.generics.stringify(),
            fields: self.fields.iter().transform(|x| x.stringify()).collect()
        }
    }
}

