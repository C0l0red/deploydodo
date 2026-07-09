ssh-keygen -t rsa -f backend/key_rsa -N '' -C deploydodo-admin
mkdir -p ~/.ssh
chmod 700 ~/.ssh
touch ~/.ssh/authorized_keys
chmod 600 ~/.ssh/authorized_keys
cat backend/key_rsa.pub >> ~/.ssh/authorized_keys
