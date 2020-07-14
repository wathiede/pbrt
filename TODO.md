# Notes on C++ implementation.
* Filter::invRadius is only used in mitchell.cpp, but it is recomputed in
  FilmTile::FilmTile, why not pass the filter and use it directly?
* Precomputing `halfPixel` versus using `Vector2f(0.5f, 0.5f)` is
  inconsistent.
* Why is `Film::pixels` a `std::unique_ptr<Pixel[]>` but `FilmTile::pixels` is
  a `std::vector<FilmTilePixel>`, why not the same for both?
* Why isn't `LOOKUP_ONE` and `LOOKUP_PTR` used for `FindOneFloat` and
  `FindFloat`.

# Pull requests:
* https://github.com/mmp/pbrt-v3/pull/295
  * Why does `ParamListItem` have `isString`, it's not used.
  * Why do `parseParams` and `AddParam` take a `SpectrumType`, it doesn't appear
    to be used.
