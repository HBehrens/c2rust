FROM gitpod/workspace-full

# More information: https://www.gitpod.io/docs/config-docker/


USER root

# Also see scripts/provision_deb.sh

RUN apt-get update && \
    apt-get install -y \
       libncurses5-dev \
       luarocks \
       build-essential \
       llvm-6.0 \
       clang-6.0 \
       libclang-6.0-dev \
       libssl-dev \
       pkg-config

USER gitpod

COPY --chown=gitpod:gitpod ${DOCKER_CONTEXT_SOURCE}/scripts/requirements.txt /tmp/c2rust_requirements.txt
RUN pip3 install -r $/tmp/c2rust_requirements.txt --disable-pip-version-check

RUN luarocks path > /etc/profile.d/luarocks-path.sh
RUN luarocks install penlight
