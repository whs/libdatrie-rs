meta:
  id: datrie
  file-extension: tri
  endian: be
seq:
  - id: alpha_map
    type: alpha_map
  - id: darray
    type: darray
  - id: tail
    type: tail
types:
  alpha_map:
    seq:
      - id: magic
        contents: [0xd9, 0xfc, 0xd9, 0xfc]
      - id: length
        type: s4
      - id: ranges
        type: range
        repeat: expr
        repeat-expr: length
  range:
    seq:
      - id: start
        type: s4
      - id: end
        type: s4
  darray:
    seq:
      - id: header_base
        contents: [0xda, 0xfc, 0xda, 0xfc]
      - id: header_check
        type: s4
      - id: cells
        type: cell
        repeat: expr
        repeat-expr: header_check - 1
  cell:
    seq:
      - id: base
        type: s4
      - id: check
        type: s4
  tail:
    seq:
      - id: magic
        contents: [0xdf, 0xfc, 0xdf, 0xfc]
      - id: first_free
        type: s4
      - id: length
        type: s4
      - id: blocks
        type: tailblock
        repeat: expr
        repeat-expr: length
  tailblock:
    seq:
      - id: next_free
        type: s4
      - id: data
        type: s4
      - id: suffix_len
        type: s2
      - id: suffix
        type: u1
        repeat: expr
        repeat-expr: suffix_len
