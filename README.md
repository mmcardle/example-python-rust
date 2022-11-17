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
```

* Build and run

```
maturin develop
python -c 'import mixed_project;mixed_project.run("xxxxxxxxx")'
```