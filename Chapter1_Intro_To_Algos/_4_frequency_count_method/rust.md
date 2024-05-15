In rust, there are strict memory management rules.

    Using a for loop, &number has been used.
    This is how rust handles ownership and borrowing.
    &number borrows a reference to each element in the array. And does not take ownership.
    (Learn about reference and ownership in Rust)
    Ownersip means that each value must have a single owner.
            The value 48 can have only one owner. If required, a copy of 48 will be made to store its value. Thw owner exists as long as the scope exists.

    Borrowing allows us to have a reference to that value using &.
    Value is not moved or dropped. Just a pointer is used to reference the value.

    If I dont use &number in &numbers, the whole array will move inside the for loop and will be destroyed after the loop has run.