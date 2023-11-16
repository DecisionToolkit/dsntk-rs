FROM scratch

# copy the binary file of Decision Toolkit to container's root directory
COPY ./target/x86_64-unknown-linux-musl/release/dsntk /

# copy example DMN model to container's root directory
COPY ./dsntk/src/examples/e2/e2.dmn /

# start DSNTK as a service and display all deployed invocables
CMD ["/dsntk", "srv", "--verbose"]
