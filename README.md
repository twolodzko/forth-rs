# Forth\.rs

Forth\.rs reads as *fortress* /fôr′trĭs/. It is a minimal-ish implementation of the [Forth] language interpreter.
Forth is a simple programming language coming from the time when code was written in uppercase (early 70').
For me, the biggest help in learning Forth was the [*Starting Forth*] book by Leo Brodie and the marvelous
[*Simple Forth*] tutorial by Leo Wong, both are freely available online. From other resources, there is also the
[standard] which is rather dry, nice [*Easy Forth*] tutorial, and [*Learn X in Y minutes*] for Forth.

## Reverse Polish notation

Nowadays, using the [most popular notation], you would write a simple mathematical formula as 
$1 + 3 \times 2$. Many programming languages do the same, parsing the expression `1 + 3 * 2` 
as something like `1.add(3.mul(2))`. This needs the parser to understand the [order or operations], making its job
slightly more complicated. But that's not the only notation possible. Lisps and some other languages use
[Polish notation], which could write the formula above as $+\  1 \times 3 \ 2$, or in Lisp syntax as `(+ 1 (* 3 2))`.
The notation does not need parentheses for the user to understand the order of operation, because the operators
precede the arguments. Forth, on another hand, uses [reverse Polish notation], where the operators follow the arguments.
The language does not use parentheses (like Lisps), so we would need to be explicit about the order of operations
and write the formula above as `3 2 * 1 +`.

## The stack

Forth is a [stack-based] language, which means that it uses the [*stack*] for passing the parameters. We can split the
`3 2 * 1 +` arithmetic operation, to the following steps

 0. We start processing the expression `3 2 * 1 +` with the empty stack.
 1. The number `3` is pushed to the stack and we continue processing `2 * 1 +`.
 2. The number `2` is pushed to the stack and we continue processing `* 1 +`.
 3. `*` is an operation that takes two arguments, so it first retrieves `2` from the top of the stack, and then `3`. 
 It multiplies the two arguments and pushes the result `6` to the stack. Next, it continues evaluating `1 +`.
 4. The number `1` is pushed to the stack and we continue evaluating `+`.
 5. `+` is an operation that retrieves `1` and `6` from the stack and adds them, pushing the result to the stack.

One important thing to notice is that the operators "communicate" by passing values through the shared stack. In this
implementation, the stack is just Rust's [`std::vec::Vec`] array of integers.

## Words

But how does Forth know what to do with `1` or `+`? For it's interpreter, both are *words*. The words are separated 
by whitespaces. In most cases, the work of the parser is trivial, as it just needs to read whatever input until
the whitespace as a word. After reading it, the word is interpreted. First, the interpreter tries searching for it in
the *dictionary* which maps words to things like functions or constants. In this implementation, the dictionary is
Rust's [`std::collections::HashMap`] hash map.

* If it finds the word in the dictionary, its definition is retrieved and executed. For example, it retrieves
a function and the function is executed, or it retrieves a constant and its value is pushed to the stack.
* If the word is not found, it tries parsing it as a number and pushing the result to the stack.
* Otherwise, it fails.

What about other data types than numbers? There are no other data types. Forth only uses integers, take it or leave it.
To be fair, the language was evolving and gradually introducing new types (like floats or strings), but this
implementation follows the classic, hardcore path. Zero is treated as binary false and every other value as binary true.

## Beyond words

Most of the time, the life of the interpreter is simple: read a word, evaluate it, and proceed to next word. 
Unfortunately, not everything can be expressed like this.

The simplest example is comments, where we start a comment with the `(` word and want the interpreter to ignore
everything until the comment ends`)`. In such a case, `(` informs the interpreter how to treat things *following* it.

Another example is loops. The `begin ... again` code block is delimited by its start `begin`, the loop body `...`, and
is ended by the `again` mark.

`variable name` reserves memory and defines a new word `name` that points to the memory location. Since
the variable name did not exist before creating it, it couldn't precede the `variable` definition.

## Beyond the stack and the dictionary

I mentioned above the `variable` keyword which "reserves memory". Other than the stack and dictionary, Forth also has
long-term *memory*. The classical Forth implementations reserved space in the computer memory, but in my
implementation, the memory is treated as Rust's [`std::vec::Vec`] array. New values can be pushed to the memory,
the old values can be retrieved or changed. To operate on the memory, we need to know the memory address of the 
values (array index). For example, `variable foo` creates the variable `foo` and reserves some location in the memory
for its content. Calling the `foo` word would return the memory location. We could use `!` word to push some value
to the location `42 foo !`, or `@` to retrieve the content of the location `foo @` and push it into the stack.

There is also the *return stack*, which can be used as a secondary, temporary memory. Because we have two, the regular
stack formally is called the *data stack*. It can be manipulated using the words `>r` (move the value from the stack to
the return stack), `r>` (move the value from the return stack to the data stack), `r@` (copy the value from the return
stack to the data stack). Forth has a special use for the return stack in counted loops. The loop `10 0 do ... loop`
would iterate from 0 to 10 each time executing the body `...`. The current loop index is pushed to the return stack.
The special keyword `i` copies the index from the return stack to the data stack. For nested loops, we could also use
`j` to copy the index of the outer loop to the stack. Unfortunately, no syntax shortcuts are available for additional
levels of nesting. The [*Simple Forth*] tutorial mentions the following [rules for using the return stack]

> Your Forth almost certainly uses the return stack for its own purposes, so your use of the return stack must follow certain rules:
> 1. Data put on the return stack must be taken back within the *same word*.
> 2. Data put on the return stack outside a `?DO (DO) ... LOOP (+LOOP)` cannot be accessed within the loop.
> 3. Data put on the return stack within a `?DO (DO) ... LOOP (+LOOP)` must be taken back before leaving the loop.
> 4. Data cannot be on the return stack when executing `I` or `J` in a loop.

If you break the rules, unexpected things may happen, but they are not enforced anyhow.


 [Forth]: https://en.wikipedia.org/wiki/Forth_(programming_language)
 [*Starting Forth*]: https://www.forth.com/starting-forth/
 [*Simple Forth*]: http://www.murphywong.net/hello/simple.htm
 [standard]: https://forth-standard.org
 [*Easy Forth*]: https://skilldrick.github.io/easyforth/
 [*Learn X in Y minutes*]: https://learnxinyminutes.com/docs/forth/
 [most popular notation]: https://en.wikipedia.org/wiki/Infix_notation
 [order or operations]: https://en.wikipedia.org/wiki/Order_of_operations
 [Polish notation]: https://en.wikipedia.org/wiki/Polish_notation
 [reverse Polish notation]: https://en.wikipedia.org/wiki/Reverse_Polish_notation
 [stack-based]: https://en.wikipedia.org/wiki/Stack-oriented_programming
 [*stack*]: https://www.forth.com/starting-forth/1-forth-stacks-dictionary/#The_Stack_Forth8217s_Workspace_for_Arithmetic
 [`std::vec::Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
 [rules for using the return stack]: http://www.murphywong.net/hello/simple.htm#L20
 [`std::collections::HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
