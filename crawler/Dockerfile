FROM rust AS build
WORKDIR /app/build
COPY . .

# ENV CARGO_TARGET_DIR=/app/production 
RUN cargo build --release

FROM build AS production
#COPY --from=build /app/production /app
COPY --from=build /app/build/release /app
WORKDIR /app

ENTRYPOINT [ "crawler" ]
