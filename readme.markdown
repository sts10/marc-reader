# MARC Reader

First-semester MLIS student trying to learn the structure of [a MARC record](https://loc.gov/marc/) with Rust. 

## Resources
* [https://loc.gov/marc/umb/](https://loc.gov/marc/umb/)
* [https://github.com/hectorcorrea/marcli/](https://github.com/hectorcorrea/marcli/)
* [https://gitlab.com/pymarc/pymarc](https://github.com/hectorcorrea/marcli/)

## Test data from:
* [https://github.com/hectorcorrea/marcli/tree/main/data](https://github.com/hectorcorrea/marcli/tree/main/data)

## To do
- [ ] Parse leader as its own Struct (consult [marc-record](https://github.com/demarque/marc-record/blob/main/src/parser.rs) and [marcli](https://github.com/hectorcorrea/marcli/blob/main/pkg/marc/leader.go))
- [ ] Implement basic search functionality (just of all "non-control fields"?)
- [ ] Write tests
- [ ] Use Clap to make it a proper command line tool
- [ ] Write benchmarks (using Criterion, probably)
- [ ] Add function to output search results, maybe to a CSV?

### Optimizations to try 

- [Aho-Corasick](https://docs.rs/aho-corasick/latest/aho_corasick/)
- [memchr](https://docs.rs/memchr/latest/memchr/)
- Parallelism (Rayon)
