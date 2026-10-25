#!/bin/bash
cd `dirname $0`/..
charon cargo --preset aeneas \
	--start-from datrie::alpha_map::AlphaMap::add_range \
	--start-from datrie::alpha_map::AlphaMap::serialize \
	--start-from datrie::alpha_map::AlphaMap::serialized_size \
	--start-from datrie::alpha_map::AlphaMap::char_to_trie \
	--start-from datrie::alpha_map::AlphaMap::trie_to_char \
	--start-from datrie::symbols::Symbols::_
