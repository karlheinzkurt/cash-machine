FROM alpine:latest as build
RUN apk update \
    && apk add --no-cache rust cargo

COPY function /function
COPY main /main

RUN cd /main && cargo build --release 

FROM alpine:latest

RUN apk update \
    && apk add --no-cache libgcc curl ca-certificates \
    && curl -sL https://github.com/openfaas/faas/releases/download/0.18.17/fwatchdog > /usr/bin/fwatchdog \
    && chmod +x /usr/bin/fwatchdog

WORKDIR /usr/src/openfaas

COPY --from=build /main/target/release/main main

HEALTHCHECK --interval=5s CMD [ -e /tmp/.lock ] || exit 1

ENV fprocess="main"

ENTRYPOINT ["fwatchdog"]
