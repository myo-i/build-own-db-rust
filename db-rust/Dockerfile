FROM ubuntu:latest

RUN apt-get update 

RUN apt -y install rust-all

RUN mkdir -p /opt/rust-owndb

WORKDIR /opt/rust-owndb

COPY . .

CMD ["/bin/bash"]
