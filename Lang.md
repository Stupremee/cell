# Specification

## Expressions

**Literals**

- Booleans
- Integers
- Floats
- Strings
- Characters
- Void / Unit

```
true, false
0, 0x123, 0b1001
0.0, 123e5, 0xFF.FF
"hello world", "cell"
'a', 'b', '\x41'
()
```

**Binary Operators**

The precedence increases the further down in the table the
precedence column is.

All binary operators are left associative,
except the `**` operator.
`**` is right associative.

| Operator | Operation    | Precedence     |
|:--------:|:------------:|:--------------:|
|    &&    | Logical AND  | Logical        |
|    ||    | Logical OR   | Logical        |
|    &     | Bitiwise AND | Bitwise        |
|    |     | Bitiwise OR  | Bitwise        |
|    ^     | Bitiwise XOR | Bitwise        |
|    >>    | Left shit    | Shifting       |
|    <<    | Right Shift  | Shifting       |
|    +     | Add          | Addition       |
|    -     | Substract    | Addtition      |
|    *     | Multiply     | Multiplication |
|    /     | Divide       | Multiplication |
|    %     | Modulus      | Multiplication |
|    **    | Power        | Exponents      |

**Comparison Operators**

`!=` and `==` have a lower precedence than the other comparison operators.
All other comparison operators have the same precedence and are left associative.

| Operator | Operation             |
|:--------:|:---------------------:|
|    ==    | Equal                 |
|    !=    | Not Equal             |
|    <     | Less than             |
|    >     | Greater than          |
|    <=    | Less than or equal    |
|    >=    | Greater than or equal |

**Unary Operators**

The precedence of all unary operators is the same and
is higher than every other binary or comparison operator.

All binary operators are right associative.

| Operator | Operation     |
|:--------:|:-------------:|
|    &     | Pointer of    |
|    *     | Dereference   |
|    !     | Logical NOT   |
|    !     | Bitiwise NOT  |
|    +     | Unary PLUS    |
|    -     | Unary MINUS   |

**Assignment Operators**

All assignment operators have the same precedence.
They are expressions, but will produce a `Unit` type

Every binary operator, except the `**`, `&&` and `||` can be used as an assignment.
Just append a `=`. E.g. `+=`, `%=`.

**Named tuples**

```
var tuple = {a: 123, b: "hello", c: false};

tuple.a;
tuple.b;
tuple.c;
```

**Unnamed tuples**

```
var tuple = {1, 2, 3};
tuple.0;
tuple.1;
tuple.2;
```

**If**

```
if a > b {
    ...
} else if !boolean {
    ...
} else {
    ...
}
```

**Match**

```
type def MyStruct {
    a: i32, b: i32, c: i32,
}

var result = match value {
    1 -> 2,
    // Inclusive ranges
    2..5 -> 3,
    MyStruct { a, b, c } -> {
        io.println("struct");
        return a + b + c;
    },
    else -> -1,
};

// Like if let in rust
if match 2 = result {
    io.println("num is 2");
}
```

**Loops**

```
while 1 < 2 {
    continue;
}

// Endless loop
while {
    break;
}
```

## Functions and types

```
// `type` creates a new struct like type
type Foo {
    val: String,
}

// This can be used to imlpement methods on a type
impl Foo {
    def execute(val: i32) {
        io.println("Executing...");
        black_box(val);
    }
}

// `type alias` defines a type alias
type alias Str = String;

def foo(a: i32, b: i32): String {
    // ...
}

// Define a trait
trait SomeTrait = {
    def to_str(): String;
}

def generics[A: SomeTrait](val: A): String {
    return val.to_str();
}
```

Generics on a type work too

```
type Array[T] {
    // raw ptr type
    ptr: *T,
    // usz = usize
    len: usz,
}

impl[T] Array[T] {
    def push(elem: T) {
        *this.ptr = elem;
        this.ptr += 1;
        this.len += 1;
    }
}
```

**Unions**

```
union Option[T] {
    // The right side defines the data inside this variant
    Some = T,
    None,
}

union Complex {
    First = {a: i32, b: i32, c: i32},
    Second = {String, Option[i32]},
}

// Matching and creating on Unions
var val = Complex:First.new({a: 1, b: 2, c: 3});
match my_union {
    Complex:First = tuple -> {
        // now the whole {a: i32, b: i32, c: i32} is bound to `tuple`
        tuple.0;
    },
    Complex:Second = {name, num} -> {
        // now `name` is the `String` and `num` the `Option[i32]`
    }
}
```
