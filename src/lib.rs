mod enumset;

pub use enumset::EnumSet;

#[cfg(test)]
mod tests {
    use crate::EnumSet;

    #[test]
    fn it_works() {
        #[derive(Debug, PartialEq)]
        enum MyEnum {
            Variant1(String),
            Variant2(f64),
        }

        let mut set: EnumSet<MyEnum> = EnumSet::default();

        // Insert a variant
        set.update(MyEnum::Variant1("Hello World".to_string()));
        assert_eq!(set.get(MyEnum::Variant1), Some(&MyEnum::Variant1("Hello World".to_string())));
        
        // Overwrite the same variant  
        set.update(MyEnum::Variant2(1.0));
        set.update(MyEnum::Variant2(2.0));
        set.update(MyEnum::Variant2(3.0));
        assert_eq!(set.get(MyEnum::Variant2), Some(&MyEnum::Variant2(3.0)));
        
        // Remove a variant
        assert_eq!(set.remove(MyEnum::Variant2), true);
        assert_eq!(set.remove(MyEnum::Variant2), false);
        assert_eq!(set.get(MyEnum::Variant2), None);
    }
}
