/*
1. **Set each byte to all values** – Yes, done byte by byte sequentially; `AAA` → `AAB` → `AAC` … per byte, one byte at a time.
2. **Flip each bit in each byte** – Yes, incrementally; each bit of each byte is flipped separately.
3. **Increment/decrement** – Yes, sequentially; each byte is incremented/decremented one at a time.
4. **Replace with ASCII tokens** – Typically sequential over a predefined token set, not random.
5. **Insert a fixed byte** – Yes, inserted after every byte in the input, usually one run per insertion position.
6. **Duplicate each byte** – Sequential; copy inserted right after the original byte, one mutation per position.
7. **Swap adjacent byte pairs** – One mutation per pair; each pair swapped in separate runs.
8. **Rotate bytes** – One mutation per rotation; each left/right rotation applied in one run.
*/