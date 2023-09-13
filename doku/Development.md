### Allow docker to run images with a different target platform than the host
- `docker run --rm --privileged multiarch/qemu-user-static --reset -p yes`
- This links all platforms supported by qemu correctly for docker
- This might have to be run after every host system reboot

### Usage
All commands run in project root

1. Develop on your host architecture: `docker compose up develop`
    - This keeps the container open in the current terminal, after you are done working you can stop it with Ctrl + C
2. Build the binary for aarch64/arm64: 
   - `./buildAarch64.sh` - this increments the build number in Cargo + builds the binary and copies it into aarch64/ to allow git tracking + removes container after its finished
      - Why increment the build number in Cargo.toml?  
      - Because sometimes the combination of docker caching and cargo build doesnt use the most up-to-date Cargo.toml. <br>
      This way the Cargo.toml always changes and wont be cached
   - `docker compose up aarch64-build` - only builds binary into aarch64/target/
