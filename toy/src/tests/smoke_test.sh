# ==
# Boilerplate circuit compilation and vkey/zkey generation for development
# ==

# Compile circuit
circom ../toy.circom --r1cs --wasm --prime vesta 

# Generate the witness, primarily as a smoke test for the circuit
node toy_js/generate_witness.js toy_js/toy.wasm smoke_test.json toy.wtns

# Clean up
mv toy_js/toy.wasm ../
mv toy.r1cs ../
rm -r toy_js/ toy.wtns
