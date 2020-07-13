# We could download the watchdog directly, but since the artifacts 
# are named differently for different architectures, there is no 
# straight forward and portable way to get the binary other than 
# using the watchdog image as a source image. 
FROM openfaas/classic-watchdog:0.18.17 as watchdog

# Since it's easier to build the container for another architecture
# than to cross compile it locally, we are using this build container.
FROM alpine:latest as build
RUN apk update && apk add --no-cache rust cargo

COPY function /function
COPY main /main

RUN cd /main && cargo build --release 

# The actual run container remains quite small.
FROM alpine:latest

COPY --from=watchdog /fwatchdog /usr/local/bin/fwatchdog
COPY --from=build /main/target/release/main /usr/local/bin/main

RUN apk update \
    && apk add --no-cache libgcc curl ca-certificates \
    && chmod +x /usr/local/bin/fwatchdog /usr/local/bin/main

HEALTHCHECK --interval=5s CMD [ -e /tmp/.lock ] || exit 1

ENV fprocess="main"

ENTRYPOINT ["fwatchdog"]
