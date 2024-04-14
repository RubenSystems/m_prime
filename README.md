# M'

An experimental bytecode interpreter.

Designed to test compiler optimisations.

Example unoptimised program to add two numbers and then output the result, noting that registers and variables always initalise to 0:

```
Var(0) # initalise an empty var named 0;
SetReg { register: 0, constant: 0 } # Set register 0 to 0
SetReg { register: 1, constant: 1 } # Set r1 to 1
Add { rega: 0, regb: 1, outreg: 0 } # Add r0 to r1, store result in r0
Store { register: 0, variable: 0 } # Store r0 to v0
Load { register: 0, variable: 0 } # Load r0 from v0
Add { rega: 0, regb: 1, outreg: 0 } # Add r0 to r1, store result in r0
Store { register: 0, variable: 0 } # Store r0 to v0
Load { register: 0, variable: 0 } # Load r0 from v0
Output(0) # output v0
```

There are several redundant memory operations. Removing them will improve the performance of the program.

### MCTS optimiser

This uses machine learning to find optimisations which can be applied to the program. The resultant program is:

```
SetReg { register: 1, constant: 1 } # Set r1 to 1
Add { rega: 0, regb: 1, outreg: 0 } # Add r0 to r1, store result in r0
Add { rega: 0, regb: 1, outreg: 0 } # Add r0 to r1, store result in r0
Output(0) # output v0
```

This reduces the compiler's cost function from 14 to 4.
