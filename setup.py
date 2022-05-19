#!/usr/bin/env python

from setuptools_rust import Binding, RustExtension
from setuptools import setup
from setuptools import dist
dist.Distribution().fetch_build_eggs(['setuptools_rust'])

setup(
    name="flitton-oasis-risk-modelling",
    version="0.1",
    rust_extensions=[RustExtension(
        ".catastrophe_risk_modelling.catastrophe_risk_modelling",
        path="Cargo.toml", binding=Binding.PyO3)],
    packages=["catastrophe_risk_modelling"],
    include_package_data=True,
    package_data={'': ['*.csv']},
    zip_safe=False,
)
