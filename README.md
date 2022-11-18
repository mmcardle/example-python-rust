# Example Project Rust + Python

A project using [maturin](https://github.com/PyO3/maturin) to use Rust from Python

## Steps

* Install Rust - https://www.rust-lang.org/tools/install
* Create a virtualenv

```
python3 -m venv .venv
source .venv/bin/activate
```

* Install requirements

```
pip install maturin
python -m pip install --upgrade pip
```

* Build and run

```
maturin build -r
pip install --force target/wheels/mixed_project-0.1.0-cp38-cp38-manylinux_2_28_x86_64.whl
python -c 'import mixed_project;mixed_project.run("xxxxxxxxx")'
```