fn main()
{
    //variable binding have scopes, just like in (int i = 0;....)
    let long_lived_binding = 1;

    //Variables lives in blocks
    {
        let short_lived_bindig = 2;
        let shadowed_binding = "abc";
        println!("inner short: {}", short_lived_bindig);
        println!("shadowed in the block: {}", shadowed_binding);
    }

    //Error
    //println!("outer short: {}", short_lived_bindig);
    
    let shadowed_binding = 43;
    println!("outer long: {}", long_lived_binding);
    println!("shadowed outside the block: {}", shadowed_binding);
}
