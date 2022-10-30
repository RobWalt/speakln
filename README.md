# `speakln!` notes

I mainly created `speakln!` for teaching purposes, so it comes with some restrictions:

- only sync programs can use `speakln!`
- currently you need to include the prelude for it to work: `use speakln::prelude::*;`
- you can use `speakln!` only on things that implement `Debug`
- `speakln!` is only supported on linux at the moment, support for other platforms is planned
