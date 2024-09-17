FROM scratch

# copy compiled binary to container's root directory
COPY ./target/x86_64-unknown-linux-musl/release/dsntk /

# save built-in examples
RUN ["/dsntk", "exs", "/examples"]

# start the service listing all deployed invocables
CMD ["/dsntk", "srv", "-v"]
