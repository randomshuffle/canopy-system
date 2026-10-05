#!/usr/bin/env python3
"""Compatibility entry point; generate fresh estimates without editing this dataset."""

import runpy
import sys
from pathlib import Path


if __name__ == '__main__':
    root = Path(__file__).resolve().parents[2]
    sys.path.insert(0, str(root / 'scripts'))
    runpy.run_path(str(root / 'scripts/reproduce_estimates.py'), run_name='__main__')
