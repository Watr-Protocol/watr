# the collator stage is normally built once, cached, and then ignored, but can
# be specified with the --target build flag. This adds some extra tooling to the
# image, which is required for a launcher script. The script simply adds two
# arguments to the list passed in:
#
#   --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/PEER_ID
#
# with the appropriate ip and ID for both Alice and Bob
FROM debian:bullseye-slim as collator
ENV NODE_MAJOR 16
RUN apt-get update && \
    apt-get install jq curl bash python3 build-essential -y && \
    curl -sSo /wait-for-it.sh https://raw.githubusercontent.com/vishnubob/wait-for-it/master/wait-for-it.sh && \
    chmod +x /wait-for-it.sh && \
    apt-get install -y ca-certificates curl gnupg && \
    mkdir -p /etc/apt/keyrings && \
    curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg && \
    echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_$NODE_MAJOR.x nodistro main" | tee /etc/apt/sources.list.d/nodesource.list && \
    apt-get update && \
    apt-get install nodejs -y && \
    npm install --global yarn && \
    yarn global add @polkadot/api-cli@0.10.0-beta.14
COPY target/production/watr-node /usr/bin
COPY docker/scripts/inject_bootnodes.sh /usr/bin
CMD ["/usr/bin/inject_bootnodes.sh"]
COPY docker/scripts/healthcheck.sh /usr/bin/
HEALTHCHECK --interval=300s --timeout=75s --start-period=30s --retries=3 \
    CMD ["/usr/bin/healthcheck.sh"]

# the runtime stage is normally built once, cached, and ignored, but can be
# specified with the --target build flag. This just preserves one of the builder's
# outputs, which can then be moved into a volume at runtime
FROM debian:bullseye-slim as runtime
COPY target/production/wbuild/watr-runtime/watr_runtime.wasm /var/opt/
CMD ["cp", "-v", "/var/opt/watr_runtime.wasm", "/runtime/"]

FROM debian:bullseye-slim
COPY target/production/watr-node /usr/bin

CMD ["/usr/bin/watr-node"]
