
# release version
if [ -z "${SCROLL_ZKVM_VERSION}" ]; then
  echo "SCROLL_ZKVM_VERSION not set"
  exit 1
fi

wget https://circuit-release.s3.us-west-2.amazonaws.com/scroll-zkvm/releases/$SCROLL_ZKVM_VERSION/chunk/app.vmexe -O ./assets/chunk/app.vmexe
wget https://circuit-release.s3.us-west-2.amazonaws.com/scroll-zkvm/releases/$SCROLL_ZKVM_VERSION/chunk/openvm.toml -O ./assets/chunk/openvm.toml

wget https://circuit-release.s3.us-west-2.amazonaws.com/scroll-zkvm/releases/$SCROLL_ZKVM_VERSION/batch/app.vmexe -O ./assets/batch/app.vmexe
wget https://circuit-release.s3.us-west-2.amazonaws.com/scroll-zkvm/releases/$SCROLL_ZKVM_VERSION/batch/openvm.toml -O ./assets/batch/openvm.toml

wget https://circuit-release.s3.us-west-2.amazonaws.com/scroll-zkvm/releases/$SCROLL_ZKVM_VERSION/bundle/app.vmexe -O ./assets/bundle/app.vmexe
wget https://circuit-release.s3.us-west-2.amazonaws.com/scroll-zkvm/releases/$SCROLL_ZKVM_VERSION/bundle/openvm.toml -O ./assets/bundle/openvm.toml

wget "https://circuit-release.s3.us-west-2.amazonaws.com/scroll-zkvm/params/kzg_bn254_22.srs" -O ./params/kzg_bn254_22.srs
wget  "https://circuit-release.s3.us-west-2.amazonaws.com/scroll-zkvm/params/kzg_bn254_24.srs" -O ./params/kzg_bn254_24.srs

mkdir -p "$HOME/.openvm"
ln -s "./params" "$HOME/.openvm/params"

mkdir -p /usr/local/bin
wget https://github.com/ethereum/solidity/releases/download/v0.8.19/solc-static-linux -O /usr/local/bin/solc
chmod +x /usr/local/bin/solc
