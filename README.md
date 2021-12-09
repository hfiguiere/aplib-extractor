aplib extractor
===============

Extract the data from Apple Aperture™ libraries in order to facilitate
importing it into another application.

Supported version are 3.x up to 3.6 (the final version).

This is written in Rust.

Requires:
- Rust and cargo (edition 2018)
- exempi (pulled by the exempi2 crate)

Building
--------

If you use this in your project, just add to your Cargo.toml:
```toml
aplib-extractor = "0.1.0"
```
To build the dumper tool:

$ cargo build --feature=binaries

Other
-----

If you are interested in extracting Lightroom catalogs, there is the
`lrcat` crate.

License
-------

  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.

See the LICENSE file in this repository.

Maintainer:
Hubert Figuière <hub@figuiere.net>