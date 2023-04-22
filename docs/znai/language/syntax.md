# Overview

Bauble has a c-style syntax. 
It closely resembles JS. You may even notice no difference on the first sight.
However, Bauble is more fun.
Well, at least it has `fun` as a keyword.

# Literals

Bauble supports a few data types. 

* Nil
* Boolean
* Number
* String
* Array
* Function

```javascript {commentsType: "inline"}
nil // nil literal
true, false // Boolean literals
42, 3.14 // Number literals
"Hello, World" // String literal. Enclosed in double quotes
[0; 10] // Array of numbers size 10 with initial value 0
["a", 5] // Array of strings with initial value "a"
fun (x, y) { return x + y } // Function declaration
```

# Variables

Bauble supports global and local variables.
Local variables has lexical scoping. 

Variable declarations starts with `let` keyword.

```javascript {commentsType: "inline"}
  let global_greet = "Hello, world"; // Global variable

  {
    let local_greet = "Hello, block"; // Local variable. Won't be available after block ends
    let uninitialized; // When variable gets no immediate value, it is initialized to nil
    uninitialized = 42; // Initialize declared variable later
  }

  unknown = 10; // Error. You can't use variables that weren't declared
```

# Conditionals

Bauble supports `if-else` statements. 

```javacript {commentsType: "inline"}
let temperature = 21.5;

if (temperature > 25) {
  print "It's too hot";
} else if (temperature > 19) {
  print "It's perfect";
} else {
  print "It's cold";
}
```

# Loops

Bauble currently supports only `while` loops.

```javascript {commentsType: "inline"}
let i = 0;
let sum = 0;

while (i < 10) { // As in if statement we have condition in parentheses
  sum = sum + i;
  i = i + 1;
}  

print sum;
```

# Arrays

The size of an array in Bauble is fixed and can't be changed after array is created.
Also, you must specify the initial value of array when creating it.

```javascript {commentsType: "inline"}
let numbers = [0; 16]; // Array of zeroes of size 16

numbers[0] = 1; // Set 1 to first element of the array

print numbers[8]; // Access 9-th element of the array

let err = numbers[16]; // Accessing array beyond size results in runtime error
```

Virtual machine controls size of the array.

You can also use strings as arrays. You can read characters in a position.
However, you can't change the string.

```javascript {commentsType: "inline"}
  let greeting = "Hello";

  print greeting[0]; // Prints "H"
  greeting[0] = "J"; // You can't do that
```

# Functions

Functions are fun.

```javascript {commentsType: "inline"}
  fun fibonacci(n) { // Starts with `fun` keyword. Accepts list of parameters
    if (n >= 2) {
      return 1; // Early return supported
    }
    return fibonacci(n - 1) + fibonacci(n - 2); // Recursion supported
  }
```

If function does not return value explicitly, it will return `nil`

