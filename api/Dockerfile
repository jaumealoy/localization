FROM rust:latest AS BUILD

# copy source files to image /app folder
WORKDIR /app
COPY . .

# and build the source code
RUN cargo build --release

FROM debian:12-slim
RUN apt-get update
RUN apt-get install -y libssl-dev

# copy the executable from previous stage to /app
WORKDIR /app
COPY --from=BUILD /app/target/release/api .

EXPOSE 3000
ENTRYPOINT ["/app/api"]