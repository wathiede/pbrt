# Notes on C++ implementation.
* Filter::invRadius is only used in mitchell.cpp, but it is recomputed in
  FilmTile::FilmTile, why not pass the filter and use it directly?
* Precomputing `halfPixel` versus using `Vector2f(0.5f, 0.5f)` is
  inconsistent.
