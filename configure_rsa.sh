#! /bin/sh

ssh-keygen -t rsa -f key_rsa -N '' -C deploydodo-admin
mkdir -p ~/.ssh
chmod 700 ~/.ssh
touch ~/.ssh/authorized_keys
chmod 600 ~/.ssh/authorized_keys

grep -v "deploydodo-admin" ~/.ssh/authorized_keys > .tmp_keys
cat .tmp_keys > ~/.ssh/authorized_keys
rm -f .tmp_keys

cat key_rsa.pub >> ~/.ssh/authorized_keys
