# Ownership

A key feature of rust is the concept of ownership, as the latter is part of what keeps rust code secure. 
Ownership involves how the memory is managed in rust, and involvs a set of rules that are invoked during compilation.  This is in contrast to traditional garbage collection ("GC") where the GC is constantly looking for unused portions of memory, or even having the programmer manually manage memory. 

## The stack and the heap
Knowing whether a value is on the stack or the heap matters in rust. 

Both the stack and the heap are part of memory that is available to your program, but they are structured in different ways. 
1. The stack is ordered top to bottom, like a stack of pancakes.  Stacks follow a LIFO rule, last in first out.  New values are pushed (appended) to the rear (top) of the stack and are popped off when needed.  Pushing data to a stack requires the stack to have a known, fixed size.   Without the latter, the data is stored in the **heap**.  
2. The heap, by definition, is less organized than the stack.  In a heap, a certain amount of space is requested by the user and the OS tries finding a spot on the heap that's big enough, marks that spot as being in use, and then return a **pointer** to that location using an address. the latter process is called **allocating** onto the heap.  This is different that pushing values onto a stack.  while you can push the pointer onto the stack, because the pointer has a known, fixed size, you have to follow the pointer if you want the actual data. 
3. Pushing to a stack is faster than allocating to the heap, as the OS does not have to search for an open portion of memory to store data and then has to do the bookkeeping necessary to keep track of pointers. Following these pointers can slow a computer down, as it is hopping around in memory rather than getting everything from one stack or collection. Simply allocating large enough spaces on a heap takes time. 
4. when code calls a function, the data passed into the function  (including pointers) gets pushed onto the stack along with any local variables in the function.  This data gets popped off the stack when the function completes. 
5.  the problems of adequately managing the data on the heap are important.  you need to track what parts of data are in the heap, minimuzing duplicate data, and cleaning up unused data.  managing heap data is why ownership exists

## Ownership rules

* each value in rust has a variable called an *owner*
* there can only be one owner at a time
* when an owner goes out of scope, the value will be dropped

## Variable Scope

if you define a variable:
* *let s = "Hello";*

the scope of s is non-existant before being declared, but when it is finally declared it is valid from that point onward
Now, after you do a thing with s, the *scope ends*

* a variable is valid from when it is declared
* it stays valid until it goes out of scope

## The *String* Type
This is an example of how ownership works.

If we don't want to use a string literal, due to it being immutable or if we want to store user input, we can use other version of the string. There's the String type we can use, which is stored on the  heap and therefor can store a range in memory. 

`
let s = String::from("hello");
`

the double-quotes essentially allows us to mutate the string type, and the difference is how these two types handle memory.  

## Memory and Allocation

With the above String type, there needs to be an amount of memory allocated to the heap, an amount that is unknown at compile time.  What this style of memory allocation means is:
* at runtime, memory must be allocated by the OS
* when we are done using the String, we need a way to return this memory to the OS

The first part is done when we call `String::from`, as it requests memory it needs. 

The second part is hard because Rust has no garbage collection built-in.  Traditionally the user has had to take care of managing memory on their own, which is difficult and allows bugs to be introduced easily.  Rust, on the other hand, is built to return memory once a variable is out of scope.  (This is likely one reason why rust complains when you initialize a variable to don't use it; this makes it harder to free up the memory used by the variable as it's  not clear where that variable runs out of scope. )  

when a variable finally goes out of scope, rust will call the `drop` function, and is invoked in the background at the end of a curly  bracket.

Now, suppose we wanted to define a string using `String::from('hello')` per the above.  Under the hood, this String is made up of three parts:
* a pointer to where the string is in memory
* the length - len() - of the string; this is how much memory the contents of the string is using
* the capacity of the string; how much memory the `String`, in bytes, the `String` has received from the OS
