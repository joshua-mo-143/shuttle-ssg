## Shuttle Static Site Generator

## Usage
Use the following command to clone this repo (make sure you have `cargo-shuttle` installed):
```bash
cargo shuttle init --from joshua-mo-143/shuttle-ssg
```
Follow the prompt, then cd into the folder and use `cargo shuttle deploy` (add `--ad` or `--allow-dirty` if on a dirty or uncommitted Git branch) to deploy to Shuttle!

Want to run this locally? Simply use `cargo shuttle run`.

## Troubleshooting
Make sure the Shuttle dependencies are at latest and in line with your cargo-shuttle version otherwise the web service may error out.

If there are any other issues, feel free to open an issue!
