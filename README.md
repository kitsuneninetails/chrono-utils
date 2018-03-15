# Chrono Utils

This crate provides utilities for Rust's `chrono` library, to add on certain
features and utilities which aren't provided by the standard crate.

## Utilities

### Monthly Calculation

The standard `chrono` library has no single function to transform a date 
based on a number of months in the future or past.  This trait defines
functions to return transformed `chrono::DateTime` objects based on a
month-defined transformation.

#### add_months

`DateTime<Tz>::add_months(<number of months to advace or regress>) -> DateTime<Tz>`

This function takes a positive or negative integer to advance a date by a certain 
number of months. A positive number will advance time while a negative number will 
regress time. Any number greater than 12 will move time by over a number of years.  
Also any value which advances or regresses time past a year boundary (for example 
specifying an advancement of three months from a date in November) will result in 
the year advancing or regressing appropriately (as in the above example, the year 
will be advanced and the month wil be set to February).
