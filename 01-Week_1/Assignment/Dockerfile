FROM rust:slim AS builder

# Install tmate
RUN apt-get update && apt-get install -y --no-install-recommends \
    tmate \
    && rm -rf /var/lib/apt/lists/*

# Create user
RUN groupadd -g 1001 devgroup && \
    useradd -u 1001 -g devgroup -ms /bin/bash developer

WORKDIR /home/developer/app
COPY --chown=developer:developer . .

# Build as root
RUN cargo build --release

# Switch to user AFTER build is done
USER developer

# Launch game directly for standard evaluation
CMD ["./target/release/guessing-game"]