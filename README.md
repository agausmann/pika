# Pika

A refreshing systems programming language.

Example: [`bluetooth.pika`](rspika/src/examples/kattis/bluetooth.pika)

```
enum Tooth {
    Healthy,
    Blue,
    Missing,
}

// [left_right][lower_upper][tooth_index]
fn bluetooth(teeth: [[[Tooth; 8]; 2]; 2]) -> i32
{
    let mut blue = [false; 2];
    let mut jaw = [[false; 2]; 2];

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..8 {
                if teeth[i][j][k] == Tooth::Blue {
                    blue[i] = true;
                } else if teeth[i][j][k] == Tooth::Healthy {
                    jaw[i][j] = true;
                }
            }
        }
    }

    for i in 0..2 {
        if !blue[i] && jaw[i][0] && jaw[i][1] {
            return i;
        }
    }
    return 2;
}
```

I don't know what exactly this language is going to be, but I do know what it is
not.

## The Airing of Grievances [What Pika is Not]

> _I've got a lot of problems with you people, now you're gonna hear about them!_

- **Pika is not C** - While I appreciate the relatively simple semantics and
  "portability" of C, it has quite a few warts that I want to avoid:

  - It has many syntax choices that make it more annoying to parse for both
    humans and computers. In general, the type of a declaration is tightly
    intertwined with the declaration itself. Arrays, pointers, function
    pointers, oh my!
  
    All types should be writable using a standalone, consistent syntax. When it
    is part of a declaration, types should have the same syntax as everywhere
    else, and should be easily separated from the declaration.

    Array variable declarations shouldn't have syntax that is different from
    normal variables. Arrays should be a first-class type, not a special case.

    But, functions _should_ have syntax that is different from variables. The
    parser (human or machine) shouldn't be left in limbo reading a declaration
    and not being sure whether it is a function or a variable until the end.
    (Instead, start declarations with keywords like `let` / `var` / `fn` /
    `def`)

  - Having to remember what size each integer type is on each target platform.
    This makes the universal "portable assembly" language unportable in my view.
    Yes, it does have `stdint.h`, but that is more of a band-aid solution that
    doesn't satisfy me, I want to have well-defined fixed-size integer types
    from the start.

- **Pika is not C++** - Many of the same grievances as C, and:
  
  - Classes - C++ pigeon-holes a very specific form of dynamic dispatch with
    OOP, classes and all of the hidden complexities that underly them. I would
    rather write my own vtables than deal with classes.

  - Exceptions - Yet another hidden cost; exception control flow is complexity
    that I don't want to deal with as a compiler developer or language user.

  The one thing I do like about C++ is scope-based resource management (aka
  RAII), I may add that at some point but it is not a priority in the early
  stages.

- **Pika is not Rust** - Rust is a language with very powerful but very complex
  features. Its complexity makes it almost unapproachable if you want to write a
  compiler from scratch, especially as a solo developer.

  That being said, I do love the syntax of Rust much more than C. If you are
  familiar with Rust, you will notice that Pika adopts a large amount of syntax
  from Rust. I am mostly satisfied with Rust's syntax; reinventing syntax is not
  one of the goals of this project.
