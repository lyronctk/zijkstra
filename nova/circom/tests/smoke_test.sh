# ==
# Boilerplate circuit compilation and vkey/zkey generation for development
# ==

# Compile circuit
circom ../traversal.circom --r1cs --wasm

# Generate the witness, primarily as a smoke test for the circuit
node traversal_js/generate_witness.js traversal_js/traversal.wasm smoke_test.json traversal.wtns

# Clean up
mv traversal_js/traversal.wasm ../out
mv traversal.r1cs ../out
rm -r traversal_js/ traversal.wtns
