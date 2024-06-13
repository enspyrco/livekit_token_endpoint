FROM rust

COPY . .

EXPOSE 8080:8080

RUN cargo build --release

CMD [ "./target/release/livekit_token_endpoint" ]
