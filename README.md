# expr_parser done in Rust

I want to develop an expression parser with Rust and LLVM, that will compile the expression to machine code. If possible, I want to add kind of functions to it.

The syntax would look like this :

```
def x :: {Natural} := 0;


Axiom (x :: {Natural}) -> {Decimal}
func invert {
	return -x
}
```
