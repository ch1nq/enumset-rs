# EnumSet

A hybrid map/set data-structure for Enums, using the variant an enum as a key.  

## Example usage

```rust
use enumset-rs::EnumSet;
  
#[derive(Debug)]
enum MyEnum {
  Variant1(i32),
  Variant2(f64),
}

fn main() {
    let mut set: EnumSet<MyEnum> = EnumSet::default();

    // Insert different variants into the set
    set.update(MyEnum::Variant1(42));
    set.update(MyEnum::Variant2(100.0));
    
    println!("{:?}", set); // Output: EnumSet{Variant1(42), Variant2(100.0)}

    // Overwrite the same variant
    set.update(MyEnum::Variant2(200.0));
    set.update(MyEnum::Variant2(300.0));
    
    println!("{:?}", set); // Output: EnumSet{Variant1(42), Variant2(300.0)}
    
    // Remove using variant as key
    set.remove(MyEnum::Variant1);
    
    println!("{:?}", set); // Output: EnumSet{Variant2(300.0)}
}
```
