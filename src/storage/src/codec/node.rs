//！NodeKey ::= <node_id>
//!
//! NodeValue ::= <header> <label_data> <property_data> <relationship_data>
//!
//! <header> ::= <labels_size> <property_offset> <relationship_offset>
//!
//! <labels_size> := u16 // number of labels
//!
//! <property_offset> ::= u32
//!
//! <relationship_offset> ::= u32
//!
//! <label_data> ::= <label_id>{<labels_size>}
//!
//! <property_data> ::=
//!
