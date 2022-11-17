use cpython::{Python, PyResult, py_module_initializer, py_fn};


fn run(_py: Python, val: &str) -> PyResult<u64> {
    let mut total = 0u64;
    for (c1, c2) in val.chars().zip(val.chars().skip(1)) {
        if c1 == c2 {
            total += 1;
        }
    }
    
    println!("Rust says: Hello Python! The total is {}", total);
    Ok(total)
}


py_module_initializer!(mixed_project, |py, m| {
    m.add(py, "__doc__", "Module documentation string")?;
    m.add(py, "run", py_fn!(py, run(val: &str)))?;
    Ok(())
});