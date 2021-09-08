#!/bin/bash
set -e

export RUST_BACKTRACE=full
export RUST_LOG=trace

export IPFS_GATEWAY="https://proof-parameters.s3.cn-south-1.jdcloud-oss.com/ipfs/"
export FIL_PROOFS_PARAMETER_CACHE="/mnt/md0/filecoin-proof-parameters"
export FIL_PROOFS_PARENT_CACHE="/mnt/md0/filecoin-parents/"

export FIL_PROOFS_MAXIMIZE_CACHING=1
export FIL_PROOFS_USE_GPU_COLUMN_BUILDER=1
export FIL_PROOFS_USE_GPU_TREE_BUILDER=1

export GOLOG_LOG_FMT=json

export MINERID="f01128206"

export CUDA_VISIBLE_DEVICES=0
export TMPDIR="/tmp/gpu0"
export Ip_proxy="10.0.3.5"

IP=$(hostname -I | awk '{print $1}')

sudo nvidia-smi
if [ $? -ne 0 ]; then
  echo "ERROR: no GPU detected $IP"
  exit
fi

ns=$(nvidia-smi -L | awk '{print $5}' | head -1)
if [ $ns -eq 3090 ] || [ $ns -eq 3080 ]; then
  export BELLMAN_CUSTOM_GPU="GeForce RTX 3090:10496, GeForce RTX 3080:8704"
fi

if [ -d /tmp/gpu0 ]; then
  echo "/tmp/gpu0 目录已存在"
else
  mkdir /tmp/gpu0 -p
fi

mkdir /tmp/c2path -p
#nohup taskset -c 0-31 /mnt/md0/ipfs/bin/1.10-20210702/c2server -local ${IP}:21800 -remote 10.0.1.47:1680  -workpath /tmp/c2path  &
#sudo prlimit --nofile=1048576 --nproc=unlimited --rtprio=99 --nice=-19 --pid $!
n=$(lscpu | grep 'Model name' | awk '{print $5}')
if [ X$n == "X7F32" ] || [  X$n == "7302" ];then
  nohup taskset -c 0-15 /mnt/md0/ipfs/bin/1.10-20210702/c2server -local ${IP}:21800 -remote ${Ip_proxy}:1680 -workpath /tmp/c2path &
  sudo prlimit --nofile=1048576 --nproc=unlimited --rtprio=99 --nice=-19 --pid $!
elif [ X$n == "X7402" ]; then
  nohup taskset -c 0-47 /mnt/md0/ipfs/bin/1.10-20210702/c2server -local ${IP}:21800 -remote ${Ip_proxy}:1680 -workpath /tmp/c2path &
  sudo prlimit --nofile=1048576 --nproc=unlimited --rtprio=99 --nice=-19 --pid $!
elif [ X$n == "X7532" ]; then
  nohup taskset -c 0-32 /mnt/md0/ipfs/bin/1.10-20210702/c2server -local ${IP}:21800 -remote ${Ip_proxy}:1680 -workpath /tmp/c2path &
  sudo prlimit --nofile=1048576 --nproc=unlimited --rtprio=99 --nice=-19 --pid $!
fi

wait


for i in  {1..10000}
do
  lotus wallet new
done
