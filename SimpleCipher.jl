module SimpleCipher

export encrypt, decrypt, create_cipher

# Encrypts a message using the Caesar cipher with a given shift
function encrypt(message::String, shift::Int)
    return transform_text(message, shift)
end

# Decrypts a message using the Caesar cipher with a given shift
function decrypt(ciphertext::String, shift::Int)
    return transform_text(ciphertext, -shift)
end

# Transforms text by shifting characters
function transform_text(text::String, shift::Int)
    return String([shift_char(c, shift) for c in text])
end

# Helper function
function shift_char(c::Char, n::Int)
    if !isletter(c)
        return c
    end
    base = islower(c) ? 'a' : 'A'
    return Char(mod(Int(c) + n - Int(base), 26) + Int(base))
end

# Creates a cipher with a preset shift
function create_cipher(shift::Int)
    encrypt_fn(message) = encrypt(message, shift)
    decrypt_fn(ciphertext) = decrypt(ciphertext, shift)
    return (encrypt_fn, decrypt_fn)
end

end
