# expr_parser done in Rust

I want to develop a tiny programming language with Rust, that has a mathematics-like syntax.
The syntax would look like this :

```
x :: Natural;
x := 0;


Axiom (x :: Natural) -> Integer;
func invert {
	return -x;
}
```

## Operator wiki

| Operator | Reference                                                                                                                                                 | Example                  |
| -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------ |
| :=       | Assignment operator<br>Is used to assign something to a variable                                                                                          | a := "Hello World!";     |
| ::       | Type-Resolution operator<br>Is used to define the type of a variable, a parameter. You should read it as "a belongs to the Natural set" or "a has type T" | a :: Natural;<br>a :: T; |
