# EnumSet

A hybrid map/set data-structure for Enums, using the variant of an enum as the key and data as the value.

This data structure makes the most sense to use when you have an enum with a lot of variants and want a collection of the where only a small portion of the variants are used.

## Overview
Here is an example showing the functionality of `EnumSet` to give you an idea of how it works.

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

## Example: Keeping track of applied CSS Styles
In this example we want to keep track of the applied CSS styles for an object. The different CSS styles can be represented with different types such as integers, strings, or even structs. Keeping this in mind, we can naturally be express all CSS Styles using a single Enum, with each style as a variant holding a relevant type as data.

```rust
enum CSSStyles {
  FontSize(f64),
  TextColor{r: i32, g: i32, b: i32},
  Margin(i32, i32, i32, i32),
  Padding(i32, i32, i32, i32),
  Italic(bool),
  Bold(bool),
  // ...etc
}
```

It makes sense to use the `EnumSet` to represent a collection of `CSSStyles` as there are around 200 unique css styles, but only a few is likely to be set for an object at a given time. In an `EnumSet` the variants of the enums are keys and will ensure uniqueness, so no style can be set twice. 

```rust
let mut applied_styles: EnumSet<CSSStyles> = EnumSet::default();

applied_styles.update(CSSStyles::Italic(true));
applied_styles.update(CSSStyles::Padding(0, 2, 2, 0));
applied_styles.update(CSSStyles::FontSize(16));
```

Now if we want to see what the padding of our object is we can simply query the `EnumSet` using the "Padding" variant of `CSSStyles`.

```rust
applied_styles.get(CSSStyles::Padding) // -> Some(CSSStyles::Padding(0, 2, 2, 0))
```

and if we try to query the "Bold" property we will get None as we have not set that property

```rust
applied_styles.get(CSSStyles::Bold) // -> None
```

