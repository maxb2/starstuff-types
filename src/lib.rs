/*!
# Starstuff Types

> 🚧 This is still a work in progress!!! 🚧

A simple star catalog parser and primitive types for star coordinates and astronomical times.

> Note: currently supports the [Yale Bright Stars Catalog](http://tdc-www.harvard.edu/catalogs/bsc5.html),  the [Hipparcos Catalog](https://heasarc.gsfc.nasa.gov/W3Browse/all/hipparcos.html), and the [Open Source Bright Star Catalog](https://github.com/johanley/star-catalog).
  Run the `get_data.sh` script to fetch the catalogs.
 */

#[macro_use]
extern crate assert_float_eq;

pub mod angle;
pub mod catalog;
pub mod constellation;
pub mod coord;
pub mod star;
pub mod time;
