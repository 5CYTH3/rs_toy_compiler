# expr_parser done in Rust

I want to develop a tiny programming language with Rust and LLVM, that has a mathematics-like syntax.
The syntax would look like this :

```
def x :: {Natural} := 0;


Axiom (x :: {Natural}) -> {Decimal}
func invert {
	return -x
}
```

## Operator wiki
| Operator | Reference                                                                                                                                              | Example                       |
|----------|--------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------|
| :=       | Assignment operator Is used to assign something to a variable                                                                                          | a :: String := "Hello World!" |
| ::       | Type-Resolution operator Is used to define the type of a variable, a parameter. You should read it as "a belongs to the Natural set" or "a has type T" | a :: Natural a :: T           |
