## High-level architecture

Astra is built around a **controller + workers** model:

### 1) Controller (main process)
Responsible for:
- loading seeds and managing the corpus
- distributing work to workers
- collecting results (coverage updates, crashes, hangs)
- printing statistics / progress

### 2) Workers (parallel fuzzing loop)
Each worker repeatedly:
1. picks a seed from the corpus
2. mutates it
3. executes the target
4. reports:
   - new coverage (interesting inputs)
   - crashes
   - timeouts / hangs

### 3) Shared state
Workers coordinate through lightweight shared state such as:
- a global coverage map / edge bitmap
- counters (execs/sec, total execs, etc.)
- channels/queues for events (crash/hang/interesting inputs)
