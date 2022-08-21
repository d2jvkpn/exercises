### Install virtual machines
---

#### references:
- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/7/html/virtualization_deployment_and_administration_guide/sect-statlists

#### 1.1 Install kvm
```bash
grep -Eoc '(vmx|svm)' /proc/cpuinfo
apt install cpu-checker
kvm-ok

apt install qemu-kvm libvirt-daemon-system libvirt-clients bridge-utils virtinst virt-manage

systemctl is-active libvirtd
usermod -aG libvirt $USER
usermod -aG kvm $USER
brctl show
```

#### 1.2. Create virtual machine
```bash
vm=ubuntu

virt-install --name=$vm --os-variant=generic --vcpus=2 --memory=2048 \
  --disk path=/var/lib/libvirt/images/$vm.qcow2,size=20              \
  --cdrom=/home/hello/Work/kvm/ubuntu-22.04.1-live-server-amd64.iso
```
#### 1.3 install UI
...username: hello

#### 1.3 ssh vm
```bash
vm=ubuntu

virsh start $vm
virsh net-list
virsh net-dhcp-leases default
# rm /var/lib/libvirt/dnsmasq/virbr0.*

addr=$(virsh domifaddr $vm | awk '$1!=""{split($NF, a, "/"); addr=a[1]} END{print addr}')
ssh hello@$addr

apt update && apt -y upgrade

apt install -y software-properties-common apt-transport-https ca-certificates \
  vim iftop iotop net-tools gnupg-agent gnupg2 tree jq at autossh pigz        \
  iputils-ping curl file

timedatectl set-timezone Asia/Shanghai

apt clean remove

sudo su
echo -e "\n\n\nPermitRootLogin yes" >> /etc/ssh/sshd_config
systemctl restart ssh

# set root password
passwd
exit
```

#### 1.5 fix ip
```bash
vm=ubuntu

virsh start $vm
addr=$(virsh domifaddr $vm | awk '$1!=""{split($NF, a, "/"); addr=a[1]} END{print addr}')
# mac=$(virsh dumpxml $vm | xq -r '.domain.devices.interface.mac."@address"')
mac=$(virsh domiflist $vm | awk 'NR==3{print $NF}')

record=$(printf "<host mac='%s' name='%s' ip='%s'/>" $mac $vm $addr)
virsh net-update default add ip-dhcp-host "$record" --live --config

virsh net-dumpxml default
# virsh net-edit default
# virsh net-destroy default
# virsh net-start default

cat >> ~/.ssh/config << EOF

Host $vm
    HostName      $addr
    User          root
    Port          22
    LogLevel      INFO
    Compression   yes
    # IdentityFile  ~/.ssh/id_rsa
EOF

virsh shutdow $vm
virsh start $vm

ssh root@$vm
```

##### 1.6 clone ubuntu
```bash
src=ubuntu
vm=vm1

virsh shutdown $src
virt-clone --original $src --name $vm --file /var/lib/libvirt/images/$vm.qcow2
# virt-clone --original $src --name $vm --auto-clone

# 2.6 extend vm disk size

#
virsh list --all
virsh start $vm
virsh domifaddr $vm # ip is as same as vm1

addr=$(virsh domifaddr $vm | awk '$1!=""{split($NF, a, "/"); addr=a[1]} END{print addr}')
ssh root@$addr

vm=vm1
hostnamectl set-hostname $vm
sed -i "2s/^127.0.1.1 .*$/127.0.1.1 $vm/" /etc/hosts

rm /etc/machine-id
dbus-uuidgen --ensure=/etc/machine-id

shutdown now

vm=vm2
#!! "2.4 fix ip"

ssh root@$vm
```

#### 1.7 extend vm disk size
```bash
vm=vm1

# virsh domblklist $vm
# qemu-img info /var/lib/libvirt/images/$vm.qcow2
# qemu-img resize /var/lib/libvirt/images/$vm.qcow2 +10G
# qemu-img info /var/lib/libvirt/images/$vm.qcow2

# Please note that qemu-img can’t resize an image which has snapshots. You will need to first remove
# all VM snapshots. See this example:
# virsh snapshot-list $vm
# virsh snapshot-delete --domain $vm --snapshotname $??

ssh root@$vm
lsblk
pvs
vgdisplay
fdisk -l | grep -- "--lv"
lvextend -L +10G /dev/mapper/ubuntu--vg-ubuntu--lv
resize2fs /dev/mapper/ubuntu--vg-ubuntu--lv
```

##### 1.8 other vrish commands
```bash
virsh edit $VHOST

virsh setvcpus ubuntu 2 --config

virsh net-list --all
virsh net-dhcp-leases default

man virsh

virsh start foo
virsh reboot foo
virsh shutdown foo
virsh suspend foo
virsh resume foo

virsh console foo

virsh dumpxml foo
virsh define foo
virsh destroy foo_new
virsh undefine foo_new
```
