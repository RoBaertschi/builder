git commit -a -m "Auto commit from clippy.sh"

cargo fmt --all -- --check

cargo clippy --fix -- \
-W clippy::pedantic \
-W clippy::nursery \
-W clippy::unwrap_used \
-W clippy::expect_used || echo clippy not working