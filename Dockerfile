
FROM rust:1.52-buster as builder

ENV APP_FOLDER=/opt/app
RUN mkdir "${APP_FOLDER}"

# In this case one does not need to set argument
# The env var RUST_LOG will be picked up by the env_logger in the applicatin and set correctly
ENV RUST_LOG="info"
ENV BIND="127.0.0.0:8080"
WORKDIR "${APP_FOLDER}"
COPY Cargo.toml Cargo.lock "${APP_FOLDER}/"
COPY src/ "${APP_FOLDER}/src"
RUN cargo build --release



# FROM gcr.io/distroless/cc

FROM gcr.io/distroless/static-debian10


# ENV APP_FOLDER=/opt/app

ENV APP_FOLDER=/opt/app
# RUN useradd app && mkdir -p "${APP_FOLDER}" && chown -R app:app "${APP_FOLDER}"
WORKDIR /opt


# USER app
# COPY --chown=app --from="builder" "${APP_FOLDER}/target/release/shotlog" .
COPY --from="builder" "${APP_FOLDER}/target/release/shotlog" .

EXPOSE 8080
CMD "/opt/shotlog" "--server" ${BIND}"--level" ${RUST_LOG}