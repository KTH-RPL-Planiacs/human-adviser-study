FROM rust:1.66 as builder
WORKDIR /src
RUN git clone https://github.com/KTH-RPL-Planiacs/human-adviser-study.git
WORKDIR /src/human-adviser-study/study_backend
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libssl1.1 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /src/human-adviser-study/study_backend/target/release/study_backend /usr/local/bin/study_backend
CMD ["study_backend"]