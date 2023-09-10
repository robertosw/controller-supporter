### Allow docker to run images with a different target platform than the host
- `docker run --rm --privileged multiarch/qemu-user-static --reset -p yes`
- This links all platforms supported by qemu correctly for docker
