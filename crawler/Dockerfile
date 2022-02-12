FROM rust AS build
WORKDIR /app/build
COPY . .

ENV CARGO_TARGET_DIR=/app/production 
RUN cargo build 

FROM build AS production
COPY --from=build /app/production /app
WORKDIR /app

ENTRYPOINT [ "crawler" ]
