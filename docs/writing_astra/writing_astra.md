# Astra: a cross-platform fuzzer.

In this article you will learn how to write a simple, yet efficient, cross-platform (desktop) fuzzer. Thanks to Rust and its great portability, it should be very easy for you to export this fuzzer on even more platforms. The goal of this article is purely educational, to learn how to design and write a fuzzer from scratch. If you want a state-of-the-art fuzzer, have a look at [AFL++](https://github.com/AFLplusplus/AFLplusplus)  or [LibAFL](https://github.com/AFLplusplus/LibAFL).

The first step in this journey is to understand what a fuzzer is. Most fuzzers share common features: they execute a program under test, feed them mutated inputs and observe the result of their execution. The ideal beginning point to understand fuzzer design is to start by reading the [afl technical whitepaper](http://lcamtuf.coredump.cx/afl/technical_details.txt) and the [historical notes](https://github.com/google/AFL/blob/master/docs/historical_notes.txt). In these documents, the author describes their desire to make a fuzzer that will be: **fast**, **simple** to use, **reliable**, and **chainable**. 

Most of the features AFL offers are possible because of the coverage measurement, which allows to track branch (edge) coverage from the program under test, in this context this measurement is called a **trace**. The fuzzer maintains a global map of tuples seen in previous executions, which can be compared with an individual trace. When a mutated input produces an execution trace producing new tuples, it is preserved and routed for additional processing later on. 

Along with new tuples, the fuzzer also considers coarse tuple hit counts, divided into several buckets:  `1`, `2`, `3`, `4-7`, `8-15`, `16-31`, `32-127`, `128+`. Changes between the range of a single bucket are ignored while transitions from one bucket to another are flagged as interesting. The hit count behavior provides a way to distinguish between potentially interesting control flow changes, such as a block of code being executed twice when it was normally hit only once. 

The fuzzer enforces aggressive timeouts, roughly **5x** the initially-calibrated execution speed rounded to 20ms. This prevents dramatic fuzzer performance degradation by descending into tarpits that would not significantly improve the coverage while being **100x** slower.

Mutated test cases that produced new state transitions within the program are added to the input queue and used as a starting point for future rounds of mutation. They supplement but do not automatically replace existing finds. The progressive state exploration approach outlined above means that some of the test cases synthesized later on in the game may have edge coverage that is a strict superset of the coverage provided by their ancestors. 

To optimize the fuzzing effort, AFL periodically re-evaluates the queue using a fast algorithm that selects a smaller subset of test cases that still cover every tuple seen so far, and whose characteristics make them particularly favorable to the tool. This algorithm basically assigns a score for every entry based on multiple criterias such as: execution time, filesize. The **non-favorite** testcases are not disregarded completely, the odds of executing them is reduced to 1% if there is non-fuzzed favorite testcase left. Otherwise their execution chance grows to 5% if there is no new-favorite, and 25% for a non-favorite that has never been through a fuzzing round. This method is called by the author: **culling the corpus**. 

The testcase file size has a huge impact on the fuzzer performances, not only because it makes the program under test slower but it also reduces the chance to mutate an interesting part of the testcase rather than redundant data blocks. Aside from low-quality testcases, some mutations can iteratively increase the size of the testcases. The built-in trimmer attempts to sequentially remove blocks of data with variable length and stepover; any deletion that doesn't affect the checksum of the trace map is committed to disk. 

Along with the built-in trimmer, a standalone `afl-tmin` tool exists, it is using a more exhaustive and iterative algorithm and attempts to perform alphabet normalization on the trimmed files. Text normalization is the process of transforming text into a single canonical form that it might not have had before, think of: `madame curie` \-\> `radium came`.  
The tool will select the operating mode: 

1. If the testcase produces a crash it will keep any tweak that still produces the crash.  
2. If not, then it will run the instrumented program under test and keep only the tweaks that produce exactly the same execution path.

The fuzzing strategies, ergo which mutations are interesting or not, are detailed in the blogpost: [binary fuzzing strategies: what works, what doesn't](https://lcamtuf.blogspot.com/2014/08/binary-fuzzing-strategies-what-works.html). The feedback (code-coverage) provided by the instrumentation makes it easy to understand the value of various fuzzing strategies. AFL starts by a series of deterministic mutation strategies:

1. **Walking bit flips:** the first and most rudimentary strategy, it involves performing sequential, ordered bit flips. The stepover is always one bit, the number of bits flipped in a row varies from one to four. Across a large and diverse corpus of input files, the observed yields are:  
   1.     Flipping a single bit: \~70 new paths per one million generated inputs,  
   2.     Flipping two bits in a row: \~20 additional paths per million generated inputs,  
   3.     Flipping four bits in a row: \~10 additional paths per million inputs. 

2. **Walking byte flips:** a natural extension of the walking bit flip approach, this method relies on 8-, 16-, or 32-bit wide bitflips with a constant stepover of one byte.   
   1. This strategy discovers around \~30 additional paths per million inputs, on top of what could have been triggered with shorter bit flips. 

3. **Simple arithmetics:** to trigger more complex conditions in a deterministic fashion, the third stage attempts to subtly increment or decrement existing integer values in the input file, this is done with a stepover of one byte.

4. **Known integers**: the last deterministic approach employed by afl relies on a hardcoded set of integers chosen for their demonstrably elevated likelihood of triggering edge conditions in typical code (e.g., `-1`, `256`, `1024`, `MAX_INT-1`, `MAX_INT`). The fuzzer uses a stepover of one byte to sequentially overwrite existing data in the input file with one of the approximately two dozen "interesting" values, using both endians (the writes are `8`, `16`, and `32-bit` wide). 

5. **Stacked tweaks:** with deterministic strategies exhausted for a particular input file, the fuzzer continues with a never-ending loop of randomized operations that consist of a stacked sequence of:  
   1. **Single-bit flips:** Attempts to set "interesting" bytes, words, or dwords (both endians)  
   2. **Addition or subtraction of small integers to bytes:** words, or dwords (both endians)  
   3. Completely random single-byte sets  
   4. Block deletion  
   5. Block duplication via overwrite or insertion  
   6. Block memset

From the feedback obtained by the instrumentation it’s easy to identify syntax tokens, the detailed approach is detailed in the blogpost: [afl-fuzz: making up grammar with a dictionary in hand](https://lcamtuf.blogspot.com/2015/01/afl-fuzz-making-up-grammar-with.html). In essence mutation strategies are efficient for binary format but perform poorly when it comes to highly structured input such as text, messages or language. The algorithm can identify a syntax token by piggybacking on top of the deterministic, sequential bit flips that are already being performed across the entire file. 

It works by identifying runs of bytes that satisfy a simple property: that flipping them triggers an execution path that is distinct from the product of flipping stuff in the neighboring regions, yet consistent across the entire sequence of bytes. 

The fuzzer is attempting to de-dup crashes and consider a crash unique if any of two  
conditions are met:

- The crash trace includes a tuple not seen in any of the previous crashes.  
- The crash trace is missing a tuple that was always present in earlier faults.

AFL tries to address exploitability by providing a [crash exploration](https://lcamtuf.blogspot.com/2014/11/afl-fuzz-crash-exploration-mode.html) mode where a known-faulting test case is fuzzed in a manner very similar to the normal operation of the  
fuzzer, but with a constraint that causes any non-crashing mutations to be  
thrown away. Mutations that stop the crash from happening are thrown away, so are the ones that do not alter the execution path in any appreciable way. The occasional mutation that makes the crash happen in a subtly different way will be kept and used to seed subsequent fuzzing rounds later on. This mode very quickly produces a small corpus of related but somewhat different crashes that can be effortlessly compared to pretty accurately estimate the degree of control you have over the faulting address.

To improve performance AFL uses a "fork server" where the fuzzed process goes through execve(), linking, and libc initialization only once, and is then cloned from a stopped process image by leveraging copy-on-write. The implementation is described in more detail in the article: [fuzzing binaries without execve](http://lcamtuf.blogspot.com/2014/10/fuzzing-binaries-without-execve.html).  It boils down to injecting a small piece of code into the fuzzed binary \- a feat that can be achieved via *`LD_PRELOAD`*, via *`PTRACE_POKETEXT`*, via compile-time instrumentation, or simply by rewriting the ELF binary ahead of the time. The purpose of the injected shim is to let *`execve()`* happen, get past the linker (ideally with *`LD_BIND_NOW=1`*, so that all the hard work is done beforehand), and then stop early on in the actual program, before it gets to processing any inputs generated by the fuzzer or doing anything else of interest. In fact, in the simplest variant, we can simply stop at *`main()`*. 

The parallelization mechanism is periodically examining the queues produced by other local or remote instances, and selectively pulling the testcases that when tried out locally produced behaviors not yet seen by the fuzzer.

The binary only instrumentation mode relies on [QEMU](https://www.qemu.org/) in user-emulation mode, QEMU uses basic blocks as translation units; the instrumentation is implemented on top of this and uses a model roughly analogous to the compile-time hooks. The start-up of binary translators such as QEMU, DynamoRIO, and PIN is fairly slow. To counter this, the QEMU mode leverages a fork server similar to that used for compiler-instrumented code, effectively spawning copies of an already-initialized process paused at `_start`.

The afl-analyse tool is a format analyzer simple extension of the minimization algorithm  
discussed earlier on, instead of attempting to remove no-op blocks, the tool performs a series of walking byte flips and then annotates runs of bytes in the input file.

Now that we carefully explored this documentation and we understand enough of the important mechanics a fuzzer need to find bugs in real world software, we also grasped the big picture of AFL architecture that we can summarize as follow:

1. The testcases are mutated with deterministics then random strategies.  
2. The coverage (edge) is collected per testcases run, along with a block hit count.  
3. Interesting testcases are kept in queue and prioritized for future fuzzing rounds.  
4. Dictionaries are deduced from bytes that satisfy a simple property.  
5. Crashes are classified as unique based on their tuples.  
6. Exploitation exploration mode uses mutation to produce interesting similar testcases.  
7. Parallelization is achieved through shared QUEUE with new testcases tuples.  
8. Binary-only fuzzing relies on custom QEMU userland instrumentation.

Of course, dozens of other fuzzers exist and each have their own specificity, however, AFL laid out the way for modern fuzzers and we can find similar architecture in a lot of them. A great way to learn more is to read the code of other state-of-the-art fuzzer such as [AFL++](https://github.com/AFLplusplus/AFLplusplus), [LibAFL](https://github.com/AFLplusplus/LibAFL), [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz), etc. 

## Designing Astra fuzzer architecture

The main goal of Astra is to provide a **simple** to use, **cross-platform** and **reliable** fuzzer. For these reasons, and because it is objectively the best language in the world, I decided to write this fuzzer in Rust. The fuzzer should be able to run on any desktop platform that supports Rust, provide a clear interface to the user and produce reliable deterministic outputs. 

State-of-the-art fuzzers have common features that Astra will implement too in order to achieve the desired results described above. From the analysis of AFL architecture we can retain a few core concepts: corpus minimization, instrumentation \- code-coverage, determinism, parallelization, mutation engine and fuzzing strategies, target observation and reporting. These constitute a solid foundational design we will now specify for Astra. Every programmer has their preferences, and I do not derogate from this rule, thus, some description might be arbitrary, feel free to adapt your own implementation to suit your tastes.

The fuzzer will be composed of multiple modules: collector, instrumentation engine, monitor, observer, mutator, reporter, runner, graphical interface (TUI). We will go through each one of these module creation in a sequential order, but let’s describe the role of each:

- **Collector**: Collects corpus and transforms it into a memory-efficient collection.  
- **Instrumentation engine**: Transform the program under test into an observable target that returns coverage per execution.  
- **Monitor**: Monitor the state of the program under test after each run.  
- **Observer**: Observe the coverage achieved by each input fed to the program under test.  
- **Mutator**: Mutate inputs to achieve better coverage.  
- **Reporter**: Report the statistics gathered by the observer and the monitor.  
- **Runner**: Run the program under test and feed them a mutated input.  
- **TUI**: Display and coordinate nicely all the fuzzer elements together.

To understand a bit better how to design Astra, we need to ask ourselves what is the workflow the fuzzer should follow to effectively stress a program under test.  

The first step is to instrument our program under test in order to observe code-coverage feedback during its fuzz execution, this is achieved by AFL with a custom compiler that injects custom assembly in the PUT (program under test).  The second step is to gather the corpus, the initial valid files we provide the fuzzer with, and transform them into mutable format, so that our third step, the mutation engine, can perform mutation on the testcases. 

The mutation engine will, based on deterministics or random approach, fuzz interesting (or not interesting) testcases. Comes in place the runner that will take a testcase candidate and run it against the PUT. After that the observer and the monitor will work in parallel to return precious information to the fuzzer: whether or not this testcases produced a crash OR interesting results (new coverage observed). 

The graphical interface, in our case the TUI, will display these results to the user. This loops until the user decides to end the fuzzing campaign and voila\! In summary, we can view the fuzzing process like this:  
```
      Target instrumentation                     
                 |                               
                 |                               
                 v                               
  Corpus collection and Transformation           
                 |                               
                 |                               
                 |                               
                 v                               
+-----------------------------------------------+
|             Fuzzing LOOP                      |
|                                               |
| Mutation engine   |                           |
|         ^         +--------->  Runner         |
|         |                        |            |
|         |                        |            |
|         |                        |            |
|         |                        v            |
|         |                                     |
|         +---------------------Observer        |
|                                   |           |
|                                   |           |
|                                   |           |
|              Reporter  <----------+           |
|                                               |
|                                               |
+-----------------------------------------------+
```

Astra needs to be compatible with Linux, Windows and Mac. This is a challenge in itself because it means we can’t rely on OS specific hacks to achieve some results. 