[![Build Status](https://travis-ci.org/danylaporte/version_tag.svg?branch=master)](https://travis-ci.org/danylaporte/version_tag)

A VersionTag struct for Rust to detect changes.

## Documentation
[API Documentation](https://danylaporte.github.io/version_tag/version_tag)

## Example
Suppose you want to do a heavy computation based on something else.

You might want to cache the heavy computation result and perform it again
only when a dependency changes.

```rust
use version_tag::{combine, VersionTag};

/// A dependency
struct Dep {
    v: u32,
    tag: VersionTag,
}

struct MyStruct {
    v: u32,
    tag: VersionTag,
}

impl MyStruct {
    fn update_if_necessary(&mut self, x: &Dep, y: &Dep) {

        // compute the actual tag
        let actual = combine(&[x.tag, y.tag]);

        // if the tag has changed, we need to recalculate
        if actual != self.tag {

            // perform the heavy computation
            self.v = x.v + y.v;

            // update the tag
            self.tag = actual;
        }
    }
}
```

## License

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
[http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0) or the MIT license
[http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT), at your
option. This file may not be copied, modified, or distributed
except according to those terms.