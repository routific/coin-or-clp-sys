# Bindings to the C interface of [CLP].

The bindings depends on a local installation of the C++ library in a directory
with a specific structure. The path to that directory is read during build time
from the environment variable `COIN_LIB_DIR`. Such an setup can be achieved by
following [these instructions](#installing-COIN-dependencies).

## Installing COIN dependencies

We recommend using [coinbrew]. Download the script and run:
```bash
./coinbrew fetch \
  --main-proj=Clp \
  --main-proj-version=stable/1.17 \
  --git

./coinbrew build \
  --main-proj=Clp \
  --build-dir=$PWD/build \
  --prefix=$PWD/build \
  --test
```

If everything goes well :sweat_smile:, you should get a message like:
```
Libraries have been installed in:
   <SOME_PATH>/build/lib

If you ever happen to want to link against installed libraries
```

Then you can build the library with `COIN_LIB_DIR=<SOME_PATH> cargo build`.

Something went wrong?
- check your version of [bash] to run the script
- check that you have the proper [build dependencies]

## Testing

For now there are only a single very simple integration test exercizing the
bindings: `COIN_LIB_DIR=<SOME_PATH> cargo test -- --nocapture`.

[CLP]: https://github.com/coin-or/Clp
[coinbrew]: https://github.com/coin-or/coinbrew
[build dependencies]: https://github.com/coin-or/COIN-OR-OptimizationSuite#building-from-source
[bash]: https://github.com/coin-or/coinbrew/pull/9
