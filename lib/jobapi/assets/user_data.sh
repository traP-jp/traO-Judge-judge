#!/bin/bash

sudo yum install -y git gcc >> /log.txt 2>&1

aws s3 cp s3://trao-infra-resources/exec-http-server/exec-http-server /root/exec-http-server >> /log.txt 2>&1
chmod +x /root/exec-http-server >> /log.txt 2>&1
/root/exec-http-server >> /log.txt 2>&1