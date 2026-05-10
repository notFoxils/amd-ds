# Pre 1.0.0 Task-List
## Features
- [x] Driver Download:
    - [x] Link Scraper
    - [x] Command
- [x] Config:
    - [x] Defaults
    - [x] Merging (Defaults to User-Provided)
    - [x] Specificity
          `DriverDownloadLink` & `DriverVersion` scrapers should have their own respective configs.
- [x] Response-Streaming Speedup With Compression: Decompression can be handled by `Reqwest` through feature-flags
    - General Info:
        - https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Accept-Encoding
        - https://en.wikipedia.org/wiki/HTTP_compression
    - Decompression Libraries:
        - [flate2](https://docs.rs/flate2/latest/flate2/)
            - gzip
            - deflate
            - zlib
        - [brotli](https://docs.rs/brotli/8.0.2/brotli/)
- [x] Hide Dependencies From API:
    - [x] `scraper`
    - [x] `reqwest`
## Improvements
- [ ] Documentation
    - [ ] Library
    - [ ] CLI Example Config
    - [x] CLI Help Messages
- [x] Error Messages
- [ ] `Reqwest` Configuration?
    - [ ] Proxies
    - [ ] Timeouts
    - [x] Headers
- [x] `DriverVersion` Invariants
    - [x] Remove trailing zeroes at initialization
          - EX:
          0.0.0.0.1 -> 1
          0.2.0.1 -> 2.0.1
    - [x] Adjusted `std::cmp` Implementations
## Tests
- [ ] `DriverVersion` `std::cmp` Implementations
- [ ] Config Parsing and Default-Propagation
- [ ] Think of More Tests
## Investigations
- [ ] `Reqwest`:
    - [x] Client Configuration
    - [x] Redirection Handling
    - [ ] Caching
- [ ] Crate Meta-Topics:
    - [ ] MSRV - IDK how this works really
    - [ ] Release Policy
    - [ ] Everything In [Here](https://rustprojectprimer.com/title.html)
    - [x] API & ABI Exposure

# Release Policy By Changes ***Draft***
## Config Keys
- Semantics Changed - major
- Removal - major
- Addition That is Optional - minor
- Addition With a Default - minor
- Defaults Changed to Have Same Function - patch
    - EX: Selector change due to website layout change
## Logical
- Scraper Change - *follow Sem-Ver guidelines based on change*
- New scraper - minor
