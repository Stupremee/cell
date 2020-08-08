# Cell

**Cell is WIP and many features don't work or are not even implemented**

Cell is a statically typed, general purpose systems programming language.

## What Cell looks like

```python
def fib(n: i32): i32 {
  return match n {
    0 -> 0,
    1 -> 1,
    else -> fib(n - 1) + fib(n - 2),
  };
}

type Array[T] {
  ptr: *T,
  len: u32,
}

// The current plan is to not support methods in structs.
// Will probably change in the future
impl[T] Array[T] {
  def push(elem: T) {
    *this.ptr = elem;
    this.ptr += 1;
    this.len += 1;
  }
}

type alias Int = i32;
```

## License

This project is licensed under the [Zlib](https://www.zlib.net/zlib_license.html) license.
