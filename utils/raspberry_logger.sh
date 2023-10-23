#!/bin/bash

HOSTNAME="seff_jr"
IP="192.168.0.5"


# Commands for the raspberry pi
if [ "$1" == 1 ]; then
  COMMAND="python $HOME/BenchmanagemenRaspbPi/kasa_energy_consumption.py"
elif [ "$1" == 0 ]; then
  COMMAND="python $HOME/BenchmanagemenRaspbPi/temp_socket_testing_manager.py"
fi
# Call Raspberry PI with command
# Uses -i ~/.ssh/id_rsa for public key use, rather than password auth
ssh -i /home/seff/.ssh/id_rsa $HOSTNAME@$IP $COMMAND
