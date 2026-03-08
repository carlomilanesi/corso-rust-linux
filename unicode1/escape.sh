# Bell (7, x07)
echo -e "\a"
echo -e "\x07"

# Backspace (8, x08)
echo -e "abc\bdefgh\b\bi"
echo -e "abc\x08defgh\x08\x08i"

# Cut
echo -e "abc\cdef" ; echo "gh"

# Escape ANSI (27, xb1, 033)
echo -e "Normale \x1b[31m Rosso \x1b[0m Ancora normale"
echo -e "Normale \e[31m Rosso \e[0m Ancora normale"

# Form feed (12, x0c)
echo -e "a\fb"
echo -e "a\x0cb"

# A capo (Nuova riga, New Line, 10, x0a)
echo -e "a\nb"
echo -e "a\x0ab"

# Ritorno carrello (Carriage Return, 13, x0d)
echo -e "a\rb"
echo -e "a\x0db"

# Tab orizzontale (9, x09)
echo -e "a\tb"
echo -e "a\x09b"
echo -e "1234567\tb"
echo -e "12345678\tb"

# Tab verticale (11, x0b)
echo -e "a\vb"
echo -e "a\x0bb"

# Unicode
echo -e "\u20ac\ufffd" # euro sign, replacement character
echo -e "[\x41][\u0041]" # [A][A]
