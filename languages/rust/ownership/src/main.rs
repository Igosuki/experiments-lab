
#[allow(unused_variables)]
// Example code around ownership
fn main() {
    // Scope
    {
        // s not in scope (block)
        let s = "somestr"; // s enters scope, allocated on stack, is type &str
        // s leaves scope
    }

    // Heap allocation
    {
        let s = String::from("somestr"); // s enters scope, is type String
        let mut s2 = String::from("somestr2"); // on the heap, mutable
        s2.push_str("lul");
    } // drop called here for all variables going out of scope

    // Move
    {
        let x = 5;
        let y = x;
        // Both x and y pushed onto stack, value is copied

        let s1 = String::from("hello");
        let s2 = s1; // Beyond this point, s1 cannot be read as the ptr has changed owners
        // Value of s1 is copied, but value s1 is {ptr, len, cap} and so ptr is a value that points to an address on the heap

        // Moving : copy to new var, invalidate first var
    }

    // Clone
    {
        let s1 = String::from("hello");
        let s2 = s1.clone(); // All data is copied, including the pointed data
        // Both values are accessible

        println!("s1 {}, s2{} ", s1, s2);
    }

    // Copy
    // Anything that has Copy can be copied onto the stack
    // Cannot be applied to anything that has Drop (typically anything that was allocated)

    // Functions
    {
        // Passing a variable to a function gives it ownership
        let s = String::from("lul");
        let s2 = do_stuff(s); // s moved onto s2 through function return value
        println!("{}", s2);
    }
    fn do_stuff(s: String) -> String {
        println!("{}", s);
        // drop is called on s or if a return is made moves ownership to call site!
        s
    }

    // References
    // Reassigning through returns is too tedious, therefore references have been implemented
    {
        let s = String::from("lul");
        let thelen = do_stuff_ref(&s); // sending a ref like this is called borrowing, borrowed immutable data can't be mut
        println!("{}", thelen);
    }
    fn do_stuff_ref(s: &String) -> usize {
        println!("{}", s);
        // references aren't dropped because this fn is not the owner
        s.len()
    }
    {
        let mut s = String::from("lul");
        do_stuff_mut(&mut s); // sending a ref like this is called borrowing, borrowed immutable data can't be mut
        println!("{}", s);
    }
    fn do_stuff_mut(s: &mut String) {
        s.push_str("lul");
    }

    // Data races
    {
        let mut s = String::from("hello");

        {
            let r1 = &mut s;

        } // r1 goes out of scope here, so we can make a new reference with no problems.

        let r2 = &mut s;
        // let r3 = &mut s; would be invalid because two mutable references would exist in the same scope
    }

    // Dangling references
    //    fn dangle() -> &String {
    //        let s = String::from("hello");
    //        &s // s goes out of scope, so this is invalid
    //    }

    // Slices
    {

        let mut s = String::from("hello world");
        s.push_str("omega");
        let hello = &s[0..5]; // reference to part of s
        let world = &s[6..11];
        let slice1 = &s[..]; // entire string
        let slice2 = &s[0..]; // from start to ?
        let slice3 = &s[0..1]; // from 0 to 1
        let slice4 = &s[..1]; // up to 1

        // s.clear() -> invalid because immutably borrowed above
        println!("{}", slice1);
    }


}