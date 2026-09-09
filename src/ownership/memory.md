# Basic Concepts of Memory Management
##  Memory

* Ownership is a set of rules that the compiler checks for to ensure the program will be free of memory errors
* Memory refers to the area of your computer that is responsible for storing the information your program use
* It's ideal to free memory when it is no longer in use
* Programming languages implement different strategies for memory management

## Manual Memory Management

* In languages like C and C++, the programmer is responsible for allocating (requesting memory) and deallocating it (giving it back to the computer)
* Unfortunately, human beings make mistakes

  * Forgetting to deallocate memory that has been allocated
  * Trying to deallocate memory that has already been deallocated

## Automatic Garbage Collection

* Languages like Java, Python, Ruby and Go implement a tool called the garbage collector
* The garbage collector looks for data that is no longer in use and deallocates it. It "automates" the cleanup process
* The garbage collector itself occupies memory and can run at disadvantageous times

## Ownership

* Rust introduces a new paradigm: ownersehip
* Ownership is a set of rules on how Rust manages your computer's memory
* The Rust compiler does not compile the program if an ownership rule is violated
* Best of all worlds: the speed of a language like C but with less room for error
* The purpose of owensrship is nto assign responsibility for deallocating memory (primarily heap memory)
* Ownership is a compiler feature for reducing duplicate heap data and cleaning up heap data that is no longer needer

## What is ownership

* The owner is who/what is responsible for cleaning up a piece of data when it is no longer in use
* Every value in a Rust program has one owner
* The owner can change over the course of the program, but there is only 1 owner for a value at a time
* The owner is usually a name

  * A variable can be a owner
  * A parameter can be a owner
* Ownership also extends to composite types that own their elements

  * A tuple and array own their values

## The Stack and the Heap

* The stack and the heap are 2 different parts/regions of the computer's memory
* The stack and heap read and write data in different ways that offer advantages and disadvantages
* The stack is generally faster, but it only supports data of a fixed, predictable constant size and also that size must be know at compile time
* The heap is generally slower but it supports dynamic data that can be change in size over the program execution

## The Stack

* A stack stores values in the sequential order it receives them
* A stack is last in, first out (LIFO). The last item added is the first one removed
* The technical terminology for adding data is pushing onto the stack
* The technical terminology for removing data is popping of the stack
* All stack data has a fixed, consistent size that is known at compile time
* Data types like integers, floats, booleans, characters, arrays and tuples hace a fixed size. Rust stores them on the stack at runtime
* The piece of data on the stack will not grow or shrink in size as the program runs

## The Heap

* The heap is a large area of storage space. think of it like a warehouse
* The heap is for data whose size is not know at compile time (user input, a file's content, etc)
* When the Rust program needs dynamic space, it request it from the heap. A program called the memory allocator finds an empty spot that is large enough to store the data
* Allocating on the heap is slower than pushing to the stack. The memory allocator has to spend time searching for an open spot large enough to fit the data
* Accesing data is faster on the stack than heap as well. With a heap, the program has to follow the pointer to find the memory address
* A stack stores the data in sequence, so there is less "jumping around" from point to point

## References

* The memory allocator returns a reference, which is an address
* The reference points to the memory address of the data
* Think of a parking lot giving you a reference (spot "H25") when they park your car
* We can store a reference in a variable in a Rust program. References have a fixed size, so Rust stores them on the stack