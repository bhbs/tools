//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_new_symbol;
mod no_unreachable;
mod use_optional_chain;
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: no_new_symbol :: NoNewSymbol , self :: no_unreachable :: NoUnreachable , self :: use_optional_chain :: UseOptionalChain ,] } }
