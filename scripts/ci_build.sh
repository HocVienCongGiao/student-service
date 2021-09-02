# Check student-app
cd student-app &&
cargo fmt --all -- --check &&
cargo clippy --all-targets -- -D clippy::all &&
cargo check --all
cargo build
cargo test
echo "student-app checked"
# Check student-lambda
cd ..
cd student-lambda &&
cargo fmt --all -- --check &&
cargo clippy --all-targets -- -D clippy::all &&
cargo check --all
cargo build
cargo test