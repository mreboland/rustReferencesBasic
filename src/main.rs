fn main() {
    println!("Hello, world!");



    // References

    // All the pointer types we've seen so far, the simple Box<T> heap pointer, and the pointers internal to String and Vec values, are owning pointers. When the owner is dropped, the referent goes with it. Rust also has non-owning pointer types called references, which have no effect on their referents' lifetimes.

    // In fact, references must never outlive their referents. We must make it apparent in our code that no reference can possibly outlive the value it points to. Rust refers to creating a reference to some value as borrowing the value. What you have borrowed, you must eventually return to its owner.

    // As an example, let's suppose we're going to build a table of murderous Renaissance artists and the works they're known for. Rust's standard library includes a has table type, so we can define our type like this:
    use std::collections::HashMap;

    type Table = HashMap<String, Vec<String>>;

    // This is a hash table that maps String values to Vec<String> values, taking the name of an artist to a list of the names of their works. We can iterate over the entries of a HashMap with a for loop, so we can write a function to print out a Table for debugging:
    // fn show(table: Table) {
    //     for (artist, works) in table {
    //         println!("works by {}:", artist);

    //         for work in works {
    //             println!(" {}", work);
    //         }
    //     }
    // }

    // Constructing and print the table is straightforward:
    // The below should be in the main function as it is here, the show function should be outside of it. Leaving it for easier lesson learning.
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(), vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(), vec!["The Musicians".to_string(), "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(), vec!["Perseus with the head of Medusa".to_string(), "a salt cellar".to_string()]);

    // show(table);

    // Having learned about moves, the definition of show should raise a few questions. In particular, HashMap is not Copy, it can't be, since it owns a dynamically allocated table. So when the program calls show(table), the whole structure gets moved to the function, leaving the variable table uninitialized. If the calling code tries to use table now, it'll run into trouble:
    assert_eq!(table["Gesualdo"][0], "many madrigals");
    // The above will give us an error about moves and a value being used after a move.

    // If we look into the definition of show, the outer for loop takes ownership of the hash table and consumes it entirely. The inner for loop does the same to each of the vectors. Because of move semantics, we've completely destroyed the entire structure simply by trying to print it out. Thanks Rust!

    // The right way to handle this issues it to use references. A reference lets us access a value without affecting its ownership. References come in two kinds:
    // 1. A shared reference lets us read but not modify its referent. However, we can have as many shared references to a particular value at a time as we'd like. The expression &e yields a shared reference to e's value. If e has the type T, then &e has the type &T, pronounce "ref T". Shared references are Copy.
    // 2. If we have a mutable reference to a value, we may both read and modify the value. However, we may not have any other references of any sort to that value active at the same time. The expression &mut e yields a mutable reference to e's value. We write its type as &mut T, which is pronounced "ref mute T". Mutable references are not Copy.

    // We can think of the distinction between shared and mutable references as a way to enforce a multiple readers or single writer rule at compile time. This rule doesn't only apply to references, it covers the borrowed value's owner as well. As long as there are shared references to a value, not even its owner can modify it. The value is locked down. If there is a mutable reference to a value, it has exclusive access to the value. We can't use the owner at all, until the mutable reference goes away. Keeping sharing and mutation fully separate turns out to be essential to memory safety.

    // The printing function in our example above doesn't need to modify the table, just read its contents. So we can just pass it a shared reference to the table:
    // show(&table);

    // References are non-owning pointers, so the table variable remains the owner of the entire structure, show has just borrowed it for a bit. We'll also need to adjust the definition of show to match:
    fn show(table: &Table) {
        for (artist, works) in table {
            println!("works by {}:", artist);

            for work in works {
                println!(" {}", work);
            }
        }
    }

    // The type of show's parameter table has changed from Table to &Table. Instead of passing the table bu value (and hence moving ownership into the function), we're now passing a shared reference. How does this work within the body?
    // The original outer loop took ownership of the HashMap and consumed it, in our new version it receives a shared reference to the HashMap. Iterating over a shared reference (of HashMap) is defined to produce shared references to each entry's key and value. Artist has changed from a String to a &String, and works from a Vec<String> to a &Vec<String>.
    // The inner loop is changed similarly. Iterating over a shared ref to a vector is defined to produce shared refs to its elements. So work is now a &String. No ownership changes hands anywhere in the function, just a passing of non-owning references.

    // If we wanted to write a function to alphabetize the works of each artist, a shared reference doesn't suffice. Shared references don't permit modification. Instead, the sorting fucntion needs to take a mutable reference to the table.
    fn sort_works(table: &mut Table) {
        for (_artist, works) in table {
            works.sort();
        }
    }

    // To which we pass:
    sort_works(&mut table);

    // This mutable borrow grants sort_works the ablility to read and modify our structure as required by the vectors' sort method.
    
    // When we pass a value to a function in a way that moves ownership of the value to the function, we say that we have passed it 'by value'. If we instead pass the function a reference to the value, we say that we have passed the value 'by reference'. This is what we did above with our show function. Many languages make the difference with value vs reference, however if Rust it's very important as it pertains to how ownership is affected.
}
