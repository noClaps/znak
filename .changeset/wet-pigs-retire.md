---
"@noclaps/znak": patch
---

Fix crashing on opening math block. This was caused by improper checks for whether there was a closing tag for the math block, both inline and blocks.
