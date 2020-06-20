use pyo3::{ffi, prelude::*, types::PySequence, AsPyPointer};

fn main() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let a = py.eval("float('nan')", None, None).unwrap();
    // nan == nan must not be Equal, but this test fails.
    assert_ne!(a.compare(a).unwrap(), std::cmp::Ordering::Equal);
}
