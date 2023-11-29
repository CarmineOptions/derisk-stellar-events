FROM rust:buster as Builder
WORKDIR /build
COPY . .
RUN cargo build --release

FROM carminefinance/rust-stellar-core:1.0.0 as StellarCore

FROM ubuntu:focal
# Avoid warnings by switching to noninteractive
ENV DEBIAN_FRONTEND=noninteractive

# setup apt / certificates
RUN apt-get update \
  && apt-get -y install --no-install-recommends apt-utils dialog ca-certificates 2>&1

# use apt mirror instead of default archives if specified
# to use, specify the build arg or as an env var on the host machine
# e.g.:
#   mirror://mirrors.ubuntu.com/mirrors.txt
#   mirror://mirrors.ubuntu.com/<country-code>.txt
#   http://<country-code>.archive.ubuntu.com/ubuntu/
#   http://<aws-region>.ec2.archive.ubuntu.com/ubuntu
ARG APT_MIRROR=
RUN if [ ! -z "${APT_MIRROR}" ]; then \
  sed -i \
  -e "s|http://archive.ubuntu.com/ubuntu/|${APT_MIRROR}|" \
  -e "s|http://security.ubuntu.com/ubuntu/|${APT_MIRROR}|" \
  /etc/apt/sources.list \
  ; fi \
  ; grep "^[^#;]" /etc/apt/sources.list

# Install common compilation tools
RUN apt-get -y install git build-essential pkg-config autoconf automake libtool bison flex sed perl libpq-dev parallel libunwind-dev curl

# Update compiler tools
RUN apt-get -y install libstdc++-10-dev

WORKDIR /app
COPY --from=Builder /build/target/release/historic_events .
COPY --from=Builder /build/target/release/stream_events .
COPY --from=StellarCore /usr/local/bin/stellar-core /usr/local/bin/
