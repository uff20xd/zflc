# Zfgc
A Compiler for the Zaffall programming language.

The zaffall programming language is a **low level language** that can be configured with so called modes,
which are applied at compile time. These allow for any customization that is needed at the moment. 

## Current State
Currently the compiler doesnt compile at all.

## Hello, World!
```
@!apply raw

get ende::io::println;

fn main() ()?Error {
    println("Hello, world!");
    return ();
}
```

## Refrences
