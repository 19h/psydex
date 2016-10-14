FROM rustarm

USER root

RUN apt-get install -fy libssl-dev

RUN cd /tmp ; curl --remote-name https://www.openssl.org/source/openssl-1.1.0b.tar.gz ; tar xfvz openssl-1.1.0b.tar.gz

RUN apt-get install -fy make

ENV CC /home/cross/pi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin/arm-linux-gnueabihf-gcc
ENV CXX /home/cross/pi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin/arm-linux-gnueabihf-g++

ENV ARMTOOLCHAINDIR /home/cross/pi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/

ENV TARGETMACH arm-none-gnueabihf
ENV BUILDMACH i686-pc-linux-gnu
ENV CROSS arm-linux-gnueabihf
ENV PATH $ARMTOOLCHAINDIR/bin:$PATH
ENV CC ${CROSS}-gcc
ENV LD ${CROSS}-ld
ENV AS ${CROSS}-as
ENV AR ${CROSS}-ar

RUN find /tmp/openssl-1.1.0b -type f | xargs sed -i 's/\-m64/\-Wall/g'

RUN cd /tmp/openssl-1.1.0b ; ./Configure -DOPENSSL_NO_HEARTBEATS shared os/compiler:arm-none-linux-gnueabihf ; make -j 4

USER cross

ENTRYPOINT bash
