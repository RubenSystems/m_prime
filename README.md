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
SetReg { register: 0, constant: 1 } # Set r0 to 1
Add { rega: 0, regb: 0, outreg: 0 } # Redundant add operation (should have been removed)
Output(0)
```

This reduces the compiler's cost function from 14 to 4.

An example larger optimisation is seen using a loop. The below is a loop that counts to 1000. It has some redundant operations (the variable decl at the top), however, everything else is needed for the loop.

```
Var(0)
SetReg { register: 0, constant: 0 }
SetReg { register: 1, constant: 1 }
Add { rega: 0, regb: 1, outreg: 0 }
SetReg { register: 1, constant: 1000 }
Sub { rega: 0, regb: 1, outreg: 1 }
PCSetIfNotZero { register: 1, jump_point: 2 }
Output(0)
```

The MCTS optimiser will eliminate the loop entirely:

```
SetReg { register: 0, constant: 1000 }
Output(0)
```

The result of this optimisation is a 99% improvement on the compilers internal cost function.

### Automatic vectorisation

Automatic vectorisation is still very basic with mcts optimiser and an ongoing area of resarch. It cannot reliably idenfify vectoriastion opportunities. Another issue is that the search space starts to become **very** large (so large that my laptop can only handle so many iterations before it kills the process). A solution for this is a function approximator. Here is an example of a pairwise addition of two vectors:

Origial code:

```
# Initalise Vectors
Var(0)
SetReg { register: 0, constant: 1 }
Store { register: 0, variable: 0 }
Var(1)
SetReg { register: 0, constant: 2 }
Store { register: 0, variable: 1 }
Var(2)
SetReg { register: 0, constant: 3 }
Store { register: 0, variable: 2 }
Var(3)
SetReg { register: 0, constant: 4 }
Store { register: 0, variable: 3 }
Var(4)
SetReg { register: 0, constant: 5 }
Store { register: 0, variable: 4 }
Var(10)
SetReg { register: 0, constant: 1 }
Store { register: 0, variable: 10 }
Var(11)
SetReg { register: 0, constant: 2 }
Store { register: 0, variable: 11 }
Var(12)
SetReg { register: 0, constant: 3 }
Store { register: 0, variable: 12 }
Var(13)
SetReg { register: 0, constant: 4 }
Store { register: 0, variable: 13 }
Var(14)
SetReg { register: 0, constant: 5 }
Store { register: 0, variable: 14 }
Var(20)

# Pairwise addition + output
Load { register: 0, variable: 0 }
Load { register: 1, variable: 10 }
Add { rega: 0, regb: 1, outreg: 0 }
Store { register: 0, variable: 20 }
Output(0)
Var(21)
Load { register: 0, variable: 1 }
Load { register: 1, variable: 11 }
Add { rega: 0, regb: 1, outreg: 0 }
Store { register: 0, variable: 21 }
Output(0)
Var(22)
Load { register: 0, variable: 2 }
Load { register: 1, variable: 12 }
Add { rega: 0, regb: 1, outreg: 0 }
Store { register: 0, variable: 22 }
Output(0)
Var(23)
Load { register: 0, variable: 3 }
Load { register: 1, variable: 13 }
Add { rega: 0, regb: 1, outreg: 0 }
Store { register: 0, variable: 23 }
Output(0)
Var(24)
Load { register: 0, variable: 4 }
Load { register: 1, variable: 14 }
Add { rega: 0, regb: 1, outreg: 0 }
Store { register: 0, variable: 24 }
Output(0)
```

With automatic vectorisation:

```
# Initalise
Var(0)
SetReg { register: 0, constant: 1 }
Store { register: 0, variable: 0 }
Var(1)
SetReg { register: 0, constant: 2 }
Store { register: 0, variable: 1 }
Var(2)
SetReg { register: 0, constant: 3 }
Store { register: 0, variable: 2 }
Var(3)
SetReg { register: 0, constant: 4 }
Store { register: 0, variable: 3 }
Var(4)
SetReg { register: 0, constant: 5 }
Store { register: 0, variable: 4 }
Var(10)
SetReg { register: 0, constant: 1 }
Store { register: 0, variable: 10 }
Var(11)
SetReg { register: 0, constant: 2 }
Store { register: 0, variable: 11 }
Var(12)
SetReg { register: 0, constant: 3 }
Store { register: 0, variable: 12 }
Var(13)
SetReg { register: 0, constant: 4 }
Store { register: 0, variable: 13 }
Var(14)
SetReg { register: 0, constant: 5 }
Store { register: 0, variable: 14 }
Var(20)

# Pairwise
Load { register: 0, variable: 0 }
Load { register: 1, variable: 10 }
Add { rega: 0, regb: 1, outreg: 0 }
Store { register: 0, variable: 20 }
Output(0)
Var(21)
Load { register: 0, variable: 1 }
Load { register: 1, variable: 11 }
Add { rega: 0, regb: 1, outreg: 0 }
Store { register: 3, variable: 21 }
Output(0)
Var(22)
Load { register: 0, variable: 2 }
Load { register: 1, variable: 12 }
Add { rega: 0, regb: 1, outreg: 0 }
Store { register: 0, variable: 22 }
Output(0)
Add { rega: 1, regb: 0, outreg: 1 }

# Vectorised Addition
VecAdd { a1r: 0, b1r: 1, r1: 1, a2r: 2, b2r: 3, r2: 0 }

Load { register: 1, variable: 13 }
Add { rega: 0, regb: 1, outreg: 0 }
Add { rega: 0, regb: 1, outreg: 0 }
Output(0)
Var(24)
Load { register: 0, variable: 4 }
Load { register: 1, variable: 14 }
Add { rega: 0, regb: 1, outreg: 0 }
Store { register: 0, variable: 24 }
Output(0)
```

Despite there being 5 additions, the vectorised load has only occured once. Note, while the outputs of the programs are the same, the resultant memory is not. This is because mprime is only checking for output correctness, and asumes any other memory operation is unrelated. In the future, memory state will be preserved.
