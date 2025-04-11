#!/bin/bash

sudo yum install -y git gcc >> /log.txt 2>&1

aws s3 cp s3://trao-infra-resources/exec-app/exec-app /root/exec-app >> /log.txt 2>&1
chmod +x /root/exec-app >> /log.txt 2>&1

docker pull public.ecr.aws/z7h0m8a7/trao-exec-container:pajri6z5qyv5rpgnbcn3srqs9r3dm7c6 >> /log.txt 2>&1

DOCKER_IMAGE_NAME=trao-exec-container:pajri6z5qyv5rpgnbcn3srqs9r3dm7c6 /root/exec-app >> /log.txt 2>&1