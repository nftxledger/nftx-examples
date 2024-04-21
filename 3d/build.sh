echo "cleaning up"
rm -rf ./pkg
rm ./pkg.wasm
rm ./Qm*

echo "build script"
wasm-pack build --target no-modules --out-dir ./pkg --out-name pkg.wasm --release
wasm-custom-section ./pkg/pkg.wasm add importobject <./pkg/pkg.js

cid=$(ipfs-cid ./pkg/pkg.wasm.out)
mv ./pkg/pkg.wasm.out ./pkg.wasm
cp ./pkg.wasm ./$cid
