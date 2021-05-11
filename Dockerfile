FROM rust as builder

COPY ./dmarc-receiver ./dmarc-receiver
WORKDIR ./dmarc-receiver

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && \
    RUNLEVEL=1 DEBIAN_FRONTEND=noninteractive apt-get install -y ca-certificates tzdata opensmtpd openssl

COPY --from=builder /dmarc-receiver/target/release/dmarc-receiver /bin/dmarc-receiver

RUN chmod +x /bin/dmarc-receiver && \
    echo "dmarcreceiver" > /etc/mailname

EXPOSE 25 587
ENTRYPOINT ["/usr/sbin/smtpd"]
CMD ["-d"]
