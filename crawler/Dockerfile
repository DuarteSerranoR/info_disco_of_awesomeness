FROM rust AS build
WORKDIR /app/build
COPY ./crawler ./crawler
COPY ./logger ./logger
COPY ./database_connector ./database_connector

WORKDIR /app/build/crawler

RUN cargo build --release

FROM build AS production
COPY --from=build /app/build/crawler/target/release/ /app/

ENTRYPOINT [ "/app/crawler" ]
