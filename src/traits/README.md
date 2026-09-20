# **🤝 Traits**
## Intro to Traits

- Traits are called interfaces in other languages
- A trait is a "distinguishing quality or characteristic"
- A trait is a quality os detail that a thing has that allows us to identify it and categorize it
- Consider the examples:
    - Flight -> describes the quality of being able to fly. A bird and a plane have the trait of flight
    - Storage -> describes the quality of being able to be kept or stored. A USB pen, a garage and a bookshelf all have a trait of storage
    - Illumination -> describes the quality of being able provide light. A candle, a computer screen and the sun all have a trait of illumination
- Traits are not a type of thing, rather they describes an ability, a functionality, a capacity to serve some defined purposes

## Traits in Rust

- A trait is a contract that describes the functionality that a type should have
- We use the word **implement** to describe when a type honor's a trait requirements. For the previous example, we can say that a candle, a computer screen and the sun *implement* the 'illumination' tait
- A trait definition declare the method(s) that a type implementing that trait must have
    - Method name
    - Parameters with types
    - Return value type

## Traits We've seen before

- The **Display** and **Debug** traits require a type to define methods for presenting itself as a string
- The **Clone** trait requires a type to define a **clone** method for creating a duplicate of itself

## Implementations

- Once we have defined a trait, we can implement it on structs and enums. The type promises to honor the trait's contract. For example: a type implementing the **Flight** trait promises it can fly
- Multiple types can implement the same trait. For example: a bird and a plane type both implement the **Flight** trait
- A type can implement multiple traits. For example: a plane can implement both the **Flight** and **Storage** traits
