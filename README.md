# hdf5-sys [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides bindings to [HDF5][1].

Note that the high-level interface is not compiled since it currently
[excludes][2] the ability to build the lower-level interface thread safe.

## [Documentation][doc]

## Contributing

1. Fork the project.
2. Implement your idea.
3. Open a pull request.

[1]: http://www.hdfgroup.org/HDF5
[2]: https://github.com/copies/hdf5/blob/v1.8.16/configure.ac#L1391

[version-img]: https://img.shields.io/crates/v/hdf5-sys.svg
[version-url]: https://crates.io/crates/hdf5-sys
[status-img]: https://travis-ci.org/stainless-steel/hdf5-sys.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/hdf5-sys
[doc]: https://stainless-steel.github.io/hdf5-sys
