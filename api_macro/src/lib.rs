pub trait DerivedMacro {
    fn print_attributes(&self);
}
pub trait GetAttributesMacro {
    fn get_attributes(&self) -> Vec<&str>;
}
