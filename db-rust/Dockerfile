FROM ubuntu:latest

RUN apt-get update 

RUN apt -y install rust-all

RUN mkdir -p /opt/MIYOJIRO

WORKDIR /opt/MIYOJIRO

COPY . .

CMD ["/bin/bash"]
