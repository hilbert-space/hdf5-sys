# hdf5-sys [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides bindings to [HDF5][1]. Note that the high-level interface
is not compiled since it currently [excludes][2] the possibility to make the
lower-level interface thread safe.

## [Documentation][doc]

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[1]: http://www.hdfgroup.org/HDF5
[2]: https://github.com/copies/hdf5/blob/v1.8.17/configure.ac#L1391

[doc]: https://stainless-steel.github.io/hdf5-sys
[status-img]: https://travis-ci.org/stainless-steel/hdf5-sys.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/hdf5-sys
[version-img]: https://img.shields.io/crates/v/hdf5-sys.svg
[version-url]: https://crates.io/crates/hdf5-sys
