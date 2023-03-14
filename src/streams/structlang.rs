use pest::Parser;
use crate::streams::SeekRead;

#[derive(Parser)]
#[grammar = "structlang.pest"]
pub struct StructLangParser;

#[derive(Debug, Clone)]
pub struct StructRepr {
    pub name: String,
    pub fields: Vec<FieldRepr>,
}

#[derive(Debug, Clone)]
pub enum FieldVal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct FieldRepr {
    pub name: String,
    pub ty: String,
    pub val: FieldVal,
}

pub fn parse_structs<S: SeekRead>(expr: &str, stream: S) {
    let pr = StructLangParser::parse(Rule::structures, expr).unwrap();
    println!("{:#?}", pr);
}
