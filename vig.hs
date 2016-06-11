module vig where


encrypt k ms = map \m -> (m + k) mod 26
