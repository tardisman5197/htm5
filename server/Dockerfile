# Create a builder docker image
FROM rust:latest as builder

## Create an empty project
RUN USER=root cargo new --bin server
WORKDIR /server
# Copy over the depenanicy list
COPY ./Cargo.lock ./Cargo.toml ./
# Get the dependancies
RUN cargo build --release
RUN rm src/*.rs
RUN rm ./target/release/deps/server*

#  Copy the project over
COPY ./src ./src
# Build the project
RUN cargo build --release

# Create release image
FROM rust:latest

# Move over the binary
COPY --from=builder /server/target/release/server .

CMD ["./server"]