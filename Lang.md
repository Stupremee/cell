# Specification

## Expressions

**Literals**

- Booleans
- Integers
- Floats
- Strings
- Characters

```
true, false
0, 0x123, 0b1001
0.0, 123e5, 0xFF.FF
"hello world", "cell"
'a', 'b', '\x41'
```

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
// `type def` creates a new struct like type
type def Foo = {
    val: String,
}

// This can be used to imlpement methods on a type
type Foo {
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
type trait SomeTrait = {
    def to_str(): String;
}

def generics[A: SomeTrait](val: A): String {
    return val.to_str();
}
```

Generics on a type work too

```
type def Array[T] = {
    // raw ptr type
    ptr: *T,
    // usz = usize
    len: usz,
}

type Array[T] {
    def push(elem: T) {
        *this.ptr = elem;
        this.ptr += 1;
        this.len += 1;
    }
}
```

**Unions**

```
type union Option[T] = {
    // The right side defines the data inside this variant
    Some = T,
    None,
}

type union Complex = {
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