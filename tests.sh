#!/usr/bin/env bash
for i in $(ls -l c2server*.log | awk '{print $NF}'); do
  sum=`echo $i |grep -a C2零知识证明任务完成 $i | grep 2021-09-03 | wc -l`
 echo "${i}  ---->>>>> ${sum}"
done

