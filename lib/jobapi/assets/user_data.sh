#!/bin/bash

sudo yum install -y git gcc >> /log.txt 2>&1

aws s3 cp s3://trao-infra-resources/exec-app/exec-app /root/exec-app >> /log.txt 2>&1
chmod +x /root/exec-app >> /log.txt 2>&1
/root/exec-app >> /log.txt 2>&1