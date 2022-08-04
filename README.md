# Rust Crash Course
Udemy course that goes over Rust fundamentals.

By Nathan Stocks

[github](https://github.com/CleanCut/ultimate_rust_crash_course)
[invaders](https://github.com/CleanCut/invaders)

---
### Setup
Run the rust and tree install files:
```shell 
bash install_rust.sh
bash install_tree.sh
```

---
### Create Project
```shell 
cargo new <name_of_project>
```

---
### Run Project
```shell 
cargo run <name_of_project>
```

---
### Scalar Types
There are unsigned and signed integers

| Unsigned | Signed |
|----------|--------|
| u8       | i8     |
| u16      | i16    |
| u32      | i32    |
| u64      | i64    |
| u128     | i128   |
| usize    | isize  |

usize is the size of
the platform's pointer
type.

Integer literals

| Type           | Value      |
|----------------|------------|
| Decimal        | 1000000    |
| Hex            | 0xdeadbeef |
| Octal          | 0o77543211 |
| Binary         | 0b11110011 |
| Byte (u8 only) | b'A'       |

For Decimal through Binary, you
can use any number of underscores
for readability. They will be ignored
when processed.
ex: 1_000_000 is the same as 1000000

Floating point
f32 and f64

FP literals look as expected:
Valid: 3.14159
Not Valid: .1

Literal Suffix
Can be directly inferred
like `let x: u16 = 5;`
Or can be suffixed in
the literal like
`let x = 5u16;`.
Underscored can be used
to help with readability
for these cases
`let x = 5_u16;`.

Boolean Type
Annotated with `bool`.
Values are `true` or `false`
Cannot have arithmetic performed
on them unless they are converted
beforehand like so: `true as u8`

Character Type
Is Misnamed!
Specified with `char`.
It is a singular unicode scalar value
that can be:
- a character from our alphabet
- a character from someone else's alphabet
- an ideograph
- a diacritic
- an emoji
- non printable unicode control character
    - can represent a sound or an action

Characters are 4 bytes (32 bits)

`[char]` = UCS-4 / UTF-32 string

They are represented by single quotes:
```rust 
let m_letter = 'a';
let i_delta = '∆';
let ideograph = '來';
let diacritic = 'ˆ';
let emoji = '👻';
```
---
### Compound Types
#### Tuple
```rust 
let info = (1, 3.3, 999);
```
When Typing:
```rust 
let info: (u8, f64, i32) = 1, 3.3, 999);
```

Field Access Expression
```rust 
let info = (1, 3.3, 999);
let jets = info.0;
let fuel = info.1;
let ammo = info.2;
// or
let (jets, fuel, ammo) = info;
```

Tuple has a limited arity to them (number it can contain).

#### Array
```rust 
let buf = [0, 1, 2];
// or
// 0 = value, 3 = how many
let buf = [0; 3];
```

When Typing
```rust 
let buf: [u8: 3] = [1, 2, 3];
```

Index values
```rust 
buf[0];
```

Arrays are limited to a size of 32.
This is because they live on the stack and are of fixed size.
Vectors (`Vec`) will be used more often.

---
### Control Flow
if expressions don't need parenthesis like c++
```rust 
if num == 5 {
    msg = "five";
}
```

Values in expressing must bee boolean.

Can chain using `else` expression
```rust 
if num == 5 {
    msg = "five";
} else if num == 4 {
    msg = "four";
} else {
    msg = "other";
}
```

Another way to set a value like this is
```rust 
msg = if num == 5 {
    "five"
} else if num == 4 {
    "four"
} else {
    "other"
};
```

Curly braces are always needed.
Ternary is not used in this language.

If expressions can be written in different ways:
```rust 
num = if a { b } else { c };

num = if a {
    if x { y } else { x }
} else {
    c
};
```
#### Loop

```rust 
// Break from loop
loop {
    break;
}

// 'bob is a label identifier
'bob: loop {
    loop {
        loop {
            // breaks out of bob
            break 'bob;
        }
    }
}
```

`continue` works like other languages,
but you can designate which loop
to continue with a label.
```rust 
loop {
    continue;
}

'bob: loop {
    loop {
        continue 'bob;
    }
}
```

`while` loops also exist
```rust 
while dizzy() {
    // do stuff
}

// Whiles are programmed like this under the hood:
loop {
    if !dizzy() { break }
    // do stuff
}

// Example of a do-while
loop {
    // do stuff
    if !dizzy() { break }
}
```

`for` loops iter over any iterable values like in other languages
```rust 
for num in [7,8,9].iter() {
    // do stuff
}

// for loops can deconstruct as well
let array = [(1,2), (3,4)];

for (x, y) in array.iter() {
    // do stuff
}

// exclusive range
for num in 0..50 {
    // do stuff
}

// inclusive range
for num in 0..=50 {
    // do stuff
}
```

---
### Strings

6 types in entire library
- str = string slice
- &str = borrowed string slice
    - literal strings are always a borrowed string slice
        - `let msg = "Hello 🌎";`
    - cannot be modified
- String
    - can be modified
    - created by conversion of borrowed string:
      ```rust 
      let msg = "abc".to_string();
      // or
      let msg = String::from("abc");
      ```

`&str` is essentially a pointer to some string bytes and a length.
`String` is a pointer to string bytes, a length, and a capacity.

Both are valid UTF-8 and cannot be indexed by char position.

To access the UTF-8 byte:
```rust 
// is indexable (good for simple English text)
word.bytes()

// Can use unicode-segmentation package
//  good for accessing graphemes
graphemes(my_string, true)
```

You can use the `.nth(3)` method in place of indexing in a string.

---
### Ownership
Gives it all of its safety measures and compile errors.

3 Rules:
1. Each value has an owner
2. Only one owner
3. Value gets dropped if owner goes out of scope

```rust 
let s1 = String::from("abc");

// the pointer, len, and capacity
//  of s1 is copied into s2 and adds
//  s2 on the stack.
// Then invalidates s1 (uninitialized)
// This "moves" the value of s1 to s2
let s2 = s1;

println("{}", s1);  // Error borrowed of moved value
```

####Stack
- In order
- fixed-size
- LIFO
- Fast

####Heap
- Unordered
- variable-size
- Unordered
- Slow

In above example, s1 and s2 exist
on the stack, and the value of
these exist on the heap.

```rust 
let s1 = String::from("abc");
// This creates a copy of the
//  stack and heap data, where
//  s2 pointer looks at the new
//  coppied heap data.
let s2 = s1.clone();
println!("{}", s1);
```

Note: Rust refers to "copy" when
stack data is copied. When heap
and pointer data is involved, then
"clone" is used. (clone is similar
to a deep copy)

When a value is dropped, then the
1. Destructor immediately runs
2. Heap portion is immediately freed
3. Stack portion is immediately popped

This gives us no leaks and no dangling pointers.

```rust 
let s1 = String::from("abc");
do_stuff(s1); // Error! s1 moved to local variable s

fn do_stuff(s: String) {
    // do stuff
}

// Fix: Not usually the pattern used
let mut s1 = String::from("abc");
s1 = do_stuff(s1);
println!("{}", s1);

fn do_stuff(s: String) -> String {
    s
}
```

Instead of the fix above, references are preferred.

---
### References & Borrowing

```rust 
let s1 = String::from("abc");
// &s1 passess a ref of s1
do_stuff(&s1);
println!("{}", s1);

// borrows a ref of s1
fn do_stuff(s: &String) {
    // do stuff
}
```

For above, what is being done is
the s parameter becomes a pointer
to s1.

####Lifetimes
References must always be valid.
They cannot outlive the data it references.

References are immutable:
```rust 
let mut s1 = String::from("abc");
do_stuff(&s1);

fn do_stuff(s: &String) {
    s.insert_str(0, "Hi, "); // Error
}
```

If the reference and value are both
mutable, then the reference can be
used to change the value.
```rust 
let mut s1 = String::from("abc");
do_stuff(&mut s1);

fn do_stuff(s: &mut String) {
    // Don't need to de-reference
    // '.' operator auto-dereferences
    // Manual dereference is (*)
    s.insert_string(0, "hi, ");
    *s = String::from("Replacement");
}
```

variable: x
immutable reference: &x
mutable reference: &mut x

Types work by the same syntax.

If variable is a mutable reference,
then de-referencing gives you mutable access.
If immutable reference, then de-referencing
gives an immutable access.

At any time you can have either
- Exactly one mutable reference
- or any number of immutable references

All these rules are enforced by the compiler.

---
### Structs
 ```rust 
 struct RedFox {
    enemy: bool,
    life: u8,
}

// Intantiate
let fox = RedFox {
    enemy: true,
    life: 70,
}

// typically an associated function is used:
impl RedFox {
    // "class method"
    // Self can be replaced with RedFox
    fn new() -> Self {
        Self {
            enemy: true,
            life: 70,
        }
    }
}

let fox = RedFox::new();
let life_left = fox.life;
fox.enemy = false;
fox.some_method();

// methods are also used in implementation block:
impl RedFox {
    // associated function
    fn function() ...
    // methods
    fn move(self) ...
    fn borrow(&self) ...
    fn mut_borrow(&mut self) ...
}
 ```

---
### Traits
```rust 
struct RedFox {
    enemy: bool,
    life: u32,
}

trait Noisy {
    fn get_noise(&self) -> &str;
}

impl Noisy for RedFox {
    fn get_noise(&self) -> &str { "Meow?" }
}

// The trait and implementation could be done at the same time:
impl RedFox {
    fn get_noise(&self) -> &str { "Meow?" }
}

// But once we have a trait, we can have generic traits
fn print_noise<T: Noisy>(item: T) {
    println!("{}", item.get_noise());
}

// Example:
impl Noisy for u8 {
    fn get_noise(&self) -> &str { "BYTE!" }
}

fn main() {
    print_noise(5_u8):  // prints "BYTE!"
}

// Special trait called Copy that copies instead of moves in move situations.
// If type is on HEAP it cannot use Copy.
// Traits implement inheritence
// Trait can inherit other traits.

// Traits can also have default behaviors.
// To do this, add block instead of semicolon
trait Run {
    fn run(&self);
    // should be the following if default is needed
    fn run(&self) {
        println!("I'm running!");
    }
}

struct Robot {}
// Don't provide definition of method when implementing
impl Run for Robot {}

fn main() {
    let robot = Robot {};
    robot.run();
}
```

---
### Collections
#### Vector<T>
```rust 
let mut v: Vec<i32> = Vec::new();
v.push(2);
v.push(4);
v.push(6);
let x = v.pop();  // x is 6
println!("{}", v[1]);  // prints "4"

// macro for creating vectors using literal values
let mut v = vec![2, 4, 6];
```

#### HashMap<K, V>
````rust 
let mut h: HashMap<u8, bool> = HashMap::new();
h.insert(5, true);
h.insert(6, false);
// .remove returns an Enum
let have_five = h.remove(&5).unwrap();
````

Many other collections:
- VecDeque
- LinkedList
- HashSet
- BinaryHeap
- BTreeMap
- BTreeSet

---

### Enums
Algebraic Data Types
```rust 
enum Color {
    Red,
    Green,
    Blue,
}
let color = Color::Red;
enum DispenserItem {
    Empty,
    Ammo(u8),
    Things(String, i32),
    Place {x: i32, y: i32},
}

use DispenserItem::*
let item = Empty;
let item = Ammo(69);
let item = Things("hat".to_string(), 7);

impl DispenserItem {
    fn display(&self) {}
}

// Represents when something is absent or present
enum Option<T> {
    Some(T),
    None,
}

// if let takes a pattern and if it is 
//  true, the variable is created for the
//  scope of the "if let" block.
if let Some(x) = my_variable {
    println!("value is {}", x);
}

// must have all options
match my_variable {
    Some(x) => {
        println!("value is {}", x);
    },
    None => {
        println!("no value");
    },
    // matches anything and can be used
    //  as a default
    _ => {
        println!("who cares");
    },
}

// function calls also work!
let x = match my_variable {
    Some(x) => x.squared() + 1,
    None => 42,
};
```

#### Option Enum
Def:
```rust 
enum Option<T> {
    Some(T),
    None,
}

// create None option
let mut x: Option<i32> = None;
x = Some(5);
x.is_some();  // true
x.is_none();  // false
for i in x {
    println!("{}", i);
}
```

#### Result Enum
```rust 
#[must_use]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

use std::fs::File;

fn main() {
    let res = File::open("foo");
    let f = res.unwrap();
    let f = res.expect("Error message");
    // Check to see if it is okay to prevent error
    if res.is_ok() {
        let f = res.unwrap();
    }
    match res {
        Ok(f) => { /* do stuff */ },
        Err(e) => { /* do stuff */ },
    }
}
```

---
### Closure
Syntax
```rust 
|x, y| {x + y}
```

Add using closure
```rust 
let add = |x, y| { x + y };

add(1, 2);  // returns 3
```


```rust 
// without params
|| { x + y }

// without block
|| {}
```

closures will borrow a reference to values
in enclosing scope
```rust 
let s = "🌶".to_string();

// Won't outlive this thread
// let f = || {
//     println!("{}", s);
// };

// force closure to move variabels
//  into itelf
let f = move || {
    println!("{}", s);
}

f();  // prints 🌶
```

Closures allow you to use functional
style programming in Rust

```rust 
let mut v = vec![2, 4, 6];

// sum of x*3 in v if x*3 > 10
v.iter()
    .map(|x| x * 3)
    .filter(|x| *x > 10)
    .fold(0, |acc, x| acc + x);
```

---
### Threads
Example
```rust 
use std::thread;

fn main() {
    
    let handle = thread::spawn(move || {
        // do stuff in child thread
    });
    
    // do stuff simultaneously in main thread
    
    // wait until thread has exited
    handle.join().unwrap();
}
```

Threading is heavyweight.
More threads you have can cause a lot of overhead

If you want to have some work to
continue to run while you are waiting
for something, then looking into *async
await*.

---
### More Sources
[Rust Docs](https://doc.rust-lang.org/book/title-page.html): A document source for Rust.

[Rustup](https://rust-lang.github.io/rustup/index.html)

[Rust Forge](https://forge.rust-lang.org/infra/other-installation-methods.html#rustup)

[rustlings](https://github.com/rust-lang/rustlings): Bug Fix challenges to improve Rust skills.

