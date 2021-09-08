#!/usr/bin/env bash

# lotus-miner backup
source /mnt/md0/ipfs/conf/.lotusprofile
export miner_dataDir="/mnt/md0/ipfs/data/lotusminer"
export back_data="/mnt/md1/metadata"
export ip_list="10.0.1.254"
date=$(date '+%Y-%m-%d %H:%M:%S')
function backup_miner() {
  if [ ! -d ${back_data} ]; then
    mkdir -p ${back_data}
  fi
  $(cat /mnt/md0/ipfs/script/run_miner.sh | grep LOTUS_BACKUP_BASE_PATH)
  if [ $? == 0 ]; then
    #       sed -i '/LOTUS_MINER_PATH/a export LOTUS_BACKUP_BASE_PATH="'${back_data}'"' /mnt/md0/ipfs/script/.lotusprofile
    source /etc/profile && /usr/bin/lotus-miner backup ${back_data}/backup.cbor >${back_data}/bak.log 2>&1
    cp ${miner_dataDir}/config.toml ${miner_dataDir}/storage.json ${back_data}/
  else
    sed -i '/LOTUS_MINER_PATH/a export LOTUS_BACKUP_BASE_PATH="'${back_data}'"' /mnt/md0/ipfs/script/run_miner.sh
    sed -i '/LOTUS_MINER_PATH/a export LOTUS_BACKUP_BASE_PATH="'${back_data}'"' /mnt/md0/ipfs/script/.lotusprofile
    echo "please restart miner"
  fi
}

function lotus-miner_backup() {
  datetime=$(date +'%Y-%m-%d')
  IP=$(hostname -I | awk '{print $1}')
  id="f01128206"
  #id=$(/usr/local/sbin/lotus-miner info --hide-sectors-info | grep Miner | grep sectors | awk '{print $2}')
  cd /mnt/md0/ipfs/data/
  #       /usr/bin/rsync -avrP -e "ssh -p  6537" --exclude 'datastore' --delete lotus 172.168.1.250:/opt/backup/${datetime}/${id}-${IP}/
  /usr/bin/rsync -avrP -e "ssh -p  6537" --delete lotusminer ${ip_list}:/opt/backup/${datetime}/${id}-$IP/
  /usr/bin/rsync -avrP -e "ssh -p  6537" --delete backup ${ip_list}:/opt/backup/${datetime}/${id}-$IP/
  backup_miner
  rsync -alrvP -e "ssh -p  6537" /mnt/md1/metadata ${ip_list}:/opt/backup/${datetime}/${id}-$IP/

}

######################### Backup Status ############################
for ip in ${ip_list}; do
  ping -c 1 $ip &>/dev/null
  a=$?
  sleep 2
  ping -c 1 $ip &>/dev/null
  b=$?
  sleep 2
  ping -c 1 $ip &>/dev/null
  c=$?
  sleep 2
  id=$(echo ${ip} | sed 's/\./_/g')
  if [ $a -ne 0 -a $b -ne 0 -a $c -ne 0 ]; then
    echo -e "备份失败  \033[31m  ERROR \033[0m    \033[33m   ${date} \033[0m   lotus-miner  Backup \033[31m ERROR ！！！！！！ \033[0m "
  else
    lotus-miner_backup
    echo -e "完成备份  \033[34m  INFO \033[0m    \033[33m   ${date} \033[0m   lotus-miner  Backup \033[32m successfully! \033[0m "
  fi
done
