# Differences from C++ version
 * Output paramters rewritten as multiple return values.
 * Functions that use bool return type with out parameter are rewritten to use
   Option<>.
 * Scene parsing is two-phase.  First phase parses the file into a series of
   enums, see parser::{WorldBlock,OptionsBlock} into a parser::Scene object.
   The second phase then walks the Scene object calling api::Pbrt methods as
   appropriate.
