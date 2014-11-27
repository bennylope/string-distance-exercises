// This is just a brief exploration of the syntax of Rust. Not
// making an attempt to learn the language in any useful way for the time
// being.

// Initially thought to use String types but Rust complained that the code
// needed to use clone(). We don't need to operate on the strings, just
// references to them, so using slices - `&str` - works much better.
fn string_distance(source: &str, target: &str) -> int {
    if source == target {
        return 0;  // early return requires a semi-colon
    }
    if source.len() == 0 {
        return target.len() as int;  // len returns a uint value, must be cast
    }
    if target.len() == 0 {
        return source.len() as int;
    }

    // Named variables will be immutable by default - specify that this one is mutable so we can increment later.
    let mut distance: int = 0;

    let longer: &str;
    let shorter: &str;

    if source.len() > target.len() {
        longer = source;
        shorter = target;
    } else {
        longer = target;
        shorter = source;
    }

    for x in range(0, shorter.len()) {
        if shorter.as_bytes()[x] != longer.as_bytes()[x] {
            println!("{} != {}", shorter.as_bytes()[x], longer.as_bytes()[x]);
            distance = distance + 1;
        }
    }

    println!("{}, {}!!", shorter, longer);

    // Subtracting 1 to account for unstripped newline that is not counted in the
    // len() value but is converted to a byte for comparisons sake.
    distance + longer.len() as int - shorter.len() as int - 1  // function return, no semi-colon
}

fn main() {
    // println is not a function, it's a macro, hence called with exclamation point
    println!("First string?");
    let source = std::io::stdin().read_line().ok().expect("Failed to read line");
    println!("Second string?");
    let target = std::io::stdin().read_line().ok().expect("Failed to read line");
    // Have to strip the newlines from the input strings

    let distance: int = string_distance(source.as_slice(), target.as_slice());
    let distance_fmt: String = distance.to_string();
    println!("{} has a distance of {} from {}", target, distance_fmt, source);
}
