rustup update
rustup component add llvm-tools-preview
rustup target add thumbv7em-none-eabihf
cargo build
cargo readobj --target thumbv7em-none-eabihf --bin engine_controller -- --file-header
cargo flash --chip STM32F303RETx --release

python3 ./can_sender/can_sender.py --channel /dev/tty.usbmodem1203 --bitrate 500000 --path ./can_sender/gears.txt
/dev/tty.usbmodem1203
