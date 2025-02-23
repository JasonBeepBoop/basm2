# TODO

- [ ] Implement codegeneration
- [ ] Create warning type
- [ ] Create more help messages
- [ ] Copy that Levenshtein distance function and perform string similarlity when things are invalid
- [ ] Implement a @define directive as constants are file-scoped (and a @define call, maybe with $<name>)
- [ ] Implement nicer verbose printing in verbose mode
- [ ] Add CLI flags to disable errors/warnings
- [ ] Maybe: Write a standard library

# DONE

- [x] URGENT: FIX SO MANY CLONES AND PANICS!!!! PLEASE!!!!
- [x] URGENT: USE REFERENCE TYPES !!!! WHY DID I FORGET THIS ???
- [x] Implement constant expression evaluation w/hashmap
- [x] Implement .include directives
- [x] Create instruction validator for instruction struct
- [x] Implement macro verification for macro struct 
- [x] Implement expand function for macro when arguments are passed
- [x] Create macro TokenKind
- [x] Implement instruction verification INSIDE of the macro expander
~~Attempt early label symbol table resolution~~ Cannot be done.
