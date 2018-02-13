# What is this?
This is not an officially supported Google product.  This is a personal
project, it serves as a learning experience for both: programming in rust, and
writing a ray-tracer.  If either of these things are interesting to you, then
you may like this project.

This is a rust implementation of the physics based ray tracer documented in
'Physically Based Rendering, Third Edition' http://www.pbrt.org/

# Differences from C++ version
 * Output parameters rewritten as multiple return values.
 * Functions that use bool return type with out parameter are rewritten to use
   Option<>.
 * Scene parsing is two-phase.  First phase parses the file into a series of
   enums, see parser::{WorldBlock,OptionsBlock} into a parser::Scene object.
   The second phase then walks the Scene object calling api::Pbrt methods as
   appropriate.
