
## Run 

```sh
python -m venv .venv
source .venv/bin/activate
pip install build setuptools
# build extension
python -m build
# install extension
pip install ./dist/cvarint-1.0.0-cp310-cp310-linux_x86_64.whl

```


```sh
python test.py 
Running speed test...
Executed 1,000,000 random tests

Python:   2766ns per case (2.767s total)
C:         419ns per case (0.420s total)
```
