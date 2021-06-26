FROM debian:buster
RUN apt-get update
RUN apt-get install -y cmake
RUN apt-get install -y gcc-arm-none-eabi
RUN apt-get install -y libnewlib-arm-none-eabi
RUN apt-get install -y build-essential
RUN apt-get install -y python3
ENV PICO_SDK_PATH /src/pico-sdk/
ADD ./pico-sdk ${PICO_SDK_PATH}
WORKDIR /src/picorgb
# ADD ./src/CMakeLists.txt /src/
# ADD ./src/*.cmake /src/
ADD ./src/picorgb /src/picorgb
RUN cmake .
RUN make