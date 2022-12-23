fn main()
{
    let _immutable_binding = 13;
    let mut mutable_binding = 14;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += _immutable_binding;


    println!("After mutation: {}", mutable_binding);
}
