use std::collections::HashMap;

pub fn hashmap_with_mutable_references(){
    println!("__hashmap");
    
    type Table = HashMap<String, Vec<String>>;

    let mut table = Table::new();
    table.insert("brancusi".to_string(), 
        vec![
            "coloana infinitului".to_string(), 
            "poarta tacerii".to_string()]
            );
    table.insert("Caravaggio".to_string(), 
        vec![
            "The musicians".to_string(),
            "The artists".to_string()
        ]);

    sort_works(&mut table);
    show(&table);
    assert_eq!(2, table.keys().count());



    fn show (table: &Table) {
        for (artist, works) in table {
            println!("works by {}", artist);
            for work in works {
                println!("    {}", work)
            }
        }
    }

    fn sort_works(table: &mut Table) {
        for (_artist, works) in table {
            works.sort();
        }
    }
}

pub fn dereferencing() {
    println!("__dereferencing");
    let x = 10;
    let r = &x;
    println!("    Accessing x via a reference: {:?}", *r);
}

pub fn mutable_references() {
    println!("__mutable_references");
    let mut x = 32;
    let y = &mut x;
    *y += 32;
    println!("    Mutated value: {:?}", *y);
}

pub fn assigning_references(b: bool) {
    println!("__assigning_references");
    let x = 10;
    let y = 20;

    let mut r = &x;

    if b {
        println!("    will reassign reference to y (20)");
        r = &y; 
    }

    println!("    Reference points to value: {:?}", *r);
}

#[derive(Debug)]
struct Point { x: i32, y: i32 }

pub fn references_to_references(){
    println!("__references_to_references");

    let point = Point { x: 100, y: 100 };
    let r: &Point = &point;
    let rr: &&Point = &&point;
    let rrr: &&&Point = &&&point;

    println!("point: {:?}", point);
    println!("r: {:?}", r );
    println!("rr: {:?}", rr );
    println!("rrr: {:?}", rrr );
}


pub fn assigning_to_reference_makes_it_point_to_another_value() {
    println!("__assigning_to_reference_makes_it_point_to_another_value");
    let x = 10; 
    let y = 20;

    let mut r = &x;
    println!("reference before assignment: {:?}", r);
    r = &y;
    println!("reference after assignment: {:?}", r);
}

pub fn references_to_arbitrary_expressions() {
    println!("__references_to_arbitrary_expressions");

    fn factorial(n: usize) -> usize {
        (1..n+1).fold(1, |a, b| a*b)
    }
    let r = &factorial(6);
    let result = r + 1009;
    assert_eq!(result, 1729);
    println!("result: {}", result);
}

// you can't borrow a reference to a local variable and take it out of the
// variable's scope
// the compiler will complain that x lives only in the inner block
pub fn borrow_a_local_variable_is_forbidden(){
    println!("__borrow_a_local_variable_is_forbidden");
    // {
    //     let r;
    //     {
    //         let x = 1;
    //         r = &x;
    //     }
    //     assert_eq!(*r, 1);
    // }
}

pub fn reference_lifetmes_good_example() {
    println!("__reference_lifetmes_good_example");

    {
        let r;
        {
            let x = 1;
            r = &x;
            assert_eq!(*r, 1);
        }
    }
}

static mut STASH: &i32 = &128;
static WORTH_POINTING_AT: i32 = 69;
pub fn store_globally(p: &'static i32){
    println!("__store_globally");
    unsafe {
        STASH = p;
        println!("Stored globally: {}", STASH);
    }
}

pub fn store_globally2(){
    println!("__store_globally2");
    store_globally(&WORTH_POINTING_AT);
}

pub fn store_globally3(){
    println!("__store_globally3");
    unsafe{
        STASH = &55;
        println!("Stored globally: {}", STASH);
    }
}

pub fn structs_containing_references(){
    println!("__structs_containing_references");
    struct S<'a>{
        r: &'a i32
    }

    let s;
    {
        let x=10;
        s = S { r: &x };
        assert_eq!(*s.r, 10);
    }

}